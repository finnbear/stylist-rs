//! The most prominent token in the css spec is called "component values".
//! You can think of this as being either a block, a function or a preserved (atomic) token.
//!
//! This guides our inline parser as follows:
//! - first re-tokenize the TokenStream into a stream of ComponentValues. For this step see also
//!   [`ComponentValueStream`].
//! - parse and verify the component values into blocks, @-rules and attributes.
//!
//! In general, only a parse error in the first step should be fatal and panic immediately,
//! while a parse error in the second step can recover and display a small precise error location
//! to the user, then continue parsing the rest of the input.
use super::css_ident::CssIdent;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};
use syn::{token, Lit};

mod function_token;
mod interpolated_expression;
mod preserved_token;
mod simple_block;
mod stream;

pub use function_token::FunctionToken;
pub use interpolated_expression::InterpolatedExpression;
pub use preserved_token::PreservedToken;
pub use simple_block::{BlockKind, SimpleBlock};
pub use stream::ComponentValueStream;

#[derive(Debug, Clone)]
pub enum ComponentValue {
    Function(FunctionToken),
    Token(PreservedToken),
    Block(SimpleBlock),
    Expr(InterpolatedExpression),
}

impl ToTokens for ComponentValue {
    fn to_tokens(&self, toks: &mut TokenStream) {
        todo!()
    }
}

impl Parse for ComponentValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        todo!()
    }
}

impl ComponentValue {
    fn parse_multiple(input: &ParseBuffer) -> ParseResult<Vec<Self>> {
        ComponentValueStream::from(input).collect()
    }
}

impl ComponentValue {

    // Overly simplified parsing of a css attribute
    #[must_use = "validation errors should not be discarded"]
    pub fn validate_attribute_token(&self) -> Vec<ParseError> {
        todo!()
    }

    // Overly simplified version of parsing a css selector :)
    pub fn validate_selector_token(&self) -> ParseResult<Vec<ParseError>> {
        todo!()
    }
}
