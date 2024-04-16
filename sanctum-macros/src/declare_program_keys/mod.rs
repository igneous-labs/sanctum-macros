mod gen;
mod parse;

pub use gen::*;
use solana_program::pubkey::Pubkey;
use syn::{LitByteStr, LitStr};

pub struct DeclareProgramKeysArgs {
    pub prog_id_lit_str: LitStr,
    pub prog_id: Pubkey,
    pub static_pdas: Vec<StaticPdaComputed>,
}

pub struct StaticPdaComputed {
    pub name: String,
    pub seeds: Vec<LitByteStr>,
    pub bump: u8,
    pub pubkey: Pubkey,
}

impl StaticPdaComputed {
    pub fn compute(prog_id: &Pubkey, StaticPda { name, seeds }: StaticPda) -> Self {
        let find_args_vec: Vec<Vec<u8>> = seeds.iter().map(|seed| seed.value()).collect();
        let find_args: Vec<&[u8]> = find_args_vec.iter().map(Vec::as_slice).collect();
        let (pubkey, bump) = Pubkey::find_program_address(&find_args, prog_id);
        Self {
            name,
            seeds,
            bump,
            pubkey,
        }
    }
}

pub struct StaticPda {
    pub name: String,
    pub seeds: Vec<LitByteStr>,
}

pub struct StaticPdaList(pub Vec<StaticPda>);
