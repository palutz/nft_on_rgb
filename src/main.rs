use rgb_lib::wallet::{DatabaseType, Wallet, WalletData};
use rgb_lib::{generate_keys, BitcoinNetwork};

fn main() -> Result<(), rgb_lib::Error> {
    let data_dir = tempfile::tempdir()?;
    let keys = generate_keys(BitcoinNetwork::Regtest);
    let wallet_data = WalletData {
        data_dir: data_dir.path().to_str().unwrap().to_string(),
        bitcoin_network: BitcoinNetwork::Regtest,
        database_type: DatabaseType::Sqlite,
        // max_allocations_per_utxo: 5,
        pubkey: keys.xpub,
        mnemonic: Some(keys.mnemonic),
        // vanilla_keychain: None,
    };
    let wallet = Wallet::new(wallet_data)?;

    let _ = wallet.get_address();

    Ok(())
}
