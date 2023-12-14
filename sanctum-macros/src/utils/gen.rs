use heck::ToShoutySnakeCase;
use proc_macro2::Span;
use quote::{format_ident, quote};
use solana_program::pubkey::Pubkey;
use syn::{LitByte, LitStr};

pub fn gen_pubkey_consts(name: &str, pubkey: Pubkey) -> proc_macro2::TokenStream {
    let b58_lit_str = LitStr::new(&pubkey.to_string(), Span::call_site());
    gen_pubkey_consts_with_litstr(name, pubkey, b58_lit_str)
}

pub fn gen_pubkey_consts_with_litstr(
    name: &str,
    pubkey: Pubkey,
    b58_lit_str: LitStr,
) -> proc_macro2::TokenStream {
    let (id_bytes_ident, id_ident, id_str_ident) = if name.is_empty() {
        (
            format_ident!("ID_BYTES"),
            format_ident!("ID"),
            format_ident!("ID_STR"),
        )
    } else {
        let name_prefix = name.to_shouty_snake_case();
        (
            format_ident!("{name_prefix}_ID_BYTES"),
            format_ident!("{name_prefix}_ID"),
            format_ident!("{name_prefix}_ID_STR"),
        )
    };

    let id_bytes = pubkey.to_bytes();
    let id_lit_bytes = id_bytes.iter().map(|b| LitByte::new(*b, Span::call_site()));

    quote! {
        pub const #id_bytes_ident: [u8; 32] = [#(#id_lit_bytes,)*];
        pub const #id_ident: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(#id_bytes_ident);
        pub const #id_str_ident: &str = #b58_lit_str;
    }
}
