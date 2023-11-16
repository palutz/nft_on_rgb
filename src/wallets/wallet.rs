use core::fmt;
use crate::commands::Commands;

#[derive(Debug, Clone)]
pub enum WState {
    New,
    Funded,
    Online,
    UTXOPrepared,
    AssetIssued,
}

// Convert the enum to a String
impl fmt::Display for WState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait WalletState {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState>;
    fn get_state(&self) -> String;
}

pub trait WalletBlindUTXO {
    fn blind_receive(&self);
    fn witness_receive(&self);
}