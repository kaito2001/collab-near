use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    json_types::Base64VecU8,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
};

use super::pool::PoolMetadata;
use super::PoolId;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Pool {
    /// Account ID of the owner of the contract.  
    pub owner_id: AccountId,

    pub all_pool_id: UnorderedSet<PoolId>,

    pub pool_metadata_by_id: LookupMap<PoolId, PoolMetadata>,

}

#[derive(BorshSerialize)]
pub enum PoolStorageKey {
    AllPoolId,
    PoolMetadataById,
}
