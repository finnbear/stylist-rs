use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Lit};

use super::{OutputCowString, Reify, ReifyContext};
use crate::inline::component_value::PreservedToken;
use crate::inline::css_ident::CssIdent;
use crate::literal::argument::Argument;

#[derive(Debug, Clone)]
pub enum OutputFragment {
    Expr(Expr),
    Arg(Argument),
    Token(PreservedToken),
    Delimiter(Delimiter, /* start: */ bool),
    Str(String),
}

impl From<char> for OutputFragment {
    fn from(c: char) -> Self {
        todo!()
    }
}

impl From<PreservedToken> for OutputFragment {
    fn from(t: PreservedToken) -> Self {
        todo!()
    }
}

impl From<CssIdent> for OutputFragment {
    fn from(i: CssIdent) -> Self {
        todo!()
    }
}

impl From<Expr> for OutputFragment {
    fn from(expr: Expr) -> Self {
        todo!()
    }
}

impl From<Argument> for OutputFragment {
    fn from(arg: Argument) -> Self {
        todo!()
    }
}

impl OutputFragment {
    pub fn into_inner(self) -> OutputCowString {
        todo!()
    }

    fn str_for_delim(d: Delimiter, start: bool) -> &'static str {
        todo!()
    }

    fn as_string(&self) -> Option<String> {
        todo!()
    }
}

impl Reify for OutputFragment {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        todo!()
        
    }
}

pub fn fragment_coalesce(
    l: OutputFragment,
    r: OutputFragment,
) -> Result<OutputFragment, (OutputFragment, OutputFragment)> {
    todo!()
}
