use super::super::css_ident::CssIdent;
use super::ComponentValue;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};
use syn::{parenthesized, token};

// Css-syntax parses like
// v----v function token
// foobar( arg1 , arg2 )
//         ---- - ---- ^-- closing bracket
//         arguments
//
// while this parses like
//
// v-------------------v function token
// foobar( arg1 , arg2 )
//         ---- - ----
//         arguments
//
// This should not lead to noticable differences since we make no effort to
// do the insane special handling of 'url()' functions that's in the
// css spec
#[derive(Debug, Clone)]
pub struct FunctionToken {
    pub(super) name: CssIdent,
    pub(super) paren: token::Paren,
    pub(super) args: Vec<ComponentValue>,
}

impl ToTokens for FunctionToken {
    fn to_tokens(&self, toks: &mut TokenStream) {
        todo!()
    }
}

impl Parse for FunctionToken {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl FunctionToken {
    pub(super) fn parse_with_name(name: CssIdent, input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}
