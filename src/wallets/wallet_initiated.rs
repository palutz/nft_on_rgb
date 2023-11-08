// In this state, the newly created Wallet will get a BTC Address for the funding
pub struct WalletInitiated {
    name    : String,
    wallet  : Wallet,
    state   : WalletStates,
    btc_add : String,  // 1:1 wallet - address!
}


impl WalletInitiated {
    pub fn new(name : &str, wallet : &Wallet) -> Self {
        Self {
            name : String::from(name),
            wallet : *wallet.clone(),
            btc_add: wallet.get_address(),
            state: WalletStates::New,
        }
    }
}

impl BtcWallet for WalletInitiated {
    fn get_btc_address(&self) -> &str {
        &self.btc_add
    }

    fn get_new_btc_address(&mut self) -> &str {
        self.btc_add = self.wallet.get_address();
        &self.btc_add.as_str()
    }
}

impl WalletState for WalletInitiated {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
        match cmd {
            Commands::GoOnline(url) => {
                match WalletOnline::new(self, url) {
                    Ok(w) => Box::new(w),
                    _ => Box::new(*self),  // some errors... state not changed
                }
            }
            _ => Box::new(*self)
        }
    }
    fn get_state(&self) -> &str {
        self.state.to_string().as_str()
    }
}