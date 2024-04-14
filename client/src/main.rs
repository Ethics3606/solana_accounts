pub mod utils;

use borsh::BorshSerialize;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey};
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use utils::{load_dev_wallet, create_http_client};

#[derive(BorshSerialize, Clone)]
pub struct TwoPoolArgs {
    pub bank: u8,
    pub p0: PoolTypes,
    pub p1: PoolTypes
}

#[derive(BorshSerialize, Clone)]
pub enum PoolTypes {
    Orca(OrcaPoolMap),
    Raydium(RaydiumPoolMap)
}

#[derive(Clone, Debug)]
pub struct OrcaPool {
    pub a: Pubkey,
    pub b: Pubkey,
    pub c: Pubkey,
}

#[derive(BorshSerialize, Clone, Debug)]
pub struct OrcaPoolMap {
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

#[derive(Clone, Debug)]
pub struct RaydiumPool {
    pub d: Pubkey,
    pub e: Pubkey,
    pub f: Pubkey,
    pub g: Pubkey,
    pub h: Pubkey,
}

#[derive(BorshSerialize, Clone, Debug)]
pub struct RaydiumPoolMap {
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub g: u8,
    pub h: u8,
}


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let program_id: Pubkey = "program address here".parse().unwrap(); 
    let conn = create_http_client("http://127.0.0.1:8899"); // run solana-test-validator

    let wallet = load_dev_wallet();


    let pool_a = OrcaPool {
        a: "11111115RidqCHAoz6dzmXxGcfWLNzevYqNpaRAUo".parse().unwrap(),
        b: "1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM".parse().unwrap(),
        c: "11111112cMQwSC9qirWGjZM6gLGwW69X22mqwLLGP".parse().unwrap(),
    };

    dbg!(&pool_a);

    let pool_b = RaydiumPool {
        d: "11111112D1oxKts8YPdTJRG5FzxTNpMtWmq8hkVx3".parse().unwrap(),
        e: "111111131h1vYVSYuKP6AhS86fbRdMw9XHiZAvAaj".parse().unwrap(),
        f: "11111113R2cuenjG5nFubqX9Wzuukdin2YfGQVzu5".parse().unwrap(),
        g: "11111113pNDtm61yGF8j2ycAwLEPsuWQXobye5qDR".parse().unwrap(),
        h: "11111114DhpssPJgSi1YU7hCMfYt1BJ334YgsffXm".parse().unwrap(),
    };
    dbg!(&pool_b);

    let mut accounts: Vec<AccountMeta> = vec![];
    
    accounts.push(AccountMeta::new(pool_a.a, false));
    accounts.push(AccountMeta::new(pool_a.b, false));
    accounts.push(AccountMeta::new(pool_a.c, false));

    accounts.push(AccountMeta::new(pool_b.d, false));
    accounts.push(AccountMeta::new(pool_b.e, false));
    accounts.push(AccountMeta::new(pool_b.f, false));
    accounts.push(AccountMeta::new(pool_b.g, false));
    accounts.push(AccountMeta::new(pool_b.h, false));
    accounts.push(AccountMeta::new_readonly(wallet.pubkey(), true));

    let args = TwoPoolArgs {
        bank: 8,
        p0: PoolTypes::Orca(OrcaPoolMap { a: 0, b: 1, c: 2 }),
        p1: PoolTypes::Raydium(RaydiumPoolMap { d: 3, e: 4, f: 5, g: 6, h: 7 })
    };

    let data = args.try_to_vec().unwrap(); 

    dbg!(data.len()); // 11 bytes

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&wallet.pubkey()),
    );

    let latest_blockhash = conn.get_latest_blockhash().await.unwrap();

    transaction.sign(&[&wallet], latest_blockhash);

    let sim = conn.simulate_transaction(&transaction).await.unwrap();
    dbg!(sim);

    let sig = conn.send_and_confirm_transaction(&transaction).await.unwrap();
    dbg!(sig);
}
