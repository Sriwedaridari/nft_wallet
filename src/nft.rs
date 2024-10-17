use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Serialize};

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct NFT {
    pub token_id: String,
    pub owner: String,
    pub metadata: String,
}

lazy_static! {
    static ref NFT_STORE: Mutex<HashMap<String, NFT>> = Mutex::new(HashMap::new());
}

pub fn mint_nft(owner: String, metadata: String) -> String {
    let token_id = generate_token_id();
    let nft = NFT {
        token_id: token_id.clone(),
        owner,
        metadata,
    };
    NFT_STORE.lock().unwrap().insert(token_id.clone(), nft);
    token_id
}

pub fn transfer(from: String, to: String, token_id: String) -> Result<(), String> {
    let mut store = NFT_STORE.lock().unwrap();
    if let Some(nft) = store.get_mut(&token_id) {
        if nft.owner == from {
            nft.owner = to;
            Ok(())
        } else {
            Err("Not the owner of the NFT".to_string())
        }
    } else {
        Err("NFT not found".to_string())
    }
}

fn generate_token_id() -> String {
    use rand::Rng;
    let rand_id: u64 = rand::thread_rng().gen();
    format!("{}", rand_id)
}
￼Enter
