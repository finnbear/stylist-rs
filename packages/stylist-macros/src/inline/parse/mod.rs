use proc_macro2::TokenStream;
use syn::parse::Error as ParseError;

mod attribute;
mod block;
mod qualifier;
mod root;
mod rule;
mod scope;
mod scope_content;

pub use attribute::{CssAttribute, CssAttributeName, CssAttributeValue};
pub use block::CssQualifiedRule;
pub use qualifier::CssBlockQualifier;
pub use root::CssRootNode;
pub use rule::CssAtRule;
pub use scope::CssScope;
pub use scope_content::CssScopeContent;

#[derive(Debug, Default)]
pub struct IntoOutputContext {
    errors: Vec<ParseError>,
}

impl IntoOutputContext {
    pub fn new() -> Self {
        todo!()
    }

    pub fn extend_errors<I>(&mut self, errors: I)
    where
        I: IntoIterator<Item = ParseError>,
    {
        todo!()
    }

    pub fn push_error(&mut self, error: ParseError) {
        todo!()
    }

    pub fn into_compile_errors(self) -> Option<TokenStream> {
        todo!()
    }
}
