use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};
use syn::token;

use super::super::component_value::{ComponentValue, ComponentValueStream};
use super::super::css_ident::CssIdent;
use super::{CssScope, IntoOutputContext};

#[derive(Debug)]
pub enum CssAtRuleContent {
    Scope(CssScope),
    Empty(token::Semi),
}

#[derive(Debug)]
pub struct CssAtRule {
    _at: token::At,
    name: CssIdent,
    prelude: Vec<ComponentValue>,
    contents: CssAtRuleContent,
    errors: Vec<ParseError>,
}

impl Parse for CssAtRule {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}
