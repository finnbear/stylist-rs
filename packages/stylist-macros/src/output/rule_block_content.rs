use super::{OutputAttribute, OutputBlock, OutputRule, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleBlockContent {
    Rule(Box<OutputRule>),
    Block(Box<OutputBlock>),
    StyleAttr(OutputAttribute),
}

impl Reify for OutputRuleBlockContent {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
