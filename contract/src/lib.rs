use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
// use serde::{Deserialize, Serialize};
use near_sdk::serde::Serialize;

// use near_sdk::{env, log, near_bindgen, AccountId};

use near_sdk::collections::LookupMap;

#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
pub enum Category {
    // Wallet service - custodial or mixed wallets
    WalletService,
    // Merchant service
    MerchantService,
    // Mining pool
    MiningPool,
    // Low risk exchange - Exchange with high KYC standards
    LowRiskExchange,
    // Medium eisk exchange
    MediumRiskExchange,
    // DeFi application
    DeFi,
    // OTC Broker
    OTCBroker,
    // Cryptocurrency ATM
    ATM,
    // Gambling
    Gambling,
    // Illicit organization
    IllicitOrganization,
    // Mixer
    Mixer,
    // Darknet market or service
    DarknetService,
    // Scam
    Scam,
    // Ransomware
    Ransomware,
    // Theft - stolen funds
    Theft,
    // Counterfeit - fake assets
    Counterfeit,
    // Terrorist financing
    TerroristFinancing,
    // Sanctions
    Sanctions,
    // Child abuse and porn materials
    ChildAbuse,
}

#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
pub struct AddressInfo {
    category: Category,
    risk: u8,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proxy {
    owner_id: AccountId,
    pub reporters: LookupMap<AccountId, u8>,
    pub addresses: LookupMap<AccountId, AddressInfo>,
}

const MAX_RISK: u8 = 10;
const MAX_PERMISSION_LEVEL: u8 = 2;

impl Default for Proxy {
    fn default() -> Self {
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Proxy {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            reporters: LookupMap::new(b"r"),
            addresses: LookupMap::new(b"a"),
        }
    }

    pub fn change_owner(&mut self, owner_id: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this method"
        );
        self.owner_id = owner_id;
    }

    pub fn create_reporter(&mut self, address: AccountId, permission_level: u8) -> Option<u8> {
        assert!(
            permission_level <= MAX_PERMISSION_LEVEL,
            "HapiProxy: Invalid permission level"
        );
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this method"
        );
        self.reporters.insert(&address, &permission_level)
    }

    pub fn get_reporter(&self, reporter_address: AccountId) -> Option<u8> {
        self.reporters.get(&reporter_address)
    } // return permission level

    pub fn update_reporter(&mut self, reporter_address: AccountId, permission_level: u8) -> bool {
        assert!(
            permission_level <= MAX_PERMISSION_LEVEL,
            "HapiProxy: Invalid permission level bitch"
        );
        assert!(
            self.reporters.contains_key(&reporter_address),
            "HapiProxy: Reporter does not exist"
        );
        self.reporters.insert(&reporter_address, &permission_level);
        true
    }

    pub fn create_address(&mut self, address: AccountId, category: Category, risk: u8) -> bool {
        assert!(
            self.get_reporter(env::predecessor_account_id()).unwrap() < MAX_PERMISSION_LEVEL,
            "HapiProxy: Invalid permission level"
        );
        assert!(risk <= MAX_RISK, "HapiProxy: Invalid risk");
        let address_info = AddressInfo { category, risk };
        self.addresses.insert(&address, &address_info);
        true
    }

    pub fn get_address(&self, _address: AccountId) -> (Category, u8) {
        (
            self.addresses.get(&_address).unwrap().category,
            self.addresses.get(&_address).unwrap().risk,
        )
    } // return risk level and category

    pub fn update_address(&mut self, address: AccountId, category: Category, risk: u8) {
        assert!(
            self.get_reporter(env::predecessor_account_id()).unwrap() < MAX_PERMISSION_LEVEL,
            "HapiProxy: Invalid permission level"
        );
        assert!(risk <= MAX_RISK, "HapiProxy: Invalid risk");
        assert!(
            self.addresses.contains_key(&address),
            "HapiProxy: Address does not exist"
        );
        let address_info = AddressInfo { category, risk };
        self.addresses.insert(&address, &address_info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Category;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    #[test]
    fn test_new() {
        let account_id: AccountId = "tester".to_string();
        let contract = Proxy::new(account_id.clone());
        assert_eq!(contract.owner_id, "tester");
    }

    #[test]
    fn test_get_reporter() {
        let mut context = VMContextBuilder::new();
        let test_level: u8 = 1;
        let account_id: AccountId = "alice".to_string();
        let reporter_id: AccountId = "reporter".to_string();
        let mut contract = Proxy::new(account_id.clone());
        testing_env!(context.predecessor_account_id(accounts(0)).build());

        contract.create_reporter(reporter_id.clone(), test_level);
        assert_eq!(
            contract.get_reporter(reporter_id.clone()).unwrap(),
            test_level,
            "reporter value is: {}",
            contract.get_reporter(reporter_id).unwrap().to_string()
        );
    }

    #[test]
    fn test_update_reporter() {
        let mut context = VMContextBuilder::new();
        let account_id: AccountId = "alice".to_string();
        let reporter_id: AccountId = "reporter".to_string();
        let mut contract = Proxy::new(account_id.clone());
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        contract.create_reporter(reporter_id.clone(), 1);
        assert!(
            contract.update_reporter(reporter_id.clone(), 2),
            "Reporter update failed"
        );

        assert_eq!(
            contract.get_reporter(reporter_id.clone()).unwrap(),
            2,
            "reporter value is: {}",
            contract.get_reporter(reporter_id).unwrap().to_string()
        );
    }

    #[test]
    fn test_get_address() {
        let mut context = VMContextBuilder::new();
        let account_id: AccountId = "alice".to_string();
        let address_id: AccountId = "MiningPool".to_string();
        let mut contract = Proxy::new(account_id.clone());
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        contract.create_reporter(account_id.clone(), 2);
        contract.create_address(address_id, Category::MiningPool, 7);
        // assert_eq!(
        //     contract.get_address(address_id),
        //     (Category::MiningPool, 7),
        //     "Address not writed normally"
        // );
    }
}

// near call proxy.contracts.sergei24.testnet new '{"owner_id": "sergei24.testnet"}' --accountId proxy.contracts.sergei24.testnet

//near call proxy.contracts.sergei24.testnet change_owner '{"owner_id": "contracts.sergei24.testnet"}' --accountId proxy.contracts.sergei24.testnet
//near call proxy.contracts.sergei24.testnet create_reporter '{"reporter_address": "reporter", "permission_level": 1}' --accountId sergei24.testnet
//near call proxy.contracts.sergei24.testnet
