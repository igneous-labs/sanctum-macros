use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use syn::LitStr;

pub fn pubkey_lit_str(b58_lit_str: LitStr) -> syn::Result<(Pubkey, LitStr)> {
    let pk = Pubkey::from_str(&b58_lit_str.value())
        .map_err(|e| syn::Error::new(b58_lit_str.span(), e))?;
    Ok((pk, b58_lit_str))
}
