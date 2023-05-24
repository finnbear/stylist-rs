use proc_macro2::{Punct, Spacing, TokenStream};
use quote::ToTokens;
use std::fmt::{Display, Formatter};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};
use syn::{token, Ident};

syn::custom_punctuation!(DoubleSub, --);

#[derive(Debug, Clone)]
pub enum IdentPart {
    Dash(Punct),
    Ident(Ident),
}

#[derive(Debug, Clone)]
pub struct CssIdent {
    parts: Vec<IdentPart>,
}

impl IdentPart {
    pub fn peek(lookahead: &ParseBuffer, accept_dash: bool, accept_ident: bool) -> bool {
        todo!()
    }
}

impl Display for CssIdent {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl CssIdent {
    pub fn peek(lookahead: &ParseBuffer) -> bool {
        todo!()
    }

    pub fn to_output_string(&self) -> String {
        todo!()
    }
}

impl IdentPart {
    fn parse_part(
        input: &ParseBuffer,
        accept_dash: bool,
        accept_ident: bool,
    ) -> ParseResult<IdentPart> {
        todo!()
    }
}

impl Parse for CssIdent {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl ToTokens for IdentPart {
    fn to_tokens(&self, toks: &mut TokenStream) {
        todo!()
    }
}

impl ToTokens for CssIdent {
    fn to_tokens(&self, toks: &mut TokenStream) {
        todo!()
    }
}
