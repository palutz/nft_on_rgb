
// Wallet that received the funds to start the RGB contract
pub struct WalletOnline {
    name    : String,
    wallet  : Wallet,
    btc_add : String,  // 1:1 wallet - address!
    state   : WalletStates,
    online  : Online,
}

impl WalletOnline {
    pub fn new (w1 : &WalletInitiated, electrum_url: &str) -> Result<Self, Error> {
        let online = w1.wallet.go_online(false, electrum_url.to_string())?;
        Ok(Self {
            name : w1.name,
            wallet: w1.wallet,
            btc_add: w1.btc_add,
            state: w1.state,
            online,
        })
    }
}

impl BtcWallet for WalletOnline {
    fn get_btc_address(&self) -> &str {
        &self.btc_add
    }

    fn get_new_btc_address(&mut self) -> &str {
        self.btc_add = self.wallet.get_address();
        &self.btc_add.as_str()
    }
}

// TO DO - insert fund message for all the wallet states supporting it 
//         - move all the states to a state subfolder

impl WalletState for WalletOnline {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
       match cmd {
        Commands::CreateUTXO => {

        }
        _ => Box::new(self),
       } 
    }

    fn get_state(&self) -> &str {
        self.state.to_string().as_str()
    }
}