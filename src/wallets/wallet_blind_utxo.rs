use std::borrow::BorrowMut;
use rgb_lib::{{Wallet, Error}, wallet::{WalletData, Online}};
use super::{WState, WInitiated, BtcWallet, WOnline, WalletState };

#[derive(Clone)]
pub struct WalletWBlindUTXO {
    name    : String,
    wl_data : WalletData,
    btc_add : Vec<String>,  // 1:1 wallet - address!
    state   : WState,
    online  : Online,
    utxo    : u8,
}

impl WalletWBlindUTXO {
    pub fn new<W> (w1 : W, fee_rate: f32) -> Result<Self, Error> 
    where W : WInitiated + BtcWallet + WOnline
    {
        // we need the wallet to be mut to create the utxo
        let wdata = w1.wl_data(); 
        let mut wtmp : Wallet = Wallet::new(wdata.clone())?;
        let utxo: u8 = wtmp.borrow_mut().create_utxos(
            w1.wonline().clone(), 
            false, 
            None, 
            None, 
            fee_rate)?;
        Ok(Self { 
            name: w1.name().to_string(), 
            wl_data: wdata,
            btc_add: w1.get_btc_address().to_vec(), 
            state: WState::UTXOPrepared, 
            online: w1.wonline().clone(),
            utxo: utxo,
        })
    }
}

impl WalletState for WalletWBlindUTXO {
    fn execute(&self, cmd : crate::commands::Commands) -> Box<dyn WalletState> {
        todo!()
    }

    fn get_state(&self) -> String {
        self.state.to_string()
    }
}