use rgb_lib::{Wallet, Error, wallet::WalletData};
use crate::commands::Commands;
use super::{WState, WalletState, WalletOnline};


// In this state, the newly created Wallet will get a BTC Address for the funding
pub struct WalletInitiated {
    name    : String,
    wallet  : Wallet,
    state   : WState,
    btc_add : Vec<String>,
}


impl WalletInitiated {
    pub fn new(name : &str, wdata : &WalletData) -> Result<Self, Error> {
        let wallet = Wallet::new(wdata.clone())?;
        Ok(Self {
            name : String::from(name),
            wallet,
            state : WState::New,
            btc_add: vec![wallet.get_address()],
        })
    }
}

impl WalletState for WalletInitiated {
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
            }
            Commands::GoOnline(url) => {
                let mut w = self.wallet;
                match WalletOnline::new(
                    &self.name, self.btc_add, &mut w, url)
                {
                    Ok(w) => Box::new(w),
                    _ => Box::new(*self)
                }
            }
            _ => Box::new(*self)
        }
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}
