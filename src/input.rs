use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::fs::File;

use crate::{
    client::{ClientId, ClientsMap},
    process::process_record,
    transaction::{TransactionId, TransactionsMap},
};

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    #[serde(rename(deserialize = "type"))]
    pub record_type: String,
    pub client: ClientId,
    #[serde(rename(deserialize = "tx"))]
    pub transaction: TransactionId,
    pub amount: Option<String>,
}

pub fn process_input_file(
    filename: &str,
    clients: &mut ClientsMap,
    transactions: &mut TransactionsMap,
) -> Result<(), &'static str> {
    let file = File::open(filename).map_err(|_| "Failed to open input file")?;

    let mut reader = ReaderBuilder::new()
        // have to accept whitespace
        .trim(Trim::All)
        .from_reader(file);

    // line 1 is the header, data starts at line 2
    let mut line: u32 = 1;

    for result in reader.deserialize() {
        line += 1;

        let record: InputRecord = match result {
            Ok(r) => r,
            Err(_) => {
                eprintln!("line {}: error parsing input", line);
                continue;
            }
        };

        if let Err(error) = process_record(&record, clients, transactions) {
            eprintln!("line {}: {}", line, error);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{client::ClientsMap, input::process_input_file, transaction::TransactionsMap};

    #[test]
    fn single_deposit() {
        let filename = "test_data/single_deposit.csv";
        let mut clients = ClientsMap::new();
        let mut transactions = TransactionsMap::new();

        assert!(process_input_file(&filename, &mut clients, &mut transactions).is_ok());

        assert_eq!(clients.len(), 1);
        assert_eq!(transactions.len(), 1);

        let client = clients.get(&1).unwrap();
        assert_eq!(client.get_available(), 1_0000);
    }
}
