use rgb_lib::{Wallet, Error, wallet::{WalletData, Online}};

use crate::commands::Commands;
use super::wallet::WOnline;
use super::{WalletState, BtcWallet, WState, WInitiated, WalletWUTXO};


// Wallet that received the funds to start the RGB contract
#[derive(Clone)]
pub struct WalletOnline {
    name    : String,
    wl_data : WalletData,
    btc_add : Vec<String>,  // 1:1 wallet - address!
    state   : WState,
    online  : Online,
}

impl WalletOnline {
    pub fn new<'a, W> (w1 : W, electrum_url: &str) -> Result<Self, Error> 
    where W : WInitiated<'a> + BtcWallet
    {
        let wdata = w1.wl_data().clone(); 
        let mut wallet = Wallet::new(*w1.wl_data())?;
        let online = wallet.go_online(false, electrum_url.to_string())?;
        Ok(Self {
            name : w1.name().to_string(),
            wl_data: w1.wl_data().clone(),
            btc_add: w1.get_btc_address().to_vec(),
            state: WState::Online,
            online,
        })
    }
}

// TO DO - insert fund message for all the wallet WState supporting it 
//         - move all the WState to a state subfolder
impl WalletState for WalletOnline {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
       match cmd {
            Commands::NewBTCAddress => {
                let wallet = Wallet::new(self.wl_data.clone()).unwrap();
                let mut tmp : Vec<String> = self.btc_add.to_vec();
                tmp.push(wallet.get_address());
                Box::new(
                    WalletOnline {
                        btc_add : tmp.to_vec(),
                        ..self.clone()
                    }
                )
            }
            Commands::CreateUTXO(fee ) => {
                match WalletWUTXO::new(self.clone(), fee) {
                    Ok(w) => Box::new(w),
                    _ => Box::new(self.clone()),
                }
            }
            _ => Box::new(self.clone()),
       } 
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}

impl WInitiated for WalletOnline {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn wl_data(&self) -> &WalletData {
        &self.wl_data
    }
}

impl BtcWallet for WalletOnline {
    fn get_btc_address(&self) -> &Vec<String> {
        self.btc_add.as_ref()
    }
}

impl WOnline for WalletOnline {
    fn wonline(&self) -> &Online {
        &self.online
    }
}
