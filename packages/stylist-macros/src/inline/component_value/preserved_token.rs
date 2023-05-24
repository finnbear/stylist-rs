use super::super::css_ident::CssIdent;
use proc_macro2::{Literal, Punct, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug, Clone)]
pub enum PreservedToken {
    Punct(Punct),
    Literal(Literal),
    Ident(CssIdent),
}

impl ToTokens for PreservedToken {
    fn to_tokens(&self, toks: &mut TokenStream) {
        todo!()
    }
}

impl Parse for PreservedToken {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl PreservedToken {
    pub fn to_output_string(&self) -> String {
        todo!()
    }
}
