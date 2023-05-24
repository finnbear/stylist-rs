
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod literal;


#[proc_macro]
#[proc_macro_error]
pub fn sheet(input: TokenStream) -> TokenStream {
    //sheet::macro_fn(input.into()).into()
    Default::default()
}

#[proc_macro]
#[proc_macro_error]
pub fn style(input: TokenStream) -> TokenStream {
    //style::macro_fn(input.into()).into()
    Default::default()
}

#[proc_macro]
#[proc_macro_error]
pub fn global_style(input: TokenStream) -> TokenStream {
    //global_style::macro_fn(input.into()).into()
    Default::default()
}

#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    //css::macro_fn(input.into()).into()
    Default::default()
}

#[proc_macro]
#[proc_macro_error]
pub fn use_style(input: TokenStream) -> TokenStream {
    //use_style::macro_fn(input.into()).into()
    Default::default()
}

#[proc_macro_attribute]
pub fn styled_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    //styled_component::macro_fn(attr, item)
    Default::default()
}

#[proc_macro_attribute]
pub fn styled_component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    //styled_component_impl::macro_fn(attr, item)
    Default::default()
}
