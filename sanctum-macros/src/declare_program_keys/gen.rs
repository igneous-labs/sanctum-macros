use heck::ToShoutySnakeCase;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{LitByte, LitByteStr};

use crate::utils::gen_pubkey_consts;

use super::StaticPdaComputed;

pub fn static_pda_gen(
    StaticPdaComputed {
        name,
        seeds,
        bump,
        pubkey,
    }: &StaticPdaComputed,
) -> proc_macro2::TokenStream {
    let name_prefix = name.to_shouty_snake_case();
    let bump_ident = format_ident!("{name_prefix}_BUMP");

    let bump_lit = LitByte::new(*bump, Span::call_site());
    let pubkey_consts = gen_pubkey_consts(name, *pubkey);

    let seeds = seeds_gen(&name_prefix, seeds);

    quote! {
        pub const #bump_ident: u8 = #bump_lit;
        #seeds
        #pubkey_consts
    }
}

pub fn seeds_gen(name_prefix: &str, seeds: &[LitByteStr]) -> proc_macro2::TokenStream {
    if seeds.len() == 1 {
        let seed = &seeds[0];
        let seed_ident = format_ident!("{name_prefix}_SEED");
        return quote! {
            pub const #seed_ident: &[u8] = #seed;
        };
    }
    let mut res = quote! {};
    for (i, seed) in seeds.iter().enumerate() {
        let seed_ident = format_ident!("{name_prefix}_SEED_{i}");
        res.extend(quote! {
            pub const #seed_ident: &[u8] = #seed;
        });
    }
    res
}
