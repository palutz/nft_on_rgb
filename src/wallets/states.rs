use core::fmt;

#[derive(Debug)]
pub enum WalletStates {
    New,
    Funded,
    Online,
    UTXOPrepared,
    AssetIssued,
}

// Convert the enum to a String
impl fmt::Display for WalletStates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}