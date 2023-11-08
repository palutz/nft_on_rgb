mod states;
pub use states::WalletStates;
mod wallet;
pub use wallet::{BtcWallet, WalletState};

pub mod wallet_new;
pub use wallet_new::WalletNew;

pub mod wallet_initiated;
pub use wallet_initiated::WalletInitiated;