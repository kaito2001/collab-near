use models::contract::{PoolStorageKey, Pool, PoolExt};
use near_sdk::borsh::BorshSerialize;
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    env, near_bindgen, AccountId,
};

pub mod application;
pub mod models;

#[near_bindgen]
impl Pool {
    #[init]
    pub fn init() -> Self {
        let owner_id = env::signer_account_id();

        Self::new(owner_id)
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            all_pool_id: UnorderedSet::new(PoolStorageKey::AllPoolId.try_to_vec().unwrap()),
            pool_metadata_by_id: LookupMap::new(
                PoolStorageKey::PoolMetadataById.try_to_vec().unwrap(),
            )
        }
    }
}
