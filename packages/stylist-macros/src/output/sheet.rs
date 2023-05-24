use super::{IntoCowVecTokens, OutputScopeContent, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct OutputSheet {
    pub contents: Vec<OutputScopeContent>,
}

impl Reify for OutputSheet {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
