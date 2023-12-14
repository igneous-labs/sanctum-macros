#![doc = include_str!("../README.md")]

use create_with_seed_args::CreateWithSeedArgs;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod create_with_seed_args;
mod declare_program_keys_args;
mod utils;

use declare_program_keys_args::{static_pda_gen, DeclareProgramKeysArgs};
use utils::{gen_pubkey_consts, gen_pubkey_consts_with_litstr};

// All #[proc_macro] s must reside at root of crate.
// Signature is (input: proc_macro::TokenStream) -> proc_macro::TokenStream
// NOT proc_macro2

#[proc_macro]
pub fn declare_program_keys(input: TokenStream) -> TokenStream {
    let DeclareProgramKeysArgs {
        prog_id_lit_str,
        prog_id,
        static_pdas,
    } = parse_macro_input!(input);
    // everything below must be infallible
    let id_consts = gen_pubkey_consts_with_litstr("", prog_id, prog_id_lit_str);
    let mut res = quote! {
        #id_consts
    };
    for static_pda in static_pdas {
        res.extend(static_pda_gen(&static_pda));
    }
    res.into()
}

#[proc_macro]
pub fn create_with_seed(input: TokenStream) -> TokenStream {
    let CreateWithSeedArgs {
        pubkey,
        base_lit_str,
        base,
        seed_lit_str,
        owner_lit_str,
        owner,
    } = parse_macro_input!(input);

    let id_consts = gen_pubkey_consts("", pubkey);
    let base_consts = gen_pubkey_consts_with_litstr("BASE", base, base_lit_str);
    let seed_const_ident = format_ident!("SEED");
    let owner_consts = gen_pubkey_consts_with_litstr("OWNER", owner, owner_lit_str);

    quote! {
        #id_consts
        #base_consts
        pub const #seed_const_ident: &str = #seed_lit_str;
        #owner_consts
    }
    .into()
}
