
use proc_macro::TokenStream;

use wasm_bindgen::JsValue;

#[proc_macro_attribute]
pub fn styled_component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    Default::default()
}
