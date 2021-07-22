mod parser;
mod syntax;
fn main() {
    let test = parser::testparse("1234abc");
    let (bla, bla1) = match test {
        Ok(v) => v,
        _ => ("testing", "xd")
    };
    let takeWhileTest = parser::basic_parsing("abcd123+"); 

    println!("{:?} and {:?} and {:?}", bla, bla1, takeWhileTest);
}
