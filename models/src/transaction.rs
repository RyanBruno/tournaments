use serde::{Serialize as SarSerialize, Deserialize as SarDeserialize};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use std::collections::HashSet;

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransactionCategory {
    Swap,
    Trade,
    Transfer,
    Deposit,
    Withdrawal,
    Unknown,
}

#[derive(Debug, Default, Clone)]
pub struct Categorizer {
    pub swap_addresses: HashSet<String>,
    pub trade_addresses: HashSet<String>,
    pub transfer_addresses: HashSet<String>,
    pub deposit_addresses: HashSet<String>,
    pub withdrawal_addresses: HashSet<String>,
}

impl Categorizer {
    pub fn categorize(&self, from: &str, to: &str) -> TransactionCategory {
        if self.swap_addresses.contains(from) || self.swap_addresses.contains(to) {
            TransactionCategory::Swap
        } else if self.trade_addresses.contains(from) || self.trade_addresses.contains(to) {
            TransactionCategory::Trade
        } else if self.transfer_addresses.contains(from) || self.transfer_addresses.contains(to) {
            TransactionCategory::Transfer
        } else if self.deposit_addresses.contains(to) {
            TransactionCategory::Deposit
        } else if self.withdrawal_addresses.contains(from) {
            TransactionCategory::Withdrawal
        } else {
            TransactionCategory::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categorizes_based_on_addresses() {
        let mut c = Categorizer::default();
        c.deposit_addresses.insert("alice".into());
        c.withdrawal_addresses.insert("bob".into());
        c.swap_addresses.insert("dex".into());

        assert_eq!(c.categorize("charlie", "alice"), TransactionCategory::Deposit);
        assert_eq!(c.categorize("bob", "dave"), TransactionCategory::Withdrawal);
        assert_eq!(c.categorize("dex", "eve"), TransactionCategory::Swap);
        assert_eq!(c.categorize("foo", "bar"), TransactionCategory::Unknown);
    }
}
