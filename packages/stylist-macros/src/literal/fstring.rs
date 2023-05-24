use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{all_consuming, cut, map, opt, recognize};
use nom::error::{context, convert_error, ErrorKind, ParseError, VerboseError};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use stylist_core::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Fragment {
    Literal(String),
    Interpolation(String),
}

#[derive(Debug)]
pub(crate) struct Parser {}

impl Parser {
    /// Returns Error when string is Empty
    fn expect_non_empty(i: &str) -> std::result::Result<(), nom::Err<VerboseError<&str>>> {
        Ok(())
    }

    /// Parse whitespace
    fn sp(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        todo!()
    }

    /// Drop whitespaces
    fn trimmed<'a, F, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&str>>
    where
        F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
    {
        |_| todo!()
    }

    /// Parse a string interpolation.
    fn interpolation(i: &str) -> IResult<&str, Fragment, VerboseError<&str>> {
        todo!()
    }

    #[allow(clippy::let_and_return)]
    fn literal(i: &str) -> IResult<&str, Fragment, VerboseError<&str>> {
        todo!()
    }

    fn fragments(i: &str) -> IResult<&str, Vec<Fragment>, VerboseError<&str>> {
        todo!()
    }

    pub fn parse(s: &str) -> Result<Vec<Fragment>> {
        todo!()
    }
}
