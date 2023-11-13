use super::{BlindDataRGB, WState, WInitiated, BtcWallet, WOnline, WalletBlindUTXO};
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
    pub fn new(name: String, btc_add: Vec<String>, bl_data: BlindDataRGB) -> Self {
        let wallet = Wallet::new(wdata.clone())?;
        wallet.blind(asset_id, amount, duration_seconds, consignment_endpoints)
}

impl WInitiated for WalletWBlindUTXO {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn wl_data(&self) -> &WalletData {
        &self.wl_data
    }
}

impl BtcWallet for WalletWBlindUTXO {
    fn get_btc_address(&self) -> &Vec<String> {
        self.btc_add.as_ref()
    }
}

impl WOnline for WalletWBlindUTXO {
    fn wonline(&self) -> &Online {
        &self.online
    }
}

impl WalletBlindUTXO for WalletWBlindUTXO {
    fn blind_receive(&self) {
        self.wa
    }
    fn witness_receive(&self) {
        todo!()
    }
}