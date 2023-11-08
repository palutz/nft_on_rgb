use core::fmt;
use crate::commands::Commands;

pub trait WalletState {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState>;
    fn get_state(&self) -> &str;
}

pub trait BtcWallet {
    fn get_btc_address(&self) -> &str;
    fn get_new_btc_address(&mut self) -> &str;
}

pub struct WalletStErr;

impl fmt::Display for WalletStErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation not allowed for this wallet")
    }
}

impl fmt::Debug for WalletStErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation not allowed for this wallet")
    }
}
