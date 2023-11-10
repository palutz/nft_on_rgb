pub trait Command {}

pub enum Commands<'a> {
    InitWallet, // to add data for init
    GoOnline(&'a str),
    NewBTCAddress,
    CreateUTXO(f32), // with online wallet address
    CheckUnspents,
    InitBlindUTXO,
    IssueCFA,
}

// Change all to Messages enum???
pub enum Queries {
    ListAssets,
    CheckPendingTransfers,
}

