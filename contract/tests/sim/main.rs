extern crate proxy_contract;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::AccountId;
use near_sdk::MockedBlockchain;
use proxy_contract::Proxy;

#[test]
#[should_panic(expected = "HapiProxy: Only the owner may call this method")]
fn test_change_owner() {
    let mut context = VMContextBuilder::new();
    let test_level: u8 = 1;
    let account_id: AccountId = "alice".to_string();
    let second_account_id: AccountId = "james.bond".to_string();
    let reporter_id: AccountId = "reporter".to_string();
    let mut contract = Proxy::new(account_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.change_owner(second_account_id);
    contract.create_reporter(reporter_id.clone(), test_level);
    assert_eq!(
        contract.get_reporter(reporter_id.clone()),
        test_level,
        "reporter value is: {}",
        contract.get_reporter(reporter_id).to_string()
    );
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
        contract.get_reporter(reporter_id.clone()),
        test_level,
        "reporter value is: {}",
        contract.get_reporter(reporter_id).to_string()
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
        contract.get_reporter(reporter_id.clone()),
        2,
        "reporter value is: {}",
        contract.get_reporter(reporter_id).to_string()
    );
}

#[test]
#[should_panic(expected = "HapiProxy: Only the owner may call this method")]
fn test_not_owner_updates_reporter() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = "alice".to_string();
    let reporter_id: AccountId = "reporter".to_string();
    let mut contract = Proxy::new(account_id.clone());
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
        contract.get_reporter(reporter_id).to_string()
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
    contract.create_address(address_id.clone(), proxy_contract::Category::MiningPool, 7);
    assert_eq!(
        contract.get_address(address_id),
        (proxy_contract::Category::MiningPool, 7),
        "Address not writed normally"
    );
}

#[test]
#[should_panic(expected = "HapiProxy: Invalid permission level")]
fn test_invalid_permission_level() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = "alice".to_string();
    let address_id: AccountId = "MiningPool".to_string();
    let mut contract = Proxy::new(account_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(account_id.clone(), 1);
    contract.create_address(address_id.clone(), proxy_contract::Category::MiningPool, 7);
    contract.get_address(address_id);
}

#[test]
#[should_panic(expected = "HapiProxy: Address does not exist")]
fn test_get_wrong_address() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = "alice".to_string();
    let address_id: AccountId = "MiningPool".to_string();
    let wrong_address: AccountId = "wrong_address".to_string();
    let mut contract = Proxy::new(account_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(account_id.clone(), 2);
    contract.create_address(address_id.clone(), proxy_contract::Category::MiningPool, 7);
    contract.get_address(wrong_address.clone());
}

#[test]
#[should_panic(expected = "HapiProxy: Invalid risk")]
fn test_create_address_wrong_risk() {
    let mut context = VMContextBuilder::new();
    let account_id: AccountId = "alice".to_string();
    let address_id: AccountId = "MiningPool".to_string();
    let mut contract = Proxy::new(account_id.clone());
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.create_reporter(account_id.clone(), 2);
    contract.create_address(address_id.clone(), proxy_contract::Category::MiningPool, 11);
}
