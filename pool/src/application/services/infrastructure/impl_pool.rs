use core::num;
use std::hash::RandomState;

use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseOrValue};

use crate::models::{
    contract::{Pool, PoolExt}, ft_request::external::cross_edu, pool::{self, DepositRecord, MentorPoint, OwnerPoolInfo, PoolFeature, PoolMetadata, RankingEntry, StatusPool, WinningMentorCourse}, PoolId
};

use super::refund_near;

pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const ATTACHED_TRANSFER_FT: u128 = 1;
pub const ATTACHED_STORAGE_DEPOSIT: u128 = 1_250_000_000_000_000_000_000;
pub const POOL_OPENING_FEE: Balance = 500_000_000_000_000_000_000; // 0.5 NEAR in yoctoNEAR

#[near_bindgen]
impl PoolFeature for Pool {
    #[payable]
    fn create_pool(
        &mut self,
        time_mentor_join: u128,
        time_voting: u128,
        time_end: u128,
        ranking_entry: Vec<RankingEntry>,
        token_id: String,
    ) -> PoolMetadata {
        let num_pool_id = self.all_pool_id.len();
        let pool_metadata = PoolMetadata {
            pool_id: num_pool_id,
            owner_info: OwnerPoolInfo {
                owner_id: env::signer_account_id(),
            },
            list_user_deposit: Vec::new(),
            list_mentor: Vec::new(),
            time_mentor_join,
            time_voting,
            time_end,
            ranking_entry: ranking_entry,
            status: StatusPool::Init,
            winning_mentor: WinningMentorCourse {
                winning_mentor: AccountId::new_unchecked("".to_string()),
                course_name: String::new(),
            },
            token_id: token_id.clone(),
        };

        assert!(
            env::attached_deposit() >= POOL_OPENING_FEE,
            "Requires a deposit of at least 0.5 NEAR"
        );

        if token_id != "near" {
            let ft_addr = AccountId::new_unchecked(token_id);
            cross_edu::ext(ft_addr.to_owned())
                .with_static_gas(GAS_FOR_CROSS_CALL)
                .with_attached_deposit(ATTACHED_STORAGE_DEPOSIT)
                .storage_deposit(env::current_account_id());
        }

        self.pool_metadata_by_id.insert(&pool_metadata.pool_id, &pool_metadata);
        self.all_pool_id.insert(&pool_metadata.pool_id);
        pool_metadata
    }

    fn deposit_near(
        &mut self,
        pool_id: PoolId,
        balance_entry: u128,
        token_id: String,
    ) {
        let pool = self.pool_metadata_by_id.get(&pool_id);
        assert!(pool.is_some(), "Course with the given pool_id does not exist");
    }

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        env::log_str(&format!("Received {} tokens from {}", amount.0, sender_id));

        
        let parts: Vec<&str> = msg.split('.').collect();

        if parts.len() != 3 {
            env::log_str("Invalid message format. Expected 'pool.number.mentor'.");
            return PromiseOrValue::Value(U128(0));
        }

        let pool = parts[0];
        let number = parts[1];
        let mentor = parts[2];
        let mentor_account_id = AccountId::new_unchecked(format!("{}.testnet", mentor));

        if pool == "pool" {
            env::log_str("Special usage detected, executing custom logic.");

            if let Ok(pool_id) = number.parse::<u64>() {
                if let Some(mut pool_metadata) = self.pool_metadata_by_id.get(&pool_id) {
                    if pool_metadata.ranking_entry.iter().any(|entry| entry.coin == amount.0) {
                        let record = DepositRecord {
                            account_id: sender_id.clone(),
                            balance_entry: pool_metadata.ranking_entry.iter().find(|entry| entry.coin == amount.0).unwrap().clone(),
                        };

                        if let Some(mentor_point) = pool_metadata.list_mentor.iter_mut().find(|m| m.mentor_id == mentor_account_id) {
                            mentor_point.point += amount.0;
                            mentor_point.votes += 1;
                        } else {
                            env::log_str("Mentor ID not found in list_mentor");
                            return PromiseOrValue::Value(U128(0));
                        }

                        pool_metadata.list_user_deposit.push(record);
                        self.pool_metadata_by_id.insert(&pool_id, &pool_metadata);
                    } else {
                        env::log_str("No matching balance entry found for the specified amount.");
                    }
                } else {
                    env::log_str("Pool metadata not found.");
                }
            } else {
                env::log_str("Failed to parse pool ID.");
            }
        }

        PromiseOrValue::Value(U128(0))
    }

    #[payable]
    fn register_as_mentor(&mut self, pool_id: PoolId, mentor_id: AccountId) {
        if let Some(mut pool_metadata) = self.pool_metadata_by_id.get(&pool_id) {
            if pool_metadata.list_mentor.iter().any(|m| m.mentor_id == mentor_id) {
                env::log_str("Mentor is already registered in this pool.");
                return;
            }

            let join_fee: u128 = 100_000_000_000_000_000_000_000;
            if env::attached_deposit() < join_fee {
                env::log_str("Insufficient deposit for joining as a mentor.");
                return;
            }

            let new_mentor = MentorPoint {
                mentor_id: mentor_id.clone(),
                point: 0,
                votes: 0,
            };
            pool_metadata.list_mentor.push(new_mentor);

            self.pool_metadata_by_id.insert(&pool_id, &pool_metadata);

            env::log_str("Mentor successfully registered.");
        } else {
            env::log_str("Pool metadata not found.");
        }
    }

    fn calculate_winning_mentor(&self, pool_id: PoolId) -> Option<AccountId> {
        if let Some(pool_metadata) = self.pool_metadata_by_id.get(&pool_id) {
            let winning_mentor = pool_metadata.list_mentor.iter()
                .max_by(|a, b| {
                    let a_ratio = if a.votes > 0 { a.point as f64 / a.votes as f64 } else { 0.0 };
                    let b_ratio = if b.votes > 0 { b.point as f64 / b.votes as f64 } else { 0.0 };
                    a_ratio.partial_cmp(&b_ratio).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|mentor| mentor.mentor_id.clone());

            return winning_mentor;
        }

        None
    }
}
