use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

use super::{CssAttribute, CssBlockQualifier, CssScope, IntoOutputContext};

#[derive(Debug)]
pub struct CssQualifiedRule {
    pub qualifier: CssBlockQualifier,
    scope: CssScope,
}

impl Parse for CssQualifiedRule {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}
