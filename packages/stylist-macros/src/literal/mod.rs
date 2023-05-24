use proc_macro2::{TokenStream, TokenTree};

use std::collections::{HashMap, HashSet};

use litrs::StringLit;
use proc_macro_error::{abort, abort_call_site};
use std::convert::TryFrom;

use stylist_core::ast::Sheet;

pub mod argument;
mod fstring;
mod to_output_with_args;

use argument::Argument;


pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    todo!()
}
