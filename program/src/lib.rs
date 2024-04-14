
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, Clone)]
pub struct TwoPoolArgs {
    pub bank: u8,
    pub p0: PoolTypes,
    pub p1: PoolTypes
}

#[derive(BorshDeserialize, Clone)]
pub enum PoolTypes {
    Orca(OrcaPoolMap),
    Raydium(RaydiumPoolMap)
}

#[derive(BorshDeserialize, Clone)]
pub struct OrcaPoolMap {
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

#[derive(BorshDeserialize, Clone)]
pub struct RaydiumPoolMap {
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub g: u8,
    pub h: u8,
}
entrypoint!(process_instruction);



pub fn process_instruction(
    _program_id: &Pubkey, 
    accounts_info: &[AccountInfo], 
    instruction_data: &[u8], 
) -> ProgramResult {
    msg!("Entrypoint run");

    let args = TwoPoolArgs::try_from_slice(instruction_data)?;

    match args.p0 {
        PoolTypes::Orca(p0) => {
            let a = accounts_info.get(p0.a as usize).unwrap();
            let b = accounts_info.get(p0.b as usize).unwrap();
            let c = accounts_info.get(p0.c as usize).unwrap();

            msg!("p0 orca a: {} b: {} c: {}",a.key, b.key, c.key);

            let balance_a = a.lamports();
            let balance_b = b.lamports();
            let balance_c = c.lamports();
            msg!("{}, {}, {}", balance_a, balance_b, balance_c);
        },
        PoolTypes::Raydium(p0) => {
            let d = accounts_info.get(p0.d as usize).unwrap().key;
            let e = accounts_info.get(p0.e as usize).unwrap().key;
            let f = accounts_info.get(p0.f as usize).unwrap().key;
            let g = accounts_info.get(p0.g as usize).unwrap().key;
            let h = accounts_info.get(p0.h as usize).unwrap().key;
            msg!("p0 raydium d: {} e: {} f: {} g: {} h: {}", d, e, f, g, h);
        }
    }

    match args.p1 {
        PoolTypes::Orca(p1) => {
            let a = accounts_info.get(p1.a as usize).unwrap().key;
            let b = accounts_info.get(p1.b as usize).unwrap().key;
            let c = accounts_info.get(p1.c as usize).unwrap().key;
            msg!("p1 orca a: {} b: {} c: {}",a, b, c);
        },
        PoolTypes::Raydium(p1) => {
            let d = accounts_info.get(p1.d as usize).unwrap().key;
            let e = accounts_info.get(p1.e as usize).unwrap().key;
            let f = accounts_info.get(p1.f as usize).unwrap().key;
            let g = accounts_info.get(p1.g as usize).unwrap().key;
            let h = accounts_info.get(p1.h as usize).unwrap().key;
            msg!("p1 raydium d: {} e: {} f: {} g: {} h: {}", d, e, f, g, h);
        }
    }

    Ok(())
}