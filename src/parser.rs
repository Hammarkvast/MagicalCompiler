extern crate nom;

use nom::{
    bytes::complete::{take_while, take_until, tag},
    character::{is_digit, is_alphanumeric},
    character::complete::{multispace0, digit1},
    IResult,
    sequence::{delimited, preceded, terminated}     

};


use crate::syntax::*;

pub fn testparse(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

pub fn basic_parsing(input: &str) -> IResult<&str, &str>{
    take_until("+")(input)
}