use super::{Reify, ReifyContext};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub trait IntoCowVecTokens: IntoIterator
where
    Self::Item: Reify,
{
    // Get a TokenStream of an expression of type Cow<'_, [typ]>, containing
    // as elements the values formed by the expressions in this stream.
    // Depending on the context in which the expression can be expanded,
    // uses either Cow::Owned or Cow::Borrowed (currently always Cow::Owned).
    fn into_cow_vec_tokens(self, typ: TokenStream, ctx: &mut ReifyContext) -> TokenStream;
}

impl<I> IntoCowVecTokens for I
where
    I: IntoIterator,
    I::Item: Reify,
{
    fn into_cow_vec_tokens(self, typ: TokenStream, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
    }
}
