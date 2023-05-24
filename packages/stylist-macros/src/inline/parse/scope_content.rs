use itertools::Itertools;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

use super::super::component_value::{ComponentValue, ComponentValueStream, PreservedToken};
use super::{CssAtRule, CssAttribute, CssQualifiedRule};

#[derive(Debug)]
pub enum CssScopeContent {
    Attribute(CssAttribute),
    AtRule(CssAtRule),
    Nested(CssQualifiedRule),
}

impl Parse for CssScopeContent {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl CssScopeContent {
    // §5.4.1: Consume a list of rules
    pub fn consume_list_of_rules(input: &ParseBuffer) -> ParseResult<Vec<Self>> {
        todo!()
    }
}
