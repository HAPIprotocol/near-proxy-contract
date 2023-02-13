use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

use near_sdk::collections::LookupMap;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Category {
    None,
    // Wallet service - custodial or mixed wallets
    WalletService,
    // Merchant service
    MerchantService,
    // Mining pool
    MiningPool,
    // Exchange
    Exchange,
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct AddressInfo {
    category: Category,
    risk: u8,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Proxy {
    owner_id: AccountId,
    pub reporters: LookupMap<AccountId, u8>,
    pub addresses: LookupMap<AccountId, AddressInfo>,
}

const MAX_RISK: u8 = 10;
const MAX_PERMISSION_LEVEL: u8 = 2;

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
            "HapiProxy: Only the owner may call this method"
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
            "HapiProxy: Only the owner may call this method"
        );
        assert!(
            !self.reporters.contains_key(&address),
            "HapiProxy: Reporter already exist"
        );

        self.reporters.insert(&address, &permission_level)
    }

    pub fn get_reporter(&self, address: AccountId) -> u8 {
        self.reporters
            .get(&address)
            .expect("HapiProxy: Reporter does not exist")
    } // return permission level

    pub fn update_reporter(&mut self, address: AccountId, permission_level: u8) -> bool {
        assert!(
            permission_level <= MAX_PERMISSION_LEVEL,
            "HapiProxy: Invalid permission level"
        );
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "HapiProxy: Only the owner may call this method"
        );
        assert!(
            self.reporters.contains_key(&address),
            "HapiProxy: Reporter does not exist"
        );
        self.reporters.insert(&address, &permission_level);
        true
    }

    pub fn create_address(&mut self, address: AccountId, category: Category, risk: u8) -> bool {
        let reporter_level = self.get_reporter(env::predecessor_account_id());
        assert!(
            reporter_level <= MAX_PERMISSION_LEVEL && reporter_level > 1,
            "HapiProxy: Invalid permission level",
        );
        assert!(risk <= MAX_RISK, "HapiProxy: Invalid risk");
        assert!(
            !self.addresses.contains_key(&address),
            "HapiProxy: Address already exist"
        );
        let address_info = AddressInfo { category, risk };
        self.addresses.insert(&address, &address_info);
        true
    }

    pub fn get_address(&self, address: AccountId) -> (Category, u8) {
        if let Some(address_info) = self.addresses.get(&address) {
            (address_info.category, address_info.risk)
        } else {
            (Category::None, 0)
        }
    } // return risk level and category

    pub fn update_address(&mut self, address: AccountId, category: Category, risk: u8) {
        let reporter_level = self.get_reporter(env::predecessor_account_id());
        assert!(
            reporter_level <= MAX_PERMISSION_LEVEL && reporter_level > 1,
            "HapiProxy: Invalid permission level",
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
    use std::str::FromStr;

    use super::*;

    use crate::Category;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    pub fn get_account_id(account_id: &str) -> AccountId {
        AccountId::from_str(account_id)
            .unwrap_or_else(|_| panic!("ERR: can't get account_id from str: {account_id}"))
    }

    #[test]
    fn test_new() {
        let account_id: AccountId = get_account_id("tester");
        let contract = Proxy::new(account_id.clone());
        assert_eq!(contract.owner_id, account_id);
    }

    #[test]
    fn test_get_reporter() {
        let mut context = VMContextBuilder::new();
        let test_level: u8 = 1;
        let account_id: AccountId = get_account_id("alice");
        let reporter_id: AccountId = get_account_id("reporter");
        let mut contract = Proxy::new(account_id);
        testing_env!(context.predecessor_account_id(accounts(0)).build());

        contract.create_reporter(reporter_id.clone(), test_level);
        assert_eq!(
            contract.get_reporter(reporter_id.clone()),
            test_level,
            "reporter value is: {}",
            contract.get_reporter(reporter_id)
        );
    }

    #[test]
    fn test_update_reporter() {
        let mut context = VMContextBuilder::new();
        let account_id: AccountId = get_account_id("alice");
        let reporter_id: AccountId = get_account_id("reporter");
        let mut contract = Proxy::new(account_id);
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        contract.create_reporter(reporter_id.clone(), 1);
        assert!(
            contract.update_reporter(reporter_id.clone(), 2),
            "Reporter update failed"
        );

        assert_eq!(
            contract.get_reporter(reporter_id.clone()),
            2,
            "reporter value is: {}",
            contract.get_reporter(reporter_id)
        );
    }

    #[test]
    fn test_get_address() {
        let mut context = VMContextBuilder::new();
        let account_id: AccountId = get_account_id("alice");
        let address_id: AccountId = get_account_id("mining.pool");
        let mut contract = Proxy::new(account_id.clone());
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        contract.create_reporter(account_id, 2);
        contract.create_address(address_id.clone(), Category::MiningPool, 7);
        assert_eq!(
            contract.get_address(address_id),
            (Category::MiningPool, 7),
            "Address not writed normally"
        );
    }
}
