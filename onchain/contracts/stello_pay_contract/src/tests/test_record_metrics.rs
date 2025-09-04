use soroban_sdk::vec;
use soroban_sdk::{testutils::Address as _, Address, Env, log, symbol_short, Vec, IntoVal};
use soroban_sdk::token::{StellarAssetClient as TokenAdmin, TokenClient};
use soroban_sdk::testutils::{Ledger, LedgerInfo};
use crate::payroll::{PayrollContract, PayrollContractClient, PayrollError};
    use soroban_sdk::testutils::{ MockAuth, MockAuthInvoke};

fn create_test_contract() -> (Env, Address, PayrollContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(PayrollContract, ());
    let client = PayrollContractClient::new(&env, &contract_id);
    (env, contract_id, client)
}

fn setup_token(env: &Env) -> (Address, TokenAdmin) {
    let token_admin = Address::generate(env);
    let token_contract_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    (
        token_contract_id.address(),
        TokenAdmin::new(&env, &token_contract_id.address()),
    )
}


// #[test]
// fn test_get_metrics() {
//     let (env, contract_id, client) = create_test_contract();
//     let (token_address, token_admin) = setup_token(&env);
//     let employer = Address::generate(&env);
//     let employer2 = Address::generate(&env);
//     let employee1 = Address::generate(&env);
//     let employee2 = Address::generate(&env);
//     let employee3 = Address::generate(&env);
//     let amount = 1000i128;
//     let interval = 86400u64;
//     let recurrence_frequency = 2592000u64; // 30 days in seconds

//     env.mock_all_auths();

//     // Fund the employer with tokens
//     token_admin.mint(&employer, &10000);
//     token_admin.mint(&employer2, &10000);

//     // Initialize contract and deposit tokens
//     client.initialize(&employer);
//     client.deposit_tokens(&employer, &token_address, &5000i128);
//     client.create_or_update_escrow(&employer, &employee1, &token_address, &amount, &interval, &recurrence_frequency);

//     client.deposit_tokens(&employer2, &token_address, &1200i128);
//     client.create_or_update_escrow(&employer, &employee2, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee3, &token_address, &amount, &interval, &recurrence_frequency);

//     let payday1 =  recurrence_frequency; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let metrics_opt = client.get_metrics(&Some(0), &Some(payday1*2), &Some(10));

//     assert_eq!(metrics_opt.len(), 2); // 2 PerformanceMetrics
//     let first_metric = metrics_opt.get(0).unwrap();

//     assert_eq!(first_metric.employee_count, 3); 
//     assert_eq!(first_metric.total_disbursements, 0); 
//     assert_eq!(first_metric.total_amount, 6200); 
//     assert_eq!(first_metric.operation_count, 5); 
//     assert_eq!(first_metric.late_disbursements, 0);
//     assert_eq!(first_metric.operation_type_counts.get(symbol_short!("deposit")).unwrap_or(0), 2); 
//     assert_eq!(first_metric.operation_type_counts.get(symbol_short!("escrow")).unwrap_or(0), 3); 
//     assert_eq!(first_metric.operation_type_amount.get(symbol_short!("deposit")).unwrap_or(0), 6200); 
//     assert_eq!(first_metric.operation_type_amount.get(symbol_short!("escrow")).unwrap_or(0), 0); 

//     let second_metric = metrics_opt.get(1).unwrap();

//     assert_eq!(second_metric.total_disbursements, 2); 
//     assert_eq!(second_metric.total_amount, 2000); 
//     assert_eq!(second_metric.operation_count, 2); 
//     assert_eq!(second_metric.late_disbursements, 0);
//     assert_eq!(second_metric.operation_type_counts.get(symbol_short!("disburses")).unwrap_or(0), 2); 
//     assert_eq!(second_metric.operation_type_amount.get(symbol_short!("disburses")).unwrap_or(0), 2000); 
// }


// #[test]
// fn test_calculate_total_metrics() {
//     let (env, contract_id, client) = create_test_contract();
//     let (token_address, token_admin) = setup_token(&env);
//     let employer = Address::generate(&env);
//     let employee1 = Address::generate(&env);
//     let employee2 = Address::generate(&env);
//     let employee3 = Address::generate(&env);
//     let amount = 1000i128;
//     let interval = 86400u64;
//     let recurrence_frequency = 2592000u64; // 30 days in seconds

//     // Set initial ledger timestamp
//     let initial_timestamp = 1000u64;
//     env.ledger().set(LedgerInfo {
//         timestamp: initial_timestamp,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     env.mock_all_auths();

//     // Fund the employer with tokens
//     token_admin.mint(&employer, &100000);

//     // Verify minting
//     let token_client = TokenClient::new(&env, &token_address);
//     let employer_balance = token_client.balance(&employer);
//     assert_eq!(employer_balance, 100000);

//     // Initialize contract and deposit tokens
//     client.initialize(&employer);
//     client.deposit_tokens(&employer, &token_address, &50000i128);

//     // Create escrow for employees
//     client.create_or_update_escrow(&employer, &employee1, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee2, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee3, &token_address, &amount, &interval, &recurrence_frequency);

