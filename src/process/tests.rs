use crate::{
    client::ClientsMap,
    input::InputRecord,
    process::{get_deposit, process_record},
    transaction::{DepositState, TransactionsMap},
};

#[test]
fn deposit_single() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let record = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&record, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
}

#[test]
fn deposit_multiple() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let record = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&record, &mut clients, &mut transactions).is_ok());

    let record = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 2,
        amount: Some("2.5".to_string()),
    };

    assert!(process_record(&record, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 2);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 3_5000);
}

#[test]
fn deposit_duplicate() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let record = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    // original
    assert!(process_record(&record, &mut clients, &mut transactions).is_ok());

    // duplicate
    assert!(process_record(&record, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
}

#[test]
fn withdrawal_ok() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let withdrawal = InputRecord {
        record_type: "withdrawal".to_string(),
        client: 1,
        transaction: 2,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&withdrawal, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 2);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 0);
}

#[test]
fn withdrawal_duplicate() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let withdrawal = InputRecord {
        record_type: "withdrawal".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&withdrawal, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
}

#[test]
fn withdrawal_invalid_client() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let withdrawal = InputRecord {
        record_type: "withdrawal".to_string(),
        client: 2,
        transaction: 2,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&withdrawal, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
}

#[test]
fn withdrawal_insufficent_funds() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let withdrawal = InputRecord {
        record_type: "withdrawal".to_string(),
        client: 1,
        transaction: 2,
        amount: Some("2".to_string()),
    };

    assert!(process_record(&withdrawal, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
}

#[test]
fn dispute_ok() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 0);
    assert_eq!(client.get_held(), 1_0000);
    assert_eq!(client.get_total(), 1_0000);

    let deposit = get_deposit(1, 1, &mut transactions).unwrap();
    assert_eq!(deposit.state, DepositState::Dispute);
}

#[test]
fn dispute_with_amount() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
    assert_eq!(client.get_held(), 0);
    assert_eq!(client.get_total(), 1_0000);

    let deposit = get_deposit(1, 1, &mut transactions).unwrap();
    assert_eq!(deposit.state, DepositState::Ok);
}

#[test]
fn dispute_invalid_transaction_id() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_err());
}

#[test]
fn dispute_client_ids_do_not_match() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 2,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
    assert_eq!(client.get_held(), 0);
    assert_eq!(client.get_total(), 1_0000);

    let deposit = get_deposit(1, 1, &mut transactions).unwrap();
    assert_eq!(deposit.state, DepositState::Ok);
}

#[test]
fn dispute_invalid_state() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    // first dispute succeeds
    assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

    // second dispute fails because the state is already disputed
    assert!(process_record(&dispute, &mut clients, &mut transactions).is_err());
}

#[test]
fn resolve_ok() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

    let resolve = InputRecord {
        record_type: "resolve".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&resolve, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
    assert_eq!(client.get_held(), 0);
    assert_eq!(client.get_total(), 1_0000);

    let deposit = get_deposit(1, 1, &mut transactions).unwrap();
    assert_eq!(deposit.state, DepositState::Ok);
}

#[test]
fn resolve_invalid_state() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let resolve = InputRecord {
        record_type: "resolve".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&resolve, &mut clients, &mut transactions).is_err());
}

#[test]
fn chargeback_ok() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

    let chargeback = InputRecord {
        record_type: "chargeback".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&chargeback, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 0);
    assert_eq!(client.get_held(), 0);
    assert_eq!(client.get_total(), 0);
    assert!(client.is_locked());

    let deposit = get_deposit(1, 1, &mut transactions).unwrap();
    assert_eq!(deposit.state, DepositState::Chargeback);
}

#[test]
fn chargeback_invalid_state() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let chargeback = InputRecord {
        record_type: "chargeback".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&chargeback, &mut clients, &mut transactions).is_err());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert_eq!(client.get_available(), 1_0000);
    assert_eq!(client.get_held(), 0);
    assert_eq!(client.get_total(), 1_0000);
    assert!(!client.is_locked());

    let deposit = get_deposit(1, 1, &mut transactions).unwrap();
    assert_eq!(deposit.state, DepositState::Ok);
}

#[test]
fn account_locked() {
    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 1,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

    let dispute = InputRecord {
        record_type: "dispute".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

    let chargeback = InputRecord {
        record_type: "chargeback".to_string(),
        client: 1,
        transaction: 1,
        amount: None,
    };

    assert!(process_record(&chargeback, &mut clients, &mut transactions).is_ok());

    assert_eq!(clients.len(), 1);
    assert_eq!(transactions.len(), 1);

    let client = clients.get(&1).unwrap();
    assert!(client.is_locked());

    // now that the account is locked additional deposits should fail

    let deposit = InputRecord {
        record_type: "deposit".to_string(),
        client: 1,
        transaction: 2,
        amount: Some("1".to_string()),
    };

    assert!(process_record(&deposit, &mut clients, &mut transactions).is_err());

    assert_eq!(transactions.len(), 1);
}
