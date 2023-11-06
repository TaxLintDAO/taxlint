use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::task::Context;

use ic_cdk_macros::{query, update};
use ic_stable_structures::BTreeMap;

use super::domain::*;
use super::service::{RecordId, WalletAddress};
use crate::common::guard::user_owner_guard;
use crate::CONTEXT;

const MAX_WALLET_NAME_LENGTH: usize = 64;

#[update(guard = "user_owner_guard")]
fn add_wallet(wallet_add_command: WalletAddCommand) -> Result<bool, String> {
    CONTEXT.with(|c| {
        if wallet_add_command.name.len() > MAX_WALLET_NAME_LENGTH {
            return Err(String::from("Wallet name exceeds maximum length 64"));
        }
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        let now = ctx.env.now();
        let id = ctx.id;
        let profile = WalletProfile {
            holder: caller,
            address: wallet_add_command.address,
            from: wallet_add_command.from,
            name: wallet_add_command.name,
            id: id,
            create_time: now,
            transactions: 0,
            last_sync_time: 0,
            last_transaction_time: 0,
        };
        match ctx.wallet_service.add_wallet(profile, caller) {
            Some(_) => {
                ctx.id += 1;
                Ok(true)
            }
            None => Err("Can not add wallet,address duplicated".to_string()),
        }
    })
}

#[update(guard = "user_owner_guard")]
fn update_wallet(wallet_update_command: WalletUpdateCommand) -> Result<bool, String> {
    CONTEXT.with(|c| {
        if wallet_update_command.name.len() > MAX_WALLET_NAME_LENGTH {
            return Err(String::from("Wallet name exceeds maximum length 64"));
        }
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        let now = ctx.env.now();
        let id: u64 = wallet_update_command.id;
        let mut profile = ctx.wallet_service.query_a_wallet(id).unwrap().clone();
        // holder: caller,
        // profile.address=wallet_update_command.address;
        // profile.from=wallet_update_command.from;
        profile.name = wallet_update_command.name;
        // id: id,
        // profile.create_time=now;
        // todo
        // transactions: now,
        // last_sync_time: now,
        // last_transaction_time: now,
        match ctx.wallet_service.update_wallet(profile, caller) {
            Some(_) => Ok(true),
            None => Err("Can not update wallet".to_string()),
        }
    })
}

#[query(guard = "user_owner_guard")]
fn query_a_wallet(id: u64) -> Result<WalletProfile, String> {
    CONTEXT.with(|c| {
        let ctx = c.borrow_mut();
        let wallet = match ctx.wallet_service.query_a_wallet(id) {
            Some(wallet) => wallet.clone(),
            None => {
                return Err("wallet not exsit".to_string());
            }
        };
        return Ok(wallet.clone());
    })
}

#[query(guard = "user_owner_guard")]
fn query_all_wallets() -> Result<Vec<WalletProfile>, Vec<WalletProfile>> {
    CONTEXT.with(|c| {
        let ctx = c.borrow_mut();
        let user = ctx.env.caller();
        let wallets = ctx.wallet_service.query_wallet_array(user);
        return Ok(wallets);
    })
}

#[update(guard = "user_owner_guard")]
fn delete_wallet(id: u64) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        ctx.wallet_service
            .delete_wallet(id)
            .ok_or(String::from("Wallet Not Found"))
    })
}

// todo use: AddRecordCommand . front end dont need to input id . id gen by backend.
#[update(guard = "user_owner_guard")]
fn add_transaction_record(cmd: AddRecordCommand) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let mut p = RecordProfile {
            id: id,
            address: cmd.address,
            price: cmd.price,
            amount: cmd.amount,
            time: cmd.time,
            t_type: cmd.t_type,
            tag: cmd.tag,
            manual: cmd.manual,
            comment: cmd.comment,
        };
        p.id = id;
        let ret = ctx.wallet_record_service.add_transaction_record(p);
        match ret {
            Ok(_) => {
                ctx.id += 1;
                return Ok(true);
            }
            Err(msg) => Err(msg),
        }
    })
}

#[update(guard = "user_owner_guard")]
fn delete_transaction_record(id: RecordId) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let ret = ctx.wallet_record_service.delete_transaction_record(id);
        match ret {
            Ok(_) => Ok(true),
            Err(msg) => Err(msg),
        }
    })
}

#[update(guard = "user_owner_guard")]
fn edit_transaction_record(cmd: EditHistoryCommand) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let service = ctx.wallet_record_service.borrow_mut();

        let addr = service.get_addr_from_id(cmd.id);
        let ret = service.add_transaction_record(convert_edit_command_to_record_profile(cmd, addr));
        match ret {
            Ok(_) => Ok(true),
            Err(msg) => Err(msg),
        }
    })
}

// todo
/**
 * 方法完成后，需要检查关联更新：钱包的交易记录总数，上次同步时间，上次交易发生的时间
描述:户点击同步钱包按钮,调用nns或者交易所等api.获得历史交易记录并存储到后端.
(前端已有一部分计算代码),可以选择全部搬移到后端或者前端直接把现有计算好的利润发送给后端
 */
#[update(guard = "user_owner_guard")]
fn sync_transaction_record(cmd: EditHistoryCommand) -> Result<bool, String> {
    return Err("sync fail".to_string());
}

// todo get all wallets of records info
#[query(guard = "user_owner_guard")]
fn wallet_history(
    cmd: HistoryQueryCommand,
) -> Result<HashMap<WalletAddress, Vec<RecordProfile>>, String> {
    return Err("no data".to_string());
}

fn convert_edit_command_to_record_profile(
    cmd: EditHistoryCommand,
    addr: WalletAddress,
) -> RecordProfile {
    RecordProfile {
        id: cmd.id,
        address: addr,
        price: cmd.price,
        amount: cmd.amount,
        time: cmd.time,
        t_type: cmd.t_type,
        tag: cmd.tag,
        manual: cmd.manual,
        comment: cmd.comment,
    }
}
