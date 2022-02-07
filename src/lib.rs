use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, ext_contract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{account_balance, current_account_id};

//const INITIAL_BALANCE: Balance = 250_000_000_000_000_000_000_000; // 2.5e23yN, 0.25N
const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000; // 1 $NEAR as yoctoNEAR

//importing another contract as a constant
//const CODE: &[u8] = include_bytes!("../../greeter.wasm");

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[ext_contract(ext_get_balance)]
pub trait Contract {
    fn get_balance(&self) -> Balance;
}

#[near_bindgen]
impl Contract {
    //#[private]
    //#[payable]
    //pub fn create_subaccount(prefix: AccountId) -> Promise {
    //    let subaccount_id: AccountId = format!("{}.{}", prefix, env::current_account_id());
    //    Promise::new(subaccount_id)
    //        .create_account()
            //.deploy_contract(CODE.to_vec())
            //.add_full_access_key(env::signer_account_pk())
            //.transfer(INITIAL_BALANCE)
    //        .transfer(env::attached_deposit())
    //}
        
    pub fn get_balance(&self) -> Balance {
        env::account_balance()
    }

    pub fn pay(&self, receiver_id: AccountId) -> Promise {
        let amount = env::account_balance() - 20*ONE_NEAR;  // should be replace by the amout transfered
        Promise::new(receiver_id).transfer(amount)
    }
}