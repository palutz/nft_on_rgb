mod wallet;
pub use wallet::*;

mod wallet_new;
pub use wallet_new::WalletNew;

mod wallet_initiated;
pub use wallet_initiated::WalletInitiated;

mod wallet_online;
pub use wallet_online::WalletOnline;

mod wallet_blind_utxo;
pub use wallet_blind_utxo::WalletWBlindUTXO;

// pub struct WalletStErr;
// 
// impl fmt::Display for WalletStErr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Operation not allowed for this wallet")
//     }
// }
// 
// impl fmt::Debug for WalletStErr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Operation not allowed for this wallet")
//     }
// }