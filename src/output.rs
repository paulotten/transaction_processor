use crate::{
    cents::cents_to_string,
    client::{ClientData, ClientId, ClientsMap},
};

pub fn write_accounts(clients: &ClientsMap) {
    // header
    println!("client,available,held,total,locked");

    // body
    let mut client_ids: Vec<_> = clients.keys().collect();
    client_ids.sort();

    for client_id in client_ids {
        let client = clients.get(client_id).unwrap();
        println!("{}", format_client(*client_id, client));
    }
}

fn format_client(id: ClientId, client: &ClientData) -> String {
    format!(
        "{},{},{},{},{}",
        id,
        cents_to_string(client.get_available()),
        cents_to_string(client.get_held()),
        cents_to_string(client.get_total()),
        client.is_locked(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        client::ClientsMap, input::InputRecord, output::format_client, process::process_record,
        transaction::TransactionsMap,
    };

    #[test]
    fn basic() {
        let mut clients = ClientsMap::new();
        let mut transactions = TransactionsMap::new();
        let client_id = 1;

        let deposit = InputRecord {
            record_type: "deposit".to_string(),
            client: client_id,
            transaction: 1,
            amount: Some("0.1234".to_string()),
        };

        assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

        let client = clients.get(&client_id).unwrap();
        assert_eq!(
            format_client(client_id, client),
            "1,0.1234,0,0.1234,false".to_string(),
        );
    }

    #[test]
    fn held() {
        let mut clients = ClientsMap::new();
        let mut transactions = TransactionsMap::new();
        let client_id = 1;

        let deposit = InputRecord {
            record_type: "deposit".to_string(),
            client: client_id,
            transaction: 1,
            amount: Some("0.1234".to_string()),
        };

        assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

        let dispute = InputRecord {
            record_type: "dispute".to_string(),
            client: client_id,
            transaction: 1,
            amount: None,
        };

        assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

        let client = clients.get(&client_id).unwrap();
        assert_eq!(
            format_client(client_id, client),
            "1,0,0.1234,0.1234,false".to_string(),
        );
    }

    #[test]
    fn locked() {
        let mut clients = ClientsMap::new();
        let mut transactions = TransactionsMap::new();
        let client_id = 1;

        let deposit = InputRecord {
            record_type: "deposit".to_string(),
            client: client_id,
            transaction: 1,
            amount: Some("0.1234".to_string()),
        };

        assert!(process_record(&deposit, &mut clients, &mut transactions).is_ok());

        let dispute = InputRecord {
            record_type: "dispute".to_string(),
            client: client_id,
            transaction: 1,
            amount: None,
        };

        assert!(process_record(&dispute, &mut clients, &mut transactions).is_ok());

        let chargeback = InputRecord {
            record_type: "chargeback".to_string(),
            client: client_id,
            transaction: 1,
            amount: None,
        };

        assert!(process_record(&chargeback, &mut clients, &mut transactions).is_ok());

        let client = clients.get(&client_id).unwrap();
        assert_eq!(format_client(client_id, client), "1,0,0,0,true".to_string(),);
    }
}
