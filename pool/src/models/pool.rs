use core::time;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize}, json_types::U128, serde::{Deserialize, Serialize}, AccountId, PromiseOrValue
};

use super::{contract::Pool, PoolId};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolMetadata {
    pub pool_id: PoolId,
    pub owner_info: OwnerPoolInfo,
    pub list_user_deposit: Vec<DepositRecord>,
    pub list_mentor: Vec<MentorPoint>,
    pub time_mentor_join: u128,
    pub time_voting: u128,
    pub time_end: u128,
    pub ranking_entry: Vec<RankingEntry>,
    pub status: bool,
    pub winning_mentor: WinningMentorCourse,
    pub token_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WinningMentorCourse {
    pub winning_mentor: AccountId, // Mentor trúng thưởng
    pub course_name: String,       // Tên khóa học
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OwnerPoolInfo {
    pub owner_id: AccountId, // Mentor trúng thưởng
    // add more some attributes
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MentorPoint {
    pub mentor_id: AccountId, // Mentor trúng thưởng
    pub point: u128,       // Tên khóa học
    pub votes: u128, // Số lượng được vote
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DepositRecord {
    pub account_id: AccountId,
    pub balance_entry: RankingEntry,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RankingEntry {
    pub rank: String,
    pub coin: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum StatusPool {
    Init,
    Joining,
    Voting,
    Fail,
    Running,
}

pub trait PoolFeature {
    fn create_pool(
        &mut self,
        time_mentor_join: u128,
        time_voting: u128,
        time_end: u128,
        ranking_entry: Vec<RankingEntry>,
        token_id: String,
    ) -> PoolMetadata;

    fn deposit_near(
        &mut self,
        pool_id: PoolId,
        balance_entry: u128,
        token_id: String
    );

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn register_as_mentor(
        &mut self,
        pool_id: PoolId,
        mentor_id: AccountId,
    );

    fn calculate_winning_mentor(&self, pool_id: PoolId) -> Option<AccountId>;

}

pub trait PoolEnum {
    fn get_all_pool_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolMetadata>;

    fn get_pool_by_pool_id(&self, pool_id: PoolId) -> PoolMetadata;

}
