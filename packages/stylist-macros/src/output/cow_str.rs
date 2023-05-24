use super::{Reify, ReifyContext};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::LitStr;

#[derive(Debug)]
pub enum OutputCowString {
    Str(String),
    Raw(TokenStream, ReifyContext),
}

impl From<String> for OutputCowString {
    fn from(s: String) -> Self {
        todo!()
    }
}

impl OutputCowString {
    pub fn from_displayable_spanned(source: impl Spanned, expr: impl Reify) -> Self {
        todo!()
    }
}

impl Reify for OutputCowString {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
