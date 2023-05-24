use super::{OutputBlock, OutputRule, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputScopeContent {
    Rule(OutputRule),
    Block(OutputBlock),
}

impl Reify for OutputScopeContent {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
