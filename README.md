# Contracts for testnet 

```bash
# deploy & init
near deploy $ID ./target/wasm32-unknown-unknown/release/pool.wasm
near call $ID init --accountId $ID

# create pool
near call collab_2.testnet create_pool '{"time_mentor_join": 100, "time_voting": 1000, "time_end": 2000, "ranking_entry": [{"rank": "silver", "coin": 10000000}, {"rank": "gold", "coin": 20000000}], "token_id": "fun_token2"}' --accountId creator1.testnet --deposit 0.5
```