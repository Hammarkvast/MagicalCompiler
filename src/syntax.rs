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
pub enum comparator {
    equal, 
    lesser, 
    greater,
    lesserEqual,
    greaterEqual,
}