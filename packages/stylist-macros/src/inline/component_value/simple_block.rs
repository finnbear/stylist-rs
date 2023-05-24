use super::ComponentValue;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};
use syn::{braced, bracketed, parenthesized, token};

#[derive(Debug, Clone)]
pub enum BlockKind {
    Braced(token::Brace),
    Bracketed(token::Bracket),
    Paren(token::Paren),
}

impl BlockKind {
    pub fn surround_tokens(&self) -> (char, char) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct SimpleBlock {
    pub kind: BlockKind,
    pub contents: Vec<ComponentValue>,
}

impl ToTokens for SimpleBlock {
    fn to_tokens(&self, toks: &mut TokenStream) {
        todo!()
    }
}

impl Parse for SimpleBlock {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}
