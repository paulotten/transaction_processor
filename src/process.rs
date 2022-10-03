use crate::{
    cents::{string_to_cents, Cents},
    client::{ClientData, ClientId, ClientsMap},
    input::InputRecord,
    transaction::{
        DepositData, DepositState, TransactionData, TransactionId, TransactionsMap, WithdrawalData,
    },
};

pub fn process_record(
    record: &InputRecord,
    clients: &mut ClientsMap,
    transactions: &mut TransactionsMap,
) -> Result<(), &'static str> {
    match record.record_type.as_str() {
        "deposit" => {
            check_transaction_id(record.transaction, &transactions)?;
            let amount = get_amount(&record.amount)?;

            // find or create client
            let client_id = record.client;
            if !clients.contains_key(&client_id) {
                clients.insert(client_id, ClientData::new());
            }
            let client = get_client(client_id, clients)?;

            // apply deposit to client
            client.deposit(amount)?;

            // insert deposit into transactions map
            let transaction_id = record.transaction;
            transactions.insert(
                transaction_id,
                TransactionData::Deposit(DepositData::new(client_id, amount)),
            );

            Ok(())
        }
        "withdrawal" => {
            check_transaction_id(record.transaction, &transactions)?;
            let amount = get_amount(&record.amount)?;

            let client_id = record.client;
            let client = get_client(client_id, clients)?;

            // apply withdrawal to client
            client.withdrawal(amount)?;

            // insert deposit into transactions map
            let transaction_id = record.transaction;
            transactions.insert(
                transaction_id,
                TransactionData::Withdrawal(WithdrawalData::new(client_id, amount)),
            );

            Ok(())
        }
        "dispute" => {
            let deposit = get_deposit(record.transaction, record.client, transactions)?;
            check_amount_is_none(&record.amount)?;
            let client = get_client(record.client, clients)?;

            if deposit.state == DepositState::Ok {
                client.dispute(deposit.get_amount())?;
                deposit.state = DepositState::Dispute;

                Ok(())
            } else {
                Err("Deposit is not in a disputable state")
            }
        }
        "resolve" => {
            let deposit = get_deposit(record.transaction, record.client, transactions)?;
            check_amount_is_none(&record.amount)?;
            let client = get_client(record.client, clients)?;

            if deposit.state == DepositState::Dispute {
                client.resolve(deposit.get_amount())?;
                deposit.state = DepositState::Ok;

                Ok(())
            } else {
                Err("Deposit is not dispute")
            }
        }
        "chargeback" => {
            let deposit = get_deposit(record.transaction, record.client, transactions)?;
            check_amount_is_none(&record.amount)?;
            let client = get_client(record.client, clients)?;

            if deposit.state == DepositState::Dispute {
                client.chargeback(deposit.get_amount())?;
                deposit.state = DepositState::Chargeback;

                Ok(())
            } else {
                Err("Deposit is not dispute")
            }
        }
        _ => Err("Unsupported transaction type"),
    }
}

fn check_transaction_id(
    id: TransactionId,
    transactions: &TransactionsMap,
) -> Result<(), &'static str> {
    if transactions.contains_key(&id) {
        Err("Transaction id already exists")
    } else {
        Ok(())
    }
}

/*
Gets the DepositData for a TransactionId or returns an error.
Returns an error if a transaction is found but is not a deposit.

Also takes the expected ClientId and makes sure it matches the TransactionData.
*/
fn get_deposit<'a>(
    transaction_id: TransactionId,
    client_id: ClientId,
    transactions: &'a mut TransactionsMap,
) -> Result<&'a mut DepositData, &'static str> {
    match transactions.get_mut(&transaction_id) {
        Some(t) => match t {
            TransactionData::Deposit(d) => {
                if d.get_client() == client_id {
                    Ok(d)
                } else {
                    Err("Client ids do not match")
                }
            }
            _ => Err("Transaction is not a withdrawal"),
        },
        None => Err("Transaction not found"),
    }
}

fn check_amount_is_none(amount: &Option<String>) -> Result<(), &'static str> {
    if amount.is_none() {
        Ok(())
    } else {
        Err("Amount was expected to be empty, but it isn't")
    }
}

fn get_amount(amount: &Option<String>) -> Result<Cents, &'static str> {
    match amount {
        Some(amount) => string_to_cents(amount),
        None => Err("Amount missing"),
    }
}

fn get_client<'a>(
    id: ClientId,
    clients: &'a mut ClientsMap,
) -> Result<&'a mut ClientData, &'static str> {
    match clients.get_mut(&id) {
        Some(c) => Ok(c),
        None => Err("Client not found"),
    }
}

#[cfg(test)]
mod tests;
