mod parser;
mod syntax;
fn main() {
    let test = parser::integer_parser("1234+abc");
    let (bla, bla1) = match test {
        Ok(v) => v,
        _ => ("testing", "xd")
    };
 //   let takeWhileTest = parser::test_take_while("abc123+"); 

    let let_parser_test = parser::let_parser("let brar:i32 = 23;");

    println!("DIGIT 1: {:?}, {:?}", bla, bla1);

    println!("LET_PARSER: {:?}", let_parser_test);

    let type_parser_test = parser::type_parser("i32:");
    println!("TYPE_PARSER: {:?}", type_parser_test);

    let operand_parser_test = parser::operandParser(" % 23");
    println!("OPERAND_PARSER: {:?}", operand_parser_test);

    let binary_parser_test = parser::binary_parser("1 + 3");
    println!("BINARY_PARSER: {:?}", binary_parser_test);

    let comp_expr_parser_test = parser::comp_expr_parser("3 <= 5");
    println!("COMPARATOR PARSER: {:?}", comp_expr_parser_test)

}
