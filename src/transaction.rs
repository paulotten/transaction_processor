use std::collections::HashMap;

use crate::{cents::Cents, client::ClientId};

pub type TransactionId = u32;
pub type TransactionsMap = HashMap<TransactionId, TransactionData>;

pub enum TransactionData {
    Deposit(DepositData),
    Withdrawal(WithdrawalData),
}

pub struct DepositData {
    client: ClientId,
    amount: Cents,
    pub state: DepositState,
}

impl DepositData {
    pub fn new(client: ClientId, amount: Cents) -> Self {
        Self {
            client,
            amount,
            state: DepositState::Ok,
        }
    }

    pub fn get_client(&self) -> ClientId {
        self.client
    }

    pub fn get_amount(&self) -> Cents {
        self.amount
    }
}

#[derive(Debug, PartialEq)]
pub enum DepositState {
    Ok,
    Dispute,
    Chargeback,
}

pub struct WithdrawalData {
    _client: ClientId,
    _amount: Cents,
}

impl WithdrawalData {
    pub fn new(client: ClientId, amount: Cents) -> Self {
        Self {
            _client: client,
            _amount: amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DepositData, DepositState};

    #[test]
    fn depost_data() {
        let client = 1;
        let amount = 2;
        let depost = DepositData::new(client, amount);

        assert_eq!(depost.get_client(), client);
        assert_eq!(depost.get_amount(), amount);
        assert_eq!(depost.state, DepositState::Ok);
    }
}
