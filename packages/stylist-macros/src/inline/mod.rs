pub mod component_value;
pub mod css_ident;

mod parse;

use proc_macro2::TokenStream;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    Default::default()
}
