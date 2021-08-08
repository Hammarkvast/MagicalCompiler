#![allow(non_snake_case)]
extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{take_while, take_until, tag},
    combinator::map,
    character::{is_digit},
    character::complete::{multispace0, digit1},
    IResult,
    multi::many0,
    sequence::{delimited, preceded, terminated}     

};

use crate::syntax::*;

pub fn integer_parser(input: &str) -> IResult<&str, &str> {
    let int = preceded(multispace0, digit1)(input);
    return int;
}

pub fn basic_parsing(input: &str) -> IResult<&str, &str>{
    take_until("+")(input)
}

pub fn test_take_while(input: &str) -> IResult<&str, &str> {
    let test = take_while(char::is_alphanumeric)(input);
    return test;
}

pub fn let_parser(input: &str) -> IResult<&str, &str> {
    let t = delimited(
                        multispace0, 
                        tag("let"), 
                        delimited(multispace0, take_while(char::is_alphanumeric), tag(":")))(input);
                        //take_while(char::is_alphanumeric))(input);
    let typeString = match t {
        Ok(v) => v.0,
        _ => panic!(), 
    };
    return t;
}

pub fn return_parser(input: &str) -> expr {
    let res_ret: IResult<&str, &str> = preceded(
        tag("return"),
        delimited(multispace0, take_while(char::is_alphanumeric), tag(";")))(input);
    let (_,ret) = match res_ret {
        Ok(v) => v,
        _ => panic!(),
    };
    return expr::Return(Box::new(expr::var(ret.to_string())));
}

// pub fn if_parser(input: &str) -> expr {
//     let res_if: IResult<&str, &str> = preceded(
//                         tag("if"), 
//                         multispace0)(input);
//     let (t, _) = match res_if {
//         Ok(v) => v,
//         _ => panic!(),
//     };
//     let n = comp_expr_parser(t);

// }

pub fn name_parser(input: &str) -> IResult<&str, &str> {
    let var = preceded(multispace0,
             take_while(char::is_alphanumeric))(input);
    return var;
}

pub fn  type_parser(input: &str) -> (&str, types) {
   let my_type: IResult<&str, types> = preceded(
                        multispace0, 
                        alt((
                            map(tag("i32"), |_| types::i_32),
                            map(tag("bool"), |_| types::boolean),
                    )),
                )(input);
    match my_type {
        Ok(v) => return v, 
        Err(e) => panic!(),
    }
}

pub fn operandParser(input: &str) -> IResult<&str, operand> {
    let op: IResult<&str, operand> = preceded(
                            multispace0, 
                            alt((
                                map(tag("+"), |_| operand::add),
                                map(tag("-"), |_| operand::sub),
                                map(tag("*"), |_| operand::mul),
                                map(tag("/"), |_| operand::div), 
                                map(tag("%"), |_| operand::modulo),
                            ))
                            )(input);
    return op;
}

pub fn comparatorParser(input: &str) -> IResult<&str, comparator> {
    let comp: IResult<&str, comparator> = preceded(
        multispace0, 
        alt((
            map(tag("=="), |_| comparator::equal),
            map(tag("<="), |_| comparator::lesserEqual),
            map(tag(">="), |_| comparator::greaterEqual),
            map(tag("<"), |_| comparator::lesser),
            map(tag(">"), |_| comparator::greater),
        ))
    )(input);
    comp
}

pub fn comp_expr_parser(input: &str) -> expr {
    let (comp_str, int1) = match integer_parser(input) {
        Ok(v) => v,
        _ => panic!("Cannot compare non integers"),
    };
    println!("test");
    let (rest_str, comp) = match comparatorParser(comp_str) {
        Ok(v) => v,
        _ => panic!("Non existent comparator"),
    };
    let (_, int2) = match integer_parser(rest_str) {
        Ok(v) => v,
        _ => panic!("Cannot compare non integers"),
    };
    return expr::CompExpr(Box::new(expr::i_32(int1.parse::<i32>().unwrap())), comp, Box::new(expr::i_32(int2.parse::<i32>().unwrap())))
}

pub fn binary_parser(input: &str) -> expr {
    let (op_str, int1) = match integer_parser(input) {
        Ok(v) => v,
        _ => panic!(),
    };
    let (rest_str, op) = match operandParser(op_str) {
        Ok(v) => v,
        _ => panic!(),
    };
    let (_, int2) = match integer_parser(rest_str) {
        Ok(v) => v,
        _ => panic!(),
    };
    return expr::BinaryExpr(Box::new(expr::i_32(int1.parse::<i32>().unwrap())), op, Box::new(expr::i_32(int2.parse::<i32>().unwrap())));
}

pub fn curly_brack_parser(input: &str) -> expr {
    let cur = preceded(
                                multispace0,
                                delimited(
                                tag("{"), 
                                many0(alt((
                                    let_parser,
                                    return_parser,
                                    if_parser,
                                    while_parser
                                ))), tag("}")))(input);
}

// pub fn if_parser(input: &str) -> expr {
//     let ()
// }