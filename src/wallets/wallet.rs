use core::fmt;
use crate::commands::Commands;
use rgb_lib::wallet::{Online, WalletData};

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

pub trait BtcWallet {
    fn get_btc_address(&self) -> &Vec<String>;
}

// Min set of functionalities for an Initiated Wallet
pub trait WInitiated  {
    fn name(&self) -> &str;    
    fn wl_data(&self) -> WalletData;
}

pub trait WOnline {
    fn wonline(&self) -> &Online;
}

pub trait WalletWUTXO {
    fn get_utxo() -> u8;
}

pub trait WalletBlindUTXO {
    fn blind_receive();
    fn witness_receive();
}