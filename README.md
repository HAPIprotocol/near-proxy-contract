# HAPI proxy contract

## CLI installation

You can install cli via this [tutorial](https://docs.near.org/docs/tools/near-cli#installation)


## Getting started

Create sale preparing process

For creating the new account for deploying contract use next command 

```
near create-account proxy.boca.testnet --masterAccount boca.testnet --initialBalance 10
```
Then using launchpad5.boca.testnet as accountId

Create constants
```
export NEAR_ENV=testnet
export CONTRACT_ID=proxy.boca.testnet
```

First of all - you will need to compile the wasm file of contracts and then deploy it like that
```
near deploy $CONTRACT_ID --wasmFile=contract/target/wasm32-unknown-unknown/release/proxy_contract.wasm
```
Then initialize contract with command where OWNER_ID is your admin UI account. 

```
near call $CONTRACT_ID new '{"owner_id": "'$OWNER_ID'"}' --accountId $CONTRACT_ID
```


## Useful commands:

1. CHANGE OWNER

```
near call $CONTRACT_ID change_owner '{"owner_id": "NEW_OWNER_ID"}' --account_id=$CONTRACT_ID
```

2. CREATE REPORTER

```
near call $CONTRACT_ID create_reporter '{"reporter_address": "reporter.id", "permission_level": 2}' --account_id=$CONTRACT_ID
```

3. UPDATE REPORTER

```
near call $CONTRACT_ID update_sale_distribute_token_id '{"sale_id": 2, "distribute_token_id": "token.solniechniy.testnet" }' --accountId $CONTRACT_ID
```

4. CREATE ADDRESS

```
near call $CONTRACT_ID update_sale_claim_available '{"sale_id": 0, "claim_available": true }' --accountId $CONTRACT_ID
```

5. UPDATE ADDRESS

```
near call $CONTRACT_ID update_sale_refund_available '{"sale_id": 0, "refund_available": true }' --accountId $CONTRACT_ID
```

