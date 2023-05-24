use std::mem;

use super::{CssAttribute, CssQualifiedRule, CssScopeContent, IntoOutputContext};
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug)]
pub struct CssRootNode {
    contents: Vec<CssScopeContent>,
}

impl Parse for CssRootNode {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}