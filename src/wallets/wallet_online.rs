use std::{rc::Rc, borrow::BorrowMut};
use rgb_lib::{Wallet, Error, wallet::Online};
use crate::commands::Commands;
use super::{WalletState, WState, WalletWUTXO};


// Wallet that received the funds to start the RGB contract
pub struct WalletOnline {
    name    : String,
    wallet  : Wallet,
    btc_add : Vec<String>,
    state   : WState,
    online  : Online,
}

impl WalletOnline {
    pub fn new (name: &str, btc_add: Vec<String>, wallet: &mut Wallet, electrum_url: &str) -> Result<Self, Error> 
    {
        let online = wallet.go_online(false, electrum_url.to_string())?;
        Ok(Self {
            name : name.to_string(),
            wallet: *wallet,
            btc_add,
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
                let mut tmp : Vec<String> = vec!();
                tmp.push(self.wallet.get_address());
                Box::new(
                    Self {
                        btc_add : vec![self.btc_add, tmp].concat(),
                        ..*self
                    }
                )
            },
            Commands::CreateUTXO(fee ) => {
                match WalletWUTXO::new(
                    &self.name, self.btc_add, self.wallet.borrow_mut(), self.online, fee)
                {
                    Ok(w) => Box::new(w),
                    _ => Box::new(*self),
                }
            },
            _ => Box::new(*self),
       } 
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}
