use parser_combinators::parsers::elem;
use std::io as io;

use grep_parser_combinators::regex_parser::parse_expression;

fn main() {
    //let mut stream = String::new();
    //let mut stdin = io::stdin();
    //stdin.read_line(&mut stream).unwrap();

    let regex = "abc";

    let (expr, tail) = parse_expression(regex).unwrap();

    println!("{:?}", expr);
    println!("{:?}", tail);
}
