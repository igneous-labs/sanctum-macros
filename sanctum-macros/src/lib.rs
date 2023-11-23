use heck::ToShoutySnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitByte, LitStr};

mod declare_program_keys_args;

use declare_program_keys_args::{DeclareProgramKeysArgs, StaticPdaComputed};

// All #[proc_macro] s must reside at root of crate.
// Signature is (input: proc_macro::TokenStream) -> proc_macro::TokenStream
// NOT proc_macro2

#[proc_macro]
pub fn declare_program_keys(input: TokenStream) -> TokenStream {
    let DeclareProgramKeysArgs {
        prog_id_str_lit,
        prog_id,
        static_pdas,
    } = parse_macro_input!(input);
    // everything below must be infallible

    let prog_id_bytes = prog_id.to_bytes();
    let prog_id_lit_bytes = prog_id_bytes
        .iter()
        .map(|b| LitByte::new(*b, Span::call_site()));

    let mut res = quote! {
        pub const ID_STR: &str = #prog_id_str_lit;
        pub const ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array([#(#prog_id_lit_bytes,)*]);
    };
    for static_pda in static_pdas {
        res.extend(static_pda_gen(&static_pda));
    }
    res.into()
}

fn static_pda_gen(
    StaticPdaComputed {
        name,
        seed,
        bump,
        pubkey,
    }: &StaticPdaComputed,
) -> proc_macro2::TokenStream {
    let name_prefix = name.to_shouty_snake_case();

    let id_ident = format_ident!("{name_prefix}_ID");
    let id_str_ident = format_ident!("{name_prefix}_ID_STR");
    let seed_ident = format_ident!("{name_prefix}_SEED");
    let bump_ident = format_ident!("{name_prefix}_BUMP");

    let id_bytes = pubkey.to_bytes();
    let id_lit_bytes = id_bytes.iter().map(|b| LitByte::new(*b, Span::call_site()));
    let bump_lit = LitByte::new(*bump, Span::call_site());
    let id_str_lit = LitStr::new(&pubkey.to_string(), Span::call_site());

    quote! {
        pub const #seed_ident: &[u8] = #seed;
        pub const #bump_ident: u8 = #bump_lit;
        pub const #id_ident: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array([#(#id_lit_bytes,)*]);
        pub const #id_str_ident: &str = #id_str_lit;
    }
}
