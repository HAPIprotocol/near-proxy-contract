# HAPI Protocol

[HAPI Protocol] is a one-of-a-kind decentralized security protocol that prevents and interrupts any potential malicious activity within the blockchain space. HAPI Protocol works by leveraging both external and off-chain data as well as on-chain data accrued directly by HAPI Protocol and is publicly available.

## HAPI NEAR Proxy

The HAPI NEAR Proxy is a smart contract used for replicating data from [HAPI Protocol] main contract on the NEAR blockchain. It acts as an interface between the HAPI Protocol and the NEAR blockchain, allowing data to be replicated from the protocol by oracles.

Reporters are entities added to the contract by the protocol authority which can report data to the protocol.

To check an address of interest for security data, consumers should call the `get_address` method.

## Methods

Each role can call its methods and the methods of roles below it.

### Owner methods

- new - initialize contract.
- change_owner - transfer ownership to new owner.

### Authority methods

- create_reporter - add new reporter with corresponding permission level.
- update_reporter - update permission level for reporter.

### Reporter methods

- create_address - add new address with corresponding category and risk level
- update_address - update address category and risk.

### User methods

- get_address - return risk level and category.
- get_reporter - return permission level.

## Integration

Consumers can integrate the HAPI NEAR Proxy Contract using the [hapi-near-connector](https://github.com/HAPIProtocol/hapi-near-connector).This crate helps to implement [HAPI Protocol] in your smart contract on the NEAR blockchain.

Alternatively, consumers can do it directly. As an example, [Jumbo Exchange](https://github.com/jumbo-exchange/contracts#hapi-protocol-integration) has integrated the HAPI NEAR Proxy Contract into their platform.

## For developers

### CLI installation

You can install cli via this [tutorial](https://docs.near.org/docs/tools/near-cli#installation)

### Getting started

For creating the new account for deploying contract use next command.

Create variables

```bash
export NEAR_ENV=testnet
export CONTRACT_ID=contract.hapi-test.testnet
export OWNER_ID=hapi-test.testnet
export REPORTER_ID=reporter.hapi-test.testnet
```

```bash
near create-account $CONTRACT_ID --masterAccount $OWNER_ID --initialBalance 10
```

First of all - you will need to compile the wasm file of contract.

```bash
./contract/build_docker.sh
```

Then deploy it.

```bash
near deploy $CONTRACT_ID --wasmFile=contract/res/proxy_contract_release.wasm
```

Then initialize contract with command where OWNER_ID is your admin UI account.

```bash
near call $CONTRACT_ID new '{"owner_id": "'$OWNER_ID'"}' --accountId $CONTRACT_ID
```

## Useful commands

- NEW

```bash
near call $CONTRACT_ID new '{"owner_id": "'$OWNER_ID'"}' --account_id=$OWNER_ID
```

- CHANGE OWNER

```bash
near call $CONTRACT_ID change_owner '{"owner_id": "NEW_OWNER_ID"}' --account_id=$OWNER_ID
```

- CREATE REPORTER

  - address - account_id of reporter
  - permission_level - permission level corresponding to the table

| Role | permission_level |
| ------ | ------ |
| Reporter | 1 |
| Authority | 2 |

```bash
near call $CONTRACT_ID create_reporter '{"address": "'$REPORTER_ID'", "permission_level": 2}' --account_id=$OWNER_ID
```

- UPDATE REPORTER

  - address - account_id of reporter
  - permission_level - permission level corresponding to the table

| Role | permission_level |
| ------ | ------ |
| Reporter | 1 |
| Authority | 2 |

```bash
near call $CONTRACT_ID update_reporter '{"address": "'$REPORTER_ID'", "permission_level": 1 }' --accountId=$OWNER_ID
```

- GET REPORTER

Returns permission level of reporter

| Role | permission_level |
| ------ | ------ |
| Reporter | 1 |
| Authority | 2 |

```bash
near call $CONTRACT_ID get_reporter '{"address": "'$REPORTER_ID'" }' --accountId=$OWNER_ID
```

- CREATE ADDRESS

  - address - address which should be updated
  - category - category from list of [Categories]
  - risk - risk level also described in [Categories] section

```bash
near call $CONTRACT_ID create_address '{"address": "address.id", "category": "Scam", "risk": 6}' --accountId=$REPORTER_ID
```

- UPDATE ADDRESS

  - address - address which should be updated
  - category - category from list of [Categories]
  - risk - risk level also described in [Categories] section

```bash
near call $CONTRACT_ID update_address '{"address": "address.id", "category": "WalletService", "risk": 6}' --accountId=$REPORTER_ID
```

- GET ADDRESS

This method returns tuple of Category and u8 (risk level). List of [Categories].

```bash
near view $CONTRACT_ID get_address '{"address": "address.id"}'
```

## Categories

If the address belongs to some category, it will have a
Risk score (on a scale from 0..10, i.e. max risk).

| Category | Description |
|----------|-------|
| None | |
| WalletService | Wallet service - custodial or mixed wallets |
| MerchantService | Merchant service |
| MiningPool | Mining pool |
| LowRiskExchange | Low-risk exchange - Exchange with high KYC standards |
| MediumRiskExchange | Medium risk exchange |
| DeFi | DeFi application |
| OTCBroker | OTC Broker |
| ATM | Cryptocurrency ATM |
| Gambling | Gambling |
| IllicitOrganization | Illicit organization |
| Mixer | Mixer |
| DarknetService | Darknet market or service |
| Scam | Scam |
| Ransomware | Ransomware |
| Theft | Theft - stolen funds |
| Counterfeit | Counterfeit - fake assets |
| TerroristFinancing | Terrorist financing |
| ChildAbuse | Child abuse and porn materials |

[HAPI Protocol]: https://hapi-one.gitbook.io/hapi-protocol/
[Categories]: (#categories)
