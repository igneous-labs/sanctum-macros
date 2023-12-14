use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Expr, ExprLit, ExprTuple, Lit, LitByteStr, LitStr, Token,
};

use crate::utils::pubkey_lit_str;

use super::{DeclareProgramKeysArgs, StaticPda, StaticPdaComputed, StaticPdaList};

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

impl Parse for StaticPdaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let list: Punctuated<ExprTuple, Comma> = Punctuated::parse_terminated(&content)?;
        let res: syn::Result<Vec<StaticPda>> = list.iter().map(parse_static_pda).collect();
        Ok(Self(res?))
    }
}

pub fn parse_static_pda(ExprTuple { elems, .. }: &ExprTuple) -> syn::Result<StaticPda> {
    let mut expr_iter = elems.iter().peekable();

    let name_expr = expr_iter
        .next()
        .ok_or_else(|| syn::Error::new(elems.span(), "missing name in (\"name\", b\"seed\")"))?;
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
    if expr_iter.peek().is_none() {
        return Err(syn::Error::new(
            elems.span(),
            "missing seed in (\"name\", b\"seed\")",
        ));
    }
    let seeds_res: syn::Result<Vec<LitByteStr>> = expr_iter.map(parse_bytestring_seed).collect();
    let seeds = seeds_res?;
    Ok(StaticPda { name, seeds })
}

/// `b"seed"`
pub fn parse_bytestring_seed(seed_expr: &Expr) -> syn::Result<LitByteStr> {
    let seed_expr_lit = if let Expr::Lit(ExprLit { lit, .. }) = seed_expr {
        lit
    } else {
        return Err(syn::Error::new(
            seed_expr.span(),
            "seed not bytestring literal",
        ));
    };
    if let Lit::ByteStr(bs) = seed_expr_lit {
        Ok(bs.clone())
    } else {
        Err(syn::Error::new(
            seed_expr.span(),
            "seed not bytestring literal",
        ))
    }
}
