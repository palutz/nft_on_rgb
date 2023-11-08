use rgb_lib::wallet::{DatabaseType, Wallet, WalletData};
use rgb_lib::{generate_keys, BitcoinNetwork};
use crate::commands::*;
use crate::wallets::{WalletState, WalletStates, WalletInitiated};

pub struct WalletNew {
    name    : String,
    wallet  : Wallet,
    state   : WalletStates,
}

impl WalletNew {
    pub fn new(name : String, network: BitcoinNetwork) -> Result<Self, rgb_lib::Error> {
        let data_dir = tempfile::tempdir()?;
        let keys = generate_keys(network);
        let wallet_data = WalletData {
            data_dir: data_dir.path().to_str().unwrap().to_string(),
            bitcoin_network: network,
            database_type: DatabaseType::Sqlite,
            pubkey: keys.xpub,
            mnemonic: Some(keys.mnemonic),
        };
        let w : Wallet = Wallet::new(wallet_data)?;
        Ok(Self {
            name,
            wallet: w,
            state: WalletStates::New,
        })
    }
}

impl WalletState for WalletNew {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
        match cmd {
            Commands::InitWallet => {
                Box::new(WalletInitiated::new(&self.name, &self.wallet))
            }
            _ => Box::new(*self)  // Action not permitted (At this stage, so returning the same wallet unchanged)
        }
    }
    fn get_state(&self) -> &str {
        self.state.to_string().as_str()
    }
}

