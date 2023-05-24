use std::mem;

use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};
use syn::{braced, token};

use super::{CssAttribute, CssQualifiedRule, CssScopeContent, IntoOutputContext};

#[derive(Debug)]
pub struct CssScope {
    _brace: token::Brace,
    pub contents: Vec<CssScopeContent>,
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}
