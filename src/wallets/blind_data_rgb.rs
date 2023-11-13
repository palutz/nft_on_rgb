use rgb_lib::wallet::BlindData;


// BlindData struct has no Clone
#[derive(Debug, Clone)]
pub struct BlindDataRGB {
    /// Bench32 invoice
     pub invoice: String,
    /// Blinded UTXO
    pub blinded_utxo: String,
    /// Secret used to blind the UTXO
    pub blinding_secret: u64,
    /// Expiration of the `blinded_utxo`
    pub expiration_timestamp: Option<i64>,
}

impl From<BlindData> for BlindDataRGB {
    fn from(value: BlindData) -> Self {
        Self {
            blinded_utxo: value.blinded_utxo,
            blinding_secret: value.blinding_secret,
            invoice: value.invoice,
            expiration_timestamp: value.expiration_timestamp, 
        }
    }
}