use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};
use syn::{braced, token, Expr};

#[derive(Debug, Clone)]
pub struct InterpolatedExpression {
    dollar: token::Dollar,
    braces: token::Brace,
    expr: Box<Expr>,
}

impl ToTokens for InterpolatedExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}

impl Parse for InterpolatedExpression {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}