// pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
// use near_sdk::serde_json::json;
// use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount};
// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::{env, near_bindgen, AccountId};
// use proxy_contract::ProxyContract;

// extern crate proxy_contract;
// near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
//     COUNTER_BYTES => "../out/main.wasm",
// }

// pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

// fn init() -> (UserAccount, ContractAccount<CounterContract>) {
//     let contract = new("tester");
//     assert_eq!(
//         "tester", self.owner_id,
//         "Only the owner may call this method"
//     );
//     // Deploy the compiled Wasm bytes
//     let counter: ContractAccount<CounterContract> = deploy!(
//         contract: CounterContract,
//         contract_id: "counter".to_string(),
//         bytes: &COUNTER_BYTES,
//         signer_account: root
//     );

//     (root, counter)
// }

// #[test]
// fn simulate_add_reporter() {
//     let account_id: AccountId = "tester".to_string();
//     let contract = new(account_id);
//     assert_eq!(
//         "tester", contract.owner_id,
//         "Only the owner may call this method"
//     );
    // Get number on account that hasn't incremented or decremented
    // let mut current_num: i8 = view!(counter.get_num()).unwrap_json();
    // println!("Number before: {}", &current_num);
    // assert_eq!(&current_num, &0, "Initial number should be zero.");

    // // Call the increment function
    // call!(root, counter.increment()).assert_success();

    // current_num = view!(counter.get_num()).unwrap_json();
    // println!("Number after first increment: {}", &current_num);
    // assert_eq!(
    //     &current_num, &1,
    //     "After incrementing, the number should be one."
    // );

    // // Now use the non-macro approach to increment the number.
    // root.call(
    //     counter.account_id(),
    //     "increment",
    //     &json!({}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // attached deposit
    // )
    // .assert_success();

    // // Similarly, use the non-macro approach to check the value.
    // current_num = root
    //     .view(
    //         counter.account_id(),
    //         "get_num",
    //         &json!({}).to_string().into_bytes(),
    //     )
    //     .unwrap_json();
    // println!("Number after second increment: {}", &current_num);
    // assert_eq!(
    //     &current_num, &2,
    //     "After incrementing twice, the number should be two."
    // );
// }