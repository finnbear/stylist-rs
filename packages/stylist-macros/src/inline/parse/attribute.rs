use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};
use syn::spanned::Spanned;
use syn::token;

use super::{IntoOutputContext};
use crate::inline::component_value::{
    ComponentValue, ComponentValueStream, InterpolatedExpression, PreservedToken,
};
use crate::inline::css_ident::CssIdent;

#[derive(Debug)]
pub enum CssAttributeName {
    Identifier(CssIdent),
    Expr(InterpolatedExpression),
}

#[derive(Debug)]
pub struct CssAttributeValue {
    values: Vec<ComponentValue>,
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub struct CssAttribute {
    name: CssAttributeName,
    _colon: token::Colon,
    value: CssAttributeValue,
    _terminator: token::Semi,
}

impl Parse for CssAttribute {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl Parse for CssAttributeValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl ComponentValue {
    pub(super) fn maybe_to_attribute_name(self) -> Option<CssAttributeName> {
        todo!()
    }
}