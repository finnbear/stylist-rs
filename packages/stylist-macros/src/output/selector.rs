use super::{fragment_coalesce, IntoCowVecTokens, OutputFragment, Reify, ReifyContext};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone)]
pub struct OutputSelector {
    pub selectors: Vec<OutputFragment>,
}

impl Reify for OutputSelector {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
