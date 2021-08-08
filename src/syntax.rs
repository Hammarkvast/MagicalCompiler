extern crate nom;
#[derive(Debug, PartialEq, Clone)]
pub enum operand {
    add, 
    sub, 
    mul,
    div,
    modulo,
}
#[derive(Debug, PartialEq, Clone)]
pub enum expr {
    BinaryExpr(Box<expr>, operand, Box<expr>),
    CompExpr(Box<expr>, comparator, Box<expr>),
    i_32(i32),
    boolean(bool),
    var(String),
    If(Box<expr>, Box<expr>),
    Let(Box<expr>, types, Box<expr>),
    Return(Box<expr>),
    While(Box<expr>, Box<expr>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum comparator {
    equal, 
    lesser, 
    greater,
    lesserEqual,
    greaterEqual,
}
#[derive(Debug, PartialEq, Clone)]
pub enum types {
    i_32, 
    string,
    boolean,
}
#[derive(Debug, PartialEq, Clone)]
pub enum var {
    Name(String),
    Value(String),
}


