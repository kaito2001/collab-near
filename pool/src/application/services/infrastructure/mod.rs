use near_sdk::env;

use crate::models::{pool::DepositRecord, ft_request::external::cross_edu};

pub mod impl_pool;

pub fn refund_near(list_user_deposit: Vec<DepositRecord>) {
    for user_deposit in list_user_deposit {       
            let promise_id = env::promise_batch_create(&user_deposit.account_id);
            env::promise_batch_action_transfer(promise_id, user_deposit.balance_entry.coin);
    }
}