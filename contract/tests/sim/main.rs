extern crate proxy_contract;
use std::str::FromStr;

use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::AccountId;
use proxy_contract::{Category, Proxy, ROLES};

pub fn get_account_id(account_id: &str) -> AccountId {
    AccountId::from_str(account_id)
        .unwrap_or_else(|_| panic!("ERR: can't get account_id from str: {account_id}"))
}

#[test]
#[should_panic(expected = "HapiProxy: Only the owner or authority may call this method")]
fn test_change_owner() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("alice");
    let second_account_id: AccountId = get_account_id("james.bond");
    let reporter_id: AccountId = get_account_id("reporter");
    let mut contract = Proxy::new(owner_id.clone());
    testing_env!(context.predecessor_account_id(owner_id).build());

    contract.change_owner(second_account_id);
    contract.create_reporter(reporter_id.clone(), ROLES::REPORTER as u8);
}

#[test]
fn test_get_reporter() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");
    let mut contract = Proxy::new(owner_id.clone());
    testing_env!(context.predecessor_account_id(owner_id).build());

    contract.create_reporter(reporter_id.clone(), ROLES::REPORTER as u8);
    assert_eq!(
        contract.get_reporter(reporter_id.clone()),
        ROLES::REPORTER as u8,
        "reporter value is: {}",
        contract.get_reporter(reporter_id)
    );
}

#[test]
#[should_panic(expected = "HapiProxy: Reporter already exist")]
fn test_twice_create_reporter() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");
    let mut contract = Proxy::new(owner_id.clone());
    testing_env!(context.predecessor_account_id(owner_id).build());

    contract.create_reporter(reporter_id.clone(), ROLES::REPORTER as u8);
    contract.create_reporter(reporter_id, ROLES::REPORTER as u8);
}

#[test]
fn test_update_reporter() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");
    testing_env!(context.predecessor_account_id(owner_id.clone()).build());

    let mut contract = Proxy::new(owner_id);
    contract.create_reporter(reporter_id.clone(), ROLES::REPORTER as u8);
    assert!(
        contract.update_reporter(reporter_id.clone(), ROLES::AUTHORITY as u8),
        "Reporter update failed"
    );

    assert_eq!(
        contract.get_reporter(reporter_id.clone()),
        ROLES::AUTHORITY as u8,
        "reporter value is: {}",
        contract.get_reporter(reporter_id)
    );
}

#[test]
#[should_panic(expected = "HapiProxy: Only the owner or authority may call this method")]
fn test_not_owner_updates_reporter() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");
    testing_env!(context.predecessor_account_id(owner_id.clone()).build());

    let mut contract = Proxy::new(owner_id);
    contract.create_reporter(reporter_id.clone(), ROLES::REPORTER as u8);

    testing_env!(context.predecessor_account_id(reporter_id.clone()).build());
    assert!(
        contract.update_reporter(reporter_id.clone(), ROLES::AUTHORITY as u8),
        "Reporter update failed"
    );

    assert_eq!(
        contract.get_reporter(reporter_id.clone()),
        ROLES::REPORTER as u8,
        "reporter value is: {}",
        contract.get_reporter(reporter_id)
    );
}

#[test]
fn test_get_address() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");
    let address_id: AccountId = get_account_id("mining.pool");
    testing_env!(context.predecessor_account_id(owner_id.clone()).build());

    let mut contract = Proxy::new(owner_id);
    contract.create_reporter(reporter_id.clone(), ROLES::REPORTER as u8);

    testing_env!(context.predecessor_account_id(reporter_id).build());
    contract.create_address(address_id.clone(), Category::MiningPool, 7);
    assert_eq!(
        contract.get_address(address_id),
        (Category::MiningPool, 7),
        "Address not added"
    );
}

#[test]
#[should_panic(expected = "HapiProxy: Invalid permission level")]
fn test_invalid_permission_level() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");

    let mut contract = Proxy::new(owner_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(reporter_id, 3);
}

#[test]
#[should_panic(expected = "HapiProxy: Invalid risk")]
fn test_create_address_wrong_risk() {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = get_account_id("owner");
    let reporter_id: AccountId = get_account_id("reporter");
    let address_id: AccountId = get_account_id("mining.pool");
    let mut contract = Proxy::new(owner_id.clone());
    testing_env!(context.predecessor_account_id(owner_id).build());
    contract.create_reporter(reporter_id.clone(), ROLES::AUTHORITY as u8);

    testing_env!(context.predecessor_account_id(reporter_id).build());
    contract.create_address(address_id, Category::MiningPool, 11);
}
