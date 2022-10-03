use std::collections::HashMap;

use crate::cents::Cents;

pub type ClientId = u16;
pub type ClientsMap = HashMap<ClientId, ClientData>;

pub struct ClientData {
    available: Cents,
    held: Cents,
    locked: bool,
}

impl ClientData {
    pub fn new() -> Self {
        Self {
            available: 0,
            held: 0,
            locked: false,
        }
    }

    pub fn get_available(&self) -> Cents {
        self.available
    }

    pub fn get_held(&self) -> Cents {
        self.held
    }

    pub fn get_total(&self) -> Cents {
        self.available + self.held
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn deposit(&mut self, cents: Cents) -> Result<(), &'static str> {
        Self::check_positive(cents)?;
        self.check_locked()?;

        self.available += cents;

        Ok(())
    }

    pub fn withdrawal(&mut self, cents: Cents) -> Result<(), &'static str> {
        Self::check_positive(cents)?;
        self.check_locked()?;

        if self.available >= cents {
            self.available -= cents;

            Ok(())
        } else {
            Err("Insufficient available funds for withdrawal")
        }
    }

    pub fn dispute(&mut self, cents: Cents) -> Result<(), &'static str> {
        Self::check_positive(cents)?;
        self.check_locked()?;

        self.available -= cents;
        self.held += cents;

        Ok(())
    }

    pub fn resolve(&mut self, cents: Cents) -> Result<(), &'static str> {
        Self::check_positive(cents)?;
        self.check_locked()?;

        self.available += cents;
        self.held -= cents;

        Ok(())
    }

    pub fn chargeback(&mut self, cents: Cents) -> Result<(), &'static str> {
        Self::check_positive(cents)?;
        self.check_locked()?;

        self.held -= cents;
        self.locked = true;

        Ok(())
    }

    fn check_locked(&self) -> Result<(), &'static str> {
        if !self.locked {
            Ok(())
        } else {
            Err("Account is locked")
        }
    }

    fn check_positive(cents: Cents) -> Result<(), &'static str> {
        if cents >= 0 {
            Ok(())
        } else {
            Err("Amount may not be negative")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientData;

    #[test]
    fn deposit() {
        let mut client = ClientData::new();
        assert!(client.deposit(100).is_ok());

        assert_eq!(client.get_available(), 100);
        assert_eq!(client.get_held(), 0);
        assert_eq!(client.get_total(), 100);
    }

    #[test]
    fn negative_cents() {
        let mut client = ClientData::new();
        assert!(client.deposit(-100).is_err());
        assert!(client.withdrawal(-100).is_err());
        assert!(client.dispute(-100).is_err());
        assert!(client.resolve(-100).is_err());
        assert!(client.chargeback(-100).is_err());

        assert_eq!(client.get_available(), 0);
        assert_eq!(client.get_held(), 0);
        assert_eq!(client.get_total(), 0);
    }

    #[test]
    fn withdrawal_ok() {
        // 100 - 20 = 80
        let mut client = ClientData::new();
        assert!(client.deposit(100).is_ok());

        assert!(client.withdrawal(20).is_ok());

        assert_eq!(client.get_available(), 80);
        assert_eq!(client.get_held(), 0);
        assert_eq!(client.get_total(), 80);

        // 100 - 100 = 0
        let mut client = ClientData::new();
        assert!(client.deposit(100).is_ok());

        assert!(client.withdrawal(100).is_ok());

        assert_eq!(client.get_available(), 0);
        assert_eq!(client.get_held(), 0);
        assert_eq!(client.get_total(), 0);
    }

    #[test]
    fn withdrawal_insufficent_funds() {
        let mut client = ClientData::new();
        assert!(client.deposit(100).is_ok());

        assert!(client.withdrawal(101).is_err());

        assert_eq!(client.get_available(), 100);
        assert_eq!(client.get_held(), 0);
        assert_eq!(client.get_total(), 100);
    }

    #[test]
    fn dispute() {
        let mut client = ClientData::new();
        assert!(client.dispute(100).is_ok());

        assert_eq!(client.get_available(), -100);
        assert_eq!(client.get_held(), 100);
        assert_eq!(client.get_total(), 0);
    }

    #[test]
    fn resolve() {
        let mut client = ClientData::new();
        assert!(client.resolve(100).is_ok());

        assert_eq!(client.get_available(), 100);
        assert_eq!(client.get_held(), -100);
        assert_eq!(client.get_total(), 0);
    }

    #[test]
    fn chargeback() {
        let mut client = ClientData::new();
        assert!(client.chargeback(100).is_ok());

        assert_eq!(client.get_available(), 0);
        assert_eq!(client.get_held(), -100);
        assert_eq!(client.get_total(), -100);

        assert!(client.is_locked());
    }

    #[test]
    fn locked() {
        // create and immediately lock an account
        let mut client = ClientData::new();
        assert!(client.chargeback(100).is_ok());

        assert!(client.deposit(100).is_err());
        assert!(client.withdrawal(100).is_err());
        assert!(client.dispute(100).is_err());
        assert!(client.resolve(100).is_err());
        assert!(client.chargeback(100).is_err());
    }
}