//     // Set ledger timestamp for first day
//     let payday1 = initial_timestamp + recurrence_frequency; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let payday1_late = initial_timestamp + recurrence_frequency + 1; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1_late,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee3);

//     // Set ledger timestamp for second day
//     let payday2 = payday1 + recurrence_frequency ; 
//     env.ledger().set(LedgerInfo {
//         timestamp: payday2,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     // Perform successful disbursement for employee2
//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let start = payday1;
//     let end = payday2;

//     let total_metrics_opt = client.calculate_total_metrics(&initial_timestamp, &end);
//     log!(&env, "TOTAL METRICS: {}", total_metrics_opt);

//     // Verify total metrics
//     let metrics = total_metrics_opt.unwrap();
//     assert_eq!(metrics.total_disbursements, 5); 
//     assert_eq!(metrics.total_amount, 55000);
//     assert_eq!(metrics.operation_count, 9); 
//     assert_eq!(metrics.late_disbursements, 1); // 
//     assert_eq!(metrics.operation_type_amount.get(symbol_short!("deposit")).unwrap_or(0), 50000); 
//     assert_eq!(metrics.operation_type_amount.get(symbol_short!("disburses")).unwrap_or(0), 5000); 
//     assert_eq!(metrics.operation_type_counts.get(symbol_short!("deposit")).unwrap_or(0), 1); 
//     assert_eq!(metrics.operation_type_counts.get(symbol_short!("disburses")).unwrap_or(0), 5); 
//     assert_eq!(metrics.operation_type_counts.get(symbol_short!("escrow")).unwrap_or(0), 3); 
// }


// #[test]
// fn test_calculate_total_deposited_token() {
//     let (env, contract_id, client) = create_test_contract();
//     let (token_address, token_admin) = setup_token(&env);
//     let employer = Address::generate(&env);
//     let employer2 = Address::generate(&env);
//     let employee1 = Address::generate(&env);
//     let employee2 = Address::generate(&env);
//     let employee3 = Address::generate(&env);
//     let amount = 1000i128;
//     let interval = 86400u64;
//     let recurrence_frequency = 2592000u64; // 30 days in seconds

//     // Set initial ledger timestamp
//     let initial_timestamp = 1000u64;
//     env.ledger().set(LedgerInfo {
//         timestamp: initial_timestamp,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     env.mock_all_auths();

//     // Fund the employer with tokens
//     token_admin.mint(&employer, &10000);
//     token_admin.mint(&employer2, &10000);

//     // Initialize contract and deposit tokens
//     client.initialize(&employer);
//     client.deposit_tokens(&employer, &token_address, &5000i128);
//     client.deposit_tokens(&employer2, &token_address, &1200i128);

//     // Create escrow for employees
//     client.create_or_update_escrow(&employer, &employee1, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee2, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee3, &token_address, &amount, &interval, &recurrence_frequency);

//     let payday1 = initial_timestamp + recurrence_frequency + 1; 
//     let payday2 = payday1 + recurrence_frequency ; 

//     let end = payday2;
//     let total_deposited_token = client.calculate_total_deposited_token(&initial_timestamp, &end).unwrap();
//     assert_eq!(total_deposited_token, 6200); // Three unique employees
// }


// #[test]
// fn test_calculate_total_metric_2() {
//     let (env, contract_id, client) = create_test_contract();
//     let (token_address, token_admin) = setup_token(&env);
//     let employer = Address::generate(&env);
//     let employer2 = Address::generate(&env);
//     let employee1 = Address::generate(&env);
//     let employee2 = Address::generate(&env);
//     let employee3 = Address::generate(&env);
//     let amount = 1000i128;
//     let interval = 86400u64;
//     let recurrence_frequency = 2592000u64; // 30 days in seconds

//     // Set initial ledger timestamp
//     let initial_timestamp = 1000u64;
//     env.ledger().set(LedgerInfo {
//         timestamp: initial_timestamp,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     env.mock_all_auths();

//     // Fund the employer with tokens
//     token_admin.mint(&employer, &100000);
//     token_admin.mint(&employer2, &100000);

//     // Verify minting
//     let token_client = TokenClient::new(&env, &token_address);
//     let employer_balance = token_client.balance(&employer);
//     assert_eq!(employer_balance, 100000);

//     // Initialize contract and deposit tokens
//     client.initialize(&employer);
//     client.deposit_tokens(&employer, &token_address, &50000i128);
//     client.deposit_tokens(&employer, &token_address, &10000i128);

//     // Create escrow for employees
//     client.create_or_update_escrow(&employer, &employee1, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee2, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee3, &token_address, &amount, &interval, &recurrence_frequency);

//     // Set ledger timestamp for first day
//     let payday1 = initial_timestamp + recurrence_frequency; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let payday1_late = initial_timestamp + recurrence_frequency + 1; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1_late,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee3);

//     // Set ledger timestamp for second day
//     let payday2 = payday1 + recurrence_frequency ; 
//     env.ledger().set(LedgerInfo {
//         timestamp: payday2,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     // Perform successful disbursement for employee2
//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let start = payday1;
//     let end = payday2;

