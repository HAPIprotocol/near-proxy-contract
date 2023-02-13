extern crate proxy_contract;
use std::str::FromStr;

use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::AccountId;
use proxy_contract::{Category, Proxy};

pub fn get_account_id(account_id: &str) -> AccountId {
    AccountId::from_str(account_id)
        .unwrap_or_else(|_| panic!("ERR: can't get account_id from str: {account_id}"))
}

#[test]
#[should_panic(expected = "HapiProxy: Only the owner may call this method")]
fn test_change_owner() {
    let mut context = VMContextBuilder::new();
    let test_level: u8 = 1;
    let account_id: AccountId = get_account_id("alice");
    let second_account_id: AccountId = get_account_id("james.bond");
    let reporter_id: AccountId = get_account_id("reporter");
    let mut contract = Proxy::new(account_id);
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.change_owner(second_account_id);
    contract.create_reporter(reporter_id.clone(), test_level);
    assert_eq!(
        contract.get_reporter(reporter_id.clone()),
        test_level,
        "reporter value is: {}",
        contract.get_reporter(reporter_id)
    );
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
#[should_panic(expected = "HapiProxy: Reporter already exist")]
fn test_twice_create_reporter() {
    let mut context = VMContextBuilder::new();
    let test_level: u8 = 1;
    let account_id: AccountId = get_account_id("alice");
    let reporter_id: AccountId = get_account_id("reporter");
    let mut contract = Proxy::new(account_id);
    testing_env!(context.predecessor_account_id(accounts(0)).build());

    contract.create_reporter(reporter_id.clone(), test_level);
    contract.create_reporter(reporter_id, test_level);
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
#[should_panic(expected = "HapiProxy: Only the owner may call this method")]
fn test_not_owner_updates_reporter() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = get_account_id("alice");
    let reporter_id: AccountId = get_account_id("reporter");
    let mut contract = Proxy::new(account_id);
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(reporter_id.clone(), 1);
    testing_env!(context.predecessor_account_id(accounts(1)).build());
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

#[test]
#[should_panic(expected = "HapiProxy: Invalid permission level")]
fn test_invalid_permission_level() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = get_account_id("alice");
    let address_id: AccountId = get_account_id("mining.pool");
    let mut contract = Proxy::new(account_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(account_id, 1);
    contract.create_address(address_id.clone(), Category::MiningPool, 7);
    contract.get_address(address_id);
}

#[test]
#[should_panic(expected = "HapiProxy: Invalid risk")]
fn test_create_address_wrong_risk() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = get_account_id("alice");
    let address_id: AccountId = get_account_id("mining.pool");
    let mut contract = Proxy::new(account_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(account_id, 2);
    contract.create_address(address_id, Category::MiningPool, 11);
}
