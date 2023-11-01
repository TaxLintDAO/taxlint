pub(crate) use candid::{CandidType, Deserialize, Principal};

use crate::common::context::TimeStamp;

use super::service::{WalletAddress, WalletId};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletProfile {
    // primary key
    pub id: u64,
    pub holder: Principal,

    // frontend para input
    pub address: String, // immutable.
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,

    // backend auto-gen
    pub create_time: u64, //ic_cdk::api::time();

    pub transactions: u64, //transactions count
    pub last_sync_time: u64,
    pub last_transaction_time: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletAddCommand {
    pub address: String,
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletUpdateCommand {
    // pub address: String,
    // pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
    pub id: WalletId,
    // pub transactions:u64,
    // pub last_sync_time:u64,
    // pub last_transaction_time:u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct RecordProfile {
    pub id:u64,
    // Primary key
    pub address: WalletAddress,
    //Transaction record
    pub price: f64,
    pub amount: u32,
    // todo , considering:
    // pub wallet_amount:u32,
    pub time: TimeStamp, //transaction_time
    pub t_type: String,  //transaction_type
    pub tag: String,
    pub manual: bool,
    pub comment: String,
    // pub warning:String,
    // todo: Warning（用户是否标记某些记录为missing cost, missing rates）这条字段先只做出来，不用,
    // 解释：比如missing rates是标记某个交易历史找不到对应的价格记录，例如某个NFT的交易价格查不到，就会被自动标记为missing rates
}
