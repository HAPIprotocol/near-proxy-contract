# HAPI proxy contract

## CLI installation

You can install cli via this [tutorial](https://docs.near.org/docs/tools/near-cli#installation)


## Getting started

Create sale preparing process

For creating the new account for deploying contract use next command 

```
near create-account $CONTRACT_ID --masterAccount $OWNER_ID --initialBalance 10
```
Then using launchpad5.boca.testnet as accountId

Create constants
```
export NEAR_ENV=testnet
export CONTRACT_ID=contract.hapi-test.testnet
export OWNER_ID=hapi-test.testnet
```

First of all - you will need to compile the wasm file of contracts and then deploy it like that
```
cargo build --release --target wasm32-unknown-unknown

near deploy $CONTRACT_ID --wasmFile=contract/res/proxy_contract_release.wasm
```
Then initialize contract with command where OWNER_ID is your admin UI account. 

```
near call $CONTRACT_ID new '{"owner_id": "'$OWNER_ID'"}' --accountId $CONTRACT_ID
```


## Useful commands:

1. NEW

```
near call $CONTRACT_ID new '{"owner_id": "your id"}' --account_id=$OWNER_ID
```

2. CHANGE OWNER

```
near call $CONTRACT_ID change_owner '{"owner_id": "NEW_OWNER_ID"}' --account_id=$OWNER_ID
```

3. CREATE REPORTER

```
near call $CONTRACT_ID create_reporter '{"address": "reporter.id", "permission_level": 2}' --account_id=$OWNER_ID
```

4. UPDATE REPORTER

```
near call $CONTRACT_ID update_reporter '{"address": "reporter.id", "permission_level": 1 }' --accountId=$OWNER_ID
```

5. GET REPORTER

```
near call $CONTRACT_ID get_reporter '{"address": "'$OWNER_ID'" }' --accountId=$OWNER_ID
```

6. CREATE ADDRESS

```
near call $CONTRACT_ID create_address '{"address": "address.id", "category": "Scam", "risk": 6}' --accountId=$OWNER_ID
```

7. UPDATE ADDRESS

```
near call $CONTRACT_ID update_address '{"address": "address.id", "category": "WalletService", "risk": 6}' --accountId=$OWNER_ID
```

8. GET ADDRESS

```
near call $CONTRACT_ID get_address '{"address": "address.id"}' --accountId=$OWNER_ID
```
