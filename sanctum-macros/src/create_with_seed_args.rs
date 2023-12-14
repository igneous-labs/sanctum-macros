use proc_macro2::Span;
use solana_program::pubkey::Pubkey;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    LitStr,
};

use crate::utils::pubkey_lit_str;

pub struct CreateWithSeedArgs {
    pub pubkey: Pubkey,
    pub base_lit_str: LitStr,
    pub base: Pubkey,
    pub seed_lit_str: LitStr,
    pub owner_lit_str: LitStr,
    pub owner: Pubkey,
}

impl Parse for CreateWithSeedArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list: Punctuated<LitStr, Comma> = Punctuated::parse_terminated(input)?;
        let mut list_iter = list.into_iter();

        let base_lit_str = list_iter
            .next()
            .ok_or(missing_arg_err(input.span(), "base"))?;
        let (base, base_lit_str) = pubkey_lit_str(base_lit_str)?;

        let seed_lit_str: LitStr = list_iter
            .next()
            .ok_or(missing_arg_err(input.span(), "seed"))?;

        let owner_lit_str: LitStr = list_iter
            .next()
            .ok_or(missing_arg_err(input.span(), "owner"))?;
        let (owner, owner_lit_str) = pubkey_lit_str(owner_lit_str)?;

        let pubkey =
            Pubkey::create_with_seed(&base, &seed_lit_str.value(), &owner).map_err(|e| {
                syn::Error::new(input.span(), format!("Could not create with seed: {e}"))
            })?;

        Ok(Self {
            pubkey,
            base_lit_str,
            base,
            seed_lit_str,
            owner_lit_str,
            owner,
        })
    }
}

fn missing_arg_err(span: Span, arg_name: &str) -> syn::Error {
    syn::Error::new(
        span,
        format!("Missing {arg_name} in create_with_seed!(<base>, <seed>, <owner>)"),
    )
}
