
use proc_macro::TokenStream;

// Error.
use stylist_core::Error;

#[proc_macro_attribute]
pub fn styled_component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    Default::default()
}
