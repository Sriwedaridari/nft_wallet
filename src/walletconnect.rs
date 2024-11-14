use ic_agent::{Agent, ic_types::Principal};
use ic_utils::call::call;
use candid::{Encode, Decode, CandidType};
use std::error::Error;

// Struktur data untuk menangkap respon dari panggilan
#[derive(CandidType, Debug)]
struct BalanceResponse {
    amount: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Buat agen baru yang terhubung ke jaringan IC
    let agent = Agent::builder()
        .with_url("https://ic0.app") // URL IC mainnet
        .build()?;

    // Hubungkan ke dompet menggunakan principal ID
    let wallet_id = "your-wallet-canister-id"; // Ganti dengan ID canister dompet kamu
    let wallet_principal = Principal::from_text(wallet_id)?;

    // Contoh memanggil metode saldo (balance) pada dompet
    let result: Result<(BalanceResponse,), _> = call(&agent, &wallet_principal, "balance")
        .with_arg(&Encode!()?) // Argumen dapat disesuaikan jika diperlukan
        .call()
        .await;

    match result {
        Ok((balance_response,)) => {
            println!("Saldo: {} ICP", balance_response.amount);
        }
        Err(e) => {
            println!("Gagal mendapatkan saldo: {:?}", e);
        }
    }

    Ok(())
} 
