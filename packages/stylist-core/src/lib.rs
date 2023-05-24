mod error;
pub use error::{Error, Result, ResultDisplay};
pub mod ast;
pub mod bow;

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod parser;