//     let total_metrics_opt = client.calculate_total_metrics(&initial_timestamp, &end);
//     log!(&env, "TOTAL METRICS: {}", total_metrics_opt);

//     // Verify total metrics
//     let total_metrics = total_metrics_opt.unwrap();
//     assert_eq!(total_metrics.total_disbursements, 5); 
//     assert_eq!(total_metrics.total_amount, 65000);
//     assert_eq!(total_metrics.operation_count, 10); 
//     assert_eq!(total_metrics.late_disbursements, 1); // 
//     assert_eq!(total_metrics.operation_type_amount.get(symbol_short!("deposit")).unwrap_or(0), 60000); 
//     assert_eq!(total_metrics.operation_type_amount.get(symbol_short!("disburses")).unwrap_or(0), 5000); 
//     assert_eq!(total_metrics.operation_type_counts.get(symbol_short!("deposit")).unwrap_or(0), 2); 
//     assert_eq!(total_metrics.operation_type_counts.get(symbol_short!("disburses")).unwrap_or(0), 5); 
//     assert_eq!(total_metrics.operation_type_counts.get(symbol_short!("escrow")).unwrap_or(0), 3); 
// }


// #[test]
// fn test_calculate_avg_metrics() {
//     let (env, contract_id, client) = create_test_contract();
//     let (token_address, token_admin) = setup_token(&env);
//     let employer = Address::generate(&env);
//     let employer2 = Address::generate(&env);
//     let employee1 = Address::generate(&env);
//     let employee2 = Address::generate(&env);
//     let employee3 = Address::generate(&env);
//     let amount = 1000i128;
//     let interval = 86400u64;
//     let recurrence_frequency = 2592000u64; // 30 days in seconds

//     // Set initial ledger timestamp
//     let initial_timestamp = 1000u64;
//     env.ledger().set(LedgerInfo {
//         timestamp: initial_timestamp,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     env.mock_all_auths();

//     // Fund the employer with tokens
//     token_admin.mint(&employer, &100000);
//     token_admin.mint(&employer2, &100000);

//     // Verify minting
//     let token_client = TokenClient::new(&env, &token_address);
//     let employer_balance = token_client.balance(&employer);
//     assert_eq!(employer_balance, 100000);

//     // Initialize contract and deposit tokens
//     client.initialize(&employer);
//     client.deposit_tokens(&employer, &token_address, &50000i128);
//     client.deposit_tokens(&employer, &token_address, &10000i128);

//     // Create escrow for employees
//     client.create_or_update_escrow(&employer, &employee1, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee2, &token_address, &amount, &interval, &recurrence_frequency);
//     client.create_or_update_escrow(&employer, &employee3, &token_address, &amount, &interval, &recurrence_frequency);

//     // Set ledger timestamp for first day
//     let payday1 = initial_timestamp + recurrence_frequency; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let payday1_late = initial_timestamp + recurrence_frequency + 1; // Aligned to expected day
//     env.ledger().set(LedgerInfo {
//         timestamp: payday1_late,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     client.disburse_salary(&employer, &employee3);

//     // Set ledger timestamp for second day
//     let payday2 = payday1 + recurrence_frequency ; 
//     env.ledger().set(LedgerInfo {
//         timestamp: payday2,
//         protocol_version: 22,
//         sequence_number: env.ledger().sequence(),
//         network_id: Default::default(),
//         base_reserve: 0,
//         min_persistent_entry_ttl: 4096,
//         min_temp_entry_ttl: 16,
//         max_entry_ttl: 6312000,
//     });

//     // Perform successful disbursement for employee2
//     client.disburse_salary(&employer, &employee1);
//     client.disburse_salary(&employer, &employee2);

//     let start = payday1;
//     let end = payday2;

//     let total_metrics_opt = client.calculate_avg_metrics(&initial_timestamp, &end);
//     log!(&env, "AVERGAE METRICS: {}", total_metrics_opt);

//     // Verify total metrics
//     let avg_metrics = total_metrics_opt.unwrap();
//     // assert_eq!(total_metrics.total_disbursements, 5); 
//     // assert_eq!(total_metrics.total_amount, 65000);
//     // assert_eq!(total_metrics.operation_count, 10); 
//     // assert_eq!(total_metrics.late_disbursements, 1); // 
//     // assert_eq!(total_metrics.operation_type_amount.get(symbol_short!("deposit")).unwrap_or(0), 60000); 
//     // assert_eq!(total_metrics.operation_type_amount.get(symbol_short!("disburses")).unwrap_or(0), 5000); 
//     // assert_eq!(total_metrics.operation_type_counts.get(symbol_short!("deposit")).unwrap_or(0), 2); 
//     // assert_eq!(total_metrics.operation_type_counts.get(symbol_short!("disburses")).unwrap_or(0), 5); 
//     // assert_eq!(total_metrics.operation_type_counts.get(symbol_short!("escrow")).unwrap_or(0), 3); 
// }