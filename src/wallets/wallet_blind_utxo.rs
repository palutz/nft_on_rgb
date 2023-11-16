use super::{BlindDataRGB, WState,  WalletBlindUTXO, WalletState, };
use std::rc::Rc;
use crate::commands::Commands;
use rgb_lib::{Wallet, Error};
use rgb_lib::wallet::{WalletData, Online};


#[derive(Clone)]
pub struct WalletWBlindUTXO {
    name    : String,
    wl_data : WalletData,
    btc_add : Vec<String>,  // 1:1 wallet - address!
    state   : WState,
    online  : Online,
    utxo    : u8,
    bl_data : BlindDataRGB,
}

impl WalletWBlindUTXO {
    pub fn new (name: &str, btc_add: Rc<[&str]>, wl: WalletData, online: &Online, bl_data: BlindDataRGB) -> Result<Self, Error>  {
        let wallet = Wallet::new(wl)?;
        // wallet.blind(asset_id, amount, duration_seconds, consignment_endpoints)

        todo!();
    }
}

// TODO - Implement REFRESH wallet and when received? 
impl WalletState for WalletWBlindUTXO {
    fn execute(&self, cmd : crate::commands::Commands) -> Box<dyn WalletState> {
        match cmd {
            Commands::NewBTCAddress => {
                let wallet = Wallet::new(self.wl_data.clone()).unwrap();
                let mut tmp : Vec<String> = self.btc_add.to_vec();
                tmp.push(wallet.get_address());
                Box::new(
                    WalletWBlindUTXO {
                        btc_add : tmp.to_vec(),
                        ..self.clone()
                    }
                )
            }
            _ => Box::new(self.clone())
        }
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}

impl WalletBlindUTXO for WalletWBlindUTXO {
    fn blind_receive(&self) {
        todo!();
    }
    fn witness_receive(&self) {
        todo!()
    }
}
