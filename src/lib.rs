use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, post_upgrade, query, update};
use std::collections::HashMap;
use serde::{Serialize};

mod nft;
mod wallet;
mod security;

#[init]
fn init() {
    ic_cdk::print("NFT Wallet Initialized!");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::print("Upgraded successfully!");
}

#[query]
fn wallet_balance(owner: String) -> u64 {
    wallet::get_balance(owner)
}

#[update]
fn transfer_nft(from: String, to: String, token_id: String) -> Result<(), String> {
    nft::transfer(from, to, token_id)
}
