use rgb_lib::{Wallet, Error};
use rgb_lib::wallet::Online;
use super::{WState, WalletState};
use crate::commands::Commands;

pub struct WalletWUTXO {
    name    : String,
    wallet  : Wallet,
    btc_add : Vec<String>, 
    state   : WState,
    online  : Online,
    utxo    : u8,
}

impl WalletWUTXO {
    pub fn new (name: &str, btc_add: Vec<String>, wallet: &mut Wallet, online: Online, fee_rate: f32) -> Result<Self, Error> 
    {
        // we need the wallet to be mut to create the utxo
        let utxo: u8 = wallet.create_utxos(
            online,
            false, 
            None, 
            None, 
            fee_rate)?;
        Ok(Self { 
            name: name.to_string(), 
            wallet: *wallet,
            btc_add: btc_add, 
            state: WState::UTXOPrepared, 
            online,
            utxo: utxo,
        })
    }
}

impl WalletState for WalletWUTXO {
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
            Commands::InitBlindUTXO(endpoint ) => {
                let blinded = self.wallet.blind(
                    None,
                    None,
                    None,
                    vec![endpoint.to_string()]);
                todo!()
            },
            _ => Box::new(*self),
        }
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}
