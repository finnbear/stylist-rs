use super::{
    fragment_coalesce, IntoCowVecTokens, OutputFragment, OutputRuleBlockContent, Reify,
    ReifyContext,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct OutputRule {
    pub condition: Vec<OutputFragment>,
    pub content: Vec<OutputRuleBlockContent>,
}

impl Reify for OutputRule {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
