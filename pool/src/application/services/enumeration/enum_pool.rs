use near_sdk::{env, near_bindgen};

use crate::models::{
    contract::{Pool, PoolExt},
    pool::{PoolEnum, PoolMetadata}, PoolId,
};

#[near_bindgen]
impl PoolEnum for Pool {
    fn get_all_pool_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolMetadata> {
        self.all_pool_id
            .iter()
            .skip(start.unwrap_or(0) as usize)
            .take(limit.unwrap_or(20) as usize)
            .map(|x| self.pool_metadata_by_id.get(&x).unwrap())
            .collect()
    }

    fn get_pool_by_pool_id(&self, pool_id: PoolId) -> PoolMetadata {
        let pool = self.pool_metadata_by_id.get(&pool_id).unwrap();

        pool
    }
    
}
