use rgb_lib::wallet::{DatabaseType, WalletData};
use rgb_lib::{generate_keys, BitcoinNetwork, Error};
use crate::commands::*;
use super::{WState, WalletState, WalletInitiated};

#[derive(Clone)]
pub struct WalletNew {
    name    : String,
    wl_data : WalletData,
    state   : WState,
}

impl WalletNew {
    pub fn new(name : String, network: BitcoinNetwork) -> Result<Self, Error> {
        let data_dir = tempfile::tempdir()?;
        let keys = generate_keys(network);
        let wl_data = WalletData {
            data_dir: data_dir.path().to_str().unwrap().to_string(),
            bitcoin_network: network,
            database_type: DatabaseType::Sqlite,
            pubkey: keys.xpub,
            mnemonic: Some(keys.mnemonic),
        };
        Ok(Self {
            name,
            wl_data,
            state: WState::New,
        })
    }
}

impl WalletState for WalletNew {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
        match cmd {
            Commands::InitWallet => {
                match WalletInitiated::new(&self.name, &self.wl_data) {
                    Ok(w) => Box::new(w),
                    Err(e) => {
                        println!("Error: {}", e);
                        Box::new(self.clone())
                    }
                }
            }
            _ => Box::new(self.clone())  // Action not permitted (At this stage, so returning the same wallet unchanged)
        }
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}
