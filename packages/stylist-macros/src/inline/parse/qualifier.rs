use super::super::component_value::{ComponentValue, ComponentValueStream, PreservedToken};
use super::{IntoOutputContext};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};
use syn::token;

#[derive(Debug, Clone, Default)]
pub struct CssBlockQualifier {
    qualifiers: Vec<ComponentValue>,
    errors: Vec<ParseError>,
}

impl Parse for CssBlockQualifier {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl ToTokens for CssBlockQualifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}