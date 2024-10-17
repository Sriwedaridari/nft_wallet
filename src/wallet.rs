use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref WALLET_BALANCES: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

pub fn get_balance(owner: String) -> u64 {
    let balances = WALLET_BALANCES.lock().unwrap();
    *balances.get(&owner).unwrap_or(&0)
}

pub fn increase_balance(owner: String, amount: u64) {
    let mut balances = WALLET_BALANCES.lock().unwrap();
    let balance = balances.entry(owner).or_insert(0);
    *balance += amount;
}

pub fn decrease_balance(owner: String, amount: u64) -> Result<(), String> {
    let mut balances = WALLET_BALANCES.lock().unwrap();
    if let Some(balance) = balances.get_mut(&owner) {
        if *balance >= amount {
            *balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    } else {
        Err("Owner not found".to_string())
    }
}
