use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::{parse::Result, Expr, Lit, LitStr};

pub fn expand_to_literal(expr: Expr) -> Result<LitStr> {
    let token_stream: TokenStream = expr.to_token_stream().into();
    let abi_or_path_expr = token_stream.expand_expr().unwrap();

    token_stream_as_lit_str(abi_or_path_expr)
}

fn token_stream_as_lit_str(input: TokenStream) -> Result<LitStr> {
    let input: TokenStream2 = input.into();
    let tokens: Vec<_> = input.into_iter().collect();
    if tokens.len() != 1 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected a single expression",
        ));
    }

    match tokens[0].clone() {
        TokenTree::Literal(lit) => {
            let lit = Lit::new(lit);
            if let Lit::Str(lit) = lit {
                Ok(lit)
            } else {
                Err(syn::Error::new(
                    lit.span(),
                    "Expected an expresion that expands to a single string literal",
                ))
            }
        }
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected an expresion that expands to a single string literal",
        )),
    }
}
