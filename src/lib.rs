use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, ext_contract, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
//use near_sdk::env::{account_balance, current_account_id};

const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000; // 1 $NEAR as yoctoNEAR

/*******************************/
/*********** STRUCTS ***********/
/*******************************/
pub type TransactionId = u64;
pub type Price = u128;
pub type TokenId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
}

/// Helper structure for keys of the persistent collections.
/*
    Since all data stored on the blockchain is kept in a single key-value store under the contract account, 
    you must always use a unique storage prefix for different collections to avoid data collision. It is used in the initialization funct.
    https://near.github.io/near-sdk-as/classes/_sdk_core_assembly_collections_persistentmap_.persistentmap.html#constructor
*/
#[derive(BorshSerialize)]
pub enum StorageKey {
    TransactionsPerAccount,
    TransactionById,
    TransactionMetadataById,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)] //why do we need serialize and deserialize?
#[serde(crate = "near_sdk::serde")]
pub struct Transaction {
    //transaction ID
    pub transaction_id: TransactionId,
    //transaction creator ID
    pub creator_id: AccountId,
    //transaction seller ID
    pub seller_id: AccountId,
    //transaction buyer ID
    pub buyer_id: AccountId,
    //transaction price
    pub price: Price,
    //token ID
    pub nft_id: TokenId,
    //token's contract ID
    pub nft_contract_id: AccountId,
    //price amount is in the contract custody or not
    pub amount_in_escrow: bool,
    //token is in the contract custody or not
    pub token_in_escrow: bool,
    //transaction is completed or not
    pub transaction_status: TransactionStatus,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransactionMetadata {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub price: u128,
    pub categories: String
}

/*******************************/
/*********** CONTRACT **********/
/*******************************/

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] // panic on default to ensure that all fields are initialized
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the transactions IDs for a given account
    pub transactions_per_account: LookupMap<AccountId, UnorderedSet<TransactionId>>,

    //keeps track of the transaction struct for a given transaction ID
    pub transaction_by_id: LookupMap<TransactionId, Transaction>,

    //keeps track of the transaction metadata for a given transaction ID [info that doesnt change during transaction]
    pub transaction_metadata_by_id: UnorderedMap<TransactionId, TransactionMetadata>,
}

/*******************************/
/******* INITIALIZATION ********/
/*******************************/

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        sets the contract owner
    */    
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            transactions_per_account: LookupMap::new(StorageKey::TransactionsPerAccount.try_to_vec().unwrap()),
            transaction_by_id: LookupMap::new(StorageKey::TransactionById.try_to_vec().unwrap()),
            transaction_metadata_by_id: UnorderedMap::new(
                StorageKey::TransactionMetadataById.try_to_vec().unwrap(),
            ),
        };
        //return the Contract object
        this
    }
}

/*******************************/
/****** CONTRACT METHODS *******/
/*******************************/

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