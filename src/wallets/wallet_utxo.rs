use std::borrow::BorrowMut;
use rgb_lib::{Wallet, Error};
use rgb_lib::wallet::{WalletData, Online};
use super::{WState, WInitiated, BtcWallet, WOnline, WalletState, WalletUTXO };
use crate::commands::Commands;

#[derive(Clone)]
pub struct WalletWUTXO {
    name    : String,
    wl_data : WalletData,
    btc_add : Vec<String>,  // 1:1 wallet - address!
    state   : WState,
    online  : Online,
    utxo    : u8,
}

impl WalletWUTXO {
    pub fn new<'a, W> (w1 : W, fee_rate: f32) -> Result<Self, Error> 
    where W : WInitiated<'a> + BtcWallet + WOnline<'a>
    {
        // we need the wallet to be mut to create the utxo
        let mut wtmp : Wallet = Wallet::new(*w1.wl_data())?;
        let utxo: u8 = wtmp.borrow_mut().create_utxos(
            w1.wonline().clone(), 
            false, 
            None, 
            None, 
            fee_rate)?;
        Ok(Self { 
            name: w1.name().to_string(), 
            wl_data: *w1.wl_data(),
            btc_add: w1.get_btc_address().to_vec(), 
            state: WState::UTXOPrepared, 
            online: w1.wonline().clone(),
            utxo: utxo,
        })
    }
}

impl WalletState for WalletWUTXO {
    fn execute(&self, cmd : Commands) -> Box<dyn WalletState> {
        match cmd {
            Commands::NewBTCAddress => {
                let wallet = Wallet::new(self.wl_data.clone()).unwrap();
                let mut tmp : Vec<String> = self.btc_add.to_vec();
                tmp.push(wallet.get_address());
                Box::new(
                    Self {
                        btc_add : tmp.to_vec(),
                        ..self.clone()
                    }
                )
            }
            Commands::InitBlindUTXO(endpoint ) => {
                let mut w : Wallet = Wallet::new(self.wl_data.clone()).unwrap();
                let blinded = w.blind(
                    None,
                    None,
                    None,
                    vec![endpoint.to_string()]);
                todo!()
            }
            _ => Box::new(self.clone())
        }
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}

impl WInitiated for WalletWUTXO {
    fn name(&self) -> &str {
        &self.name
    }

    fn wl_data(&self) -> &WalletData {
        &self.wl_data
    }
}

impl BtcWallet for WalletWUTXO {
    fn get_btc_address(&self) -> &Vec<String> {
        &self.btc_add
    }
}

impl WOnline for WalletWUTXO {
    fn wonline(&self) -> &Online {
        &self.online
    }
}

impl WalletUTXO for WalletWUTXO {
    fn get_utxo(&self) -> u8 {
        self.utxo
    }
}