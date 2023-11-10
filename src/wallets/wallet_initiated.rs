use rgb_lib::{Wallet, Error, wallet::WalletData};
use crate::commands::Commands;
use super::{WalletOnline, BtcWallet, WState, WalletState, WInitiated};


// In this state, the newly created Wallet will get a BTC Address for the funding
#[derive(Clone)]
pub struct WalletInitiated {
    name    : String,
    wl_data : WalletData,
    state   : WState,
    btc_add : Vec<String>,  // 1:1 wallet - address!
}


impl WalletInitiated {
    pub fn new(name : &str, wdata : WalletData) -> Result<Self, Error> {
        let wallet = Wallet::new(wdata.clone())?;
        Ok(Self {
            name : String::from(name),
            wl_data : wdata,
            state : WState::New,
            btc_add: vec![wallet.get_address()],
        })
    }
}

impl WalletState for WalletInitiated {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
        match cmd {
            Commands::NewBTCAddress => {
                let wallet = Wallet::new(self.wl_data.clone()).unwrap();
                let mut tmp : Vec<String> = self.btc_add.to_vec();
                tmp.push(wallet.get_address());
                Box::new(
                    WalletInitiated {
                        btc_add : tmp.to_vec(),
                        ..self.clone()
                }
                )
            }
            Commands::GoOnline(url) => {
                match WalletOnline::new(self.clone(), url) {
                    Ok(w) => Box::new(w),
                    _ => Box::new(self.clone()),  // some errors... state not changed
                }
            }
            _ => Box::new(self.clone())
        }
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}

impl WInitiated for WalletInitiated {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn wl_data(&self) -> WalletData {
        self.wl_data.clone()
    }
}

// Checking the BTC address.
// WE always have at least 1 BTC address since we create one when the wallet get in this state
impl BtcWallet for WalletInitiated {
    fn get_btc_address(&self) -> &Vec<String> {
        &self.btc_add
    }
}
