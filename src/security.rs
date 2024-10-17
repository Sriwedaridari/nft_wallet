use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn verify_signature(message: &str, signature: &str, public_key: &str) -> bool {
    // Dummy verification logic, replace with actual signature verification
    let hashed_message = hash_message(message);
    hashed_message == signature && public_key == "valid_public_key"
}

pub fn hash_message(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn generate_nonce() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_epoch.as_secs()
}
