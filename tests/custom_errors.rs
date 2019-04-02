#![allow(dead_code)]
#![cfg_attr(feature = "cargo-clippy", allow(block_in_if_condition_stmt))]

#[macro_use]
extern crate nom;

use nom::{IResult, ErrorKind, ParseError};
use nom::digit;

use std::convert::From;

#[derive(Debug)]
pub struct CustomError(String);

impl From<(&str, ErrorKind)> for CustomError {
  fn from(error: (&str, ErrorKind)) -> Self {
    CustomError(format!("error code was: {:?}", error))
  }
}

impl ParseError<&str> for CustomError {
  fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
    CustomError(format!("error code was: {:?}", kind))
  }

  fn append(input: &str, kind: ErrorKind, other: CustomError) -> Self {
    CustomError(format!("{:?}\nerror code was: {:?}", other, kind))

  }

}

fn test1(input: &str) -> IResult<&str, &str, CustomError> {
  //fix_error!(input, CustomError, tag!("abcd"))
  tag!(input, "abcd")
}

fn test2(input: &str) -> IResult<&str, &str, CustomError> {
  //terminated!(input, test1, fix_error!(CustomError, digit))
  terminated!(input, test1, digit)
}

fn test3(input: &str) -> IResult<&str, &str, CustomError> {
  verify!(input, test1, |s: &str| {
    s.starts_with("abcd")
  })
}

#[cfg(feature = "alloc")]
fn test4(input: &str) -> IResult<&str, Vec<&str>, CustomError> {
  count!(input, test1, 4)
}
