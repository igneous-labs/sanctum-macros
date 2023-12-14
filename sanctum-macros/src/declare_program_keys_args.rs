use heck::ToShoutySnakeCase;
use proc_macro2::Span;
use quote::{format_ident, quote};
use solana_program::pubkey::Pubkey;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Expr, ExprLit, ExprTuple, Lit, LitByte, LitByteStr, LitStr, Token,
};

use crate::utils::{gen_pubkey_consts, pubkey_lit_str};

pub struct DeclareProgramKeysArgs {
    pub prog_id_lit_str: LitStr,
    pub prog_id: Pubkey,
    pub static_pdas: Vec<StaticPdaComputed>,
}

impl Parse for DeclareProgramKeysArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prog_id_lit_str: LitStr = input.parse()?;
        let (prog_id, prog_id_lit_str) = pubkey_lit_str(prog_id_lit_str)?;

        input.parse::<Token![,]>()?;
        let StaticPdaList(entries) = input.parse()?;

        let static_pdas = entries
            .into_iter()
            .map(|s| StaticPdaComputed::compute(&prog_id, s))
            .collect();

        Ok(Self {
            prog_id_lit_str,
            prog_id,
            static_pdas,
        })
    }
}

pub struct StaticPdaComputed {
    pub name: String,
    pub seed: LitByteStr,
    pub bump: u8,
    pub pubkey: Pubkey,
}

impl StaticPdaComputed {
    pub fn compute(prog_id: &Pubkey, StaticPda { name, seed }: StaticPda) -> Self {
        let (pubkey, bump) = Pubkey::find_program_address(&[seed.value().as_ref()], prog_id);
        Self {
            name,
            seed,
            bump,
            pubkey,
        }
    }
}

pub struct StaticPda {
    pub name: String,
    pub seed: LitByteStr,
}

pub struct StaticPdaList(pub Vec<StaticPda>);

impl Parse for StaticPdaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let list: Punctuated<ExprTuple, Comma> = Punctuated::parse_terminated(&content)?;
        let res: syn::Result<Vec<StaticPda>> = list
            .iter()
            .map(|tup| {
                let mut expr_iter = tup.elems.iter();

                let name_expr = expr_iter.next().ok_or_else(|| {
                    syn::Error::new(tup.elems.span(), "missing name in (\"name\", b\"seed\")")
                })?;
                let name_expr_lit = if let Expr::Lit(ExprLit { lit, .. }) = name_expr {
                    lit
                } else {
                    return Err(syn::Error::new(name_expr.span(), "name not string literal"));
                };
                let name = if let Lit::Str(ls) = name_expr_lit {
                    ls.value()
                } else {
                    return Err(syn::Error::new(name_expr.span(), "name not string literal"));
                };
                if name.is_empty() {
                    return Err(syn::Error::new(name_expr.span(), "name cannot be empty"));
                }

                let seed_expr = expr_iter.next().ok_or_else(|| {
                    syn::Error::new(tup.elems.span(), "missing seed in (\"name\", b\"seed\")")
                })?;
                let seed_expr_lit = if let Expr::Lit(ExprLit { lit, .. }) = seed_expr {
                    lit
                } else {
                    return Err(syn::Error::new(
                        seed_expr.span(),
                        "seed not bytestring literal",
                    ));
                };
                let seed = if let Lit::ByteStr(bs) = seed_expr_lit {
                    bs.clone()
                } else {
                    return Err(syn::Error::new(
                        seed_expr.span(),
                        "seed not bytestring literal",
                    ));
                };

                Ok(StaticPda { name, seed })
            })
            .collect();
        Ok(Self(res?))
    }
}

pub fn static_pda_gen(
    StaticPdaComputed {
        name,
        seed,
        bump,
        pubkey,
    }: &StaticPdaComputed,
) -> proc_macro2::TokenStream {
    let name_prefix = name.to_shouty_snake_case();
    let seed_ident = format_ident!("{name_prefix}_SEED");
    let bump_ident = format_ident!("{name_prefix}_BUMP");

    let bump_lit = LitByte::new(*bump, Span::call_site());
    let pubkey_consts = gen_pubkey_consts(name, *pubkey);

    quote! {
        pub const #seed_ident: &[u8] = #seed;
        pub const #bump_ident: u8 = #bump_lit;
        #pubkey_consts
    }
}
