use std::io as io;
use std::env::args;

use parser_combinators::parsers::elem;

use grep_parser_combinators::regex_parser::parse_expression;
use grep_parser_combinators::transformers::expression_parser_factory;

fn main() {
    let mut args_iter = args();
    args_iter.next();
    let regex = args_iter.next().unwrap();

    let (expr, tail) = parse_expression(&regex).unwrap();
    let parser = expression_parser_factory(&expr);

    let mut stream = String::new();
    let stdin = io::stdin();

    loop {
        match stdin.read_line(&mut stream) {
            Ok(0) => break,
            Ok(_) => {
                if let Ok(_) = parser(&stream) {
                    println!("{}", stream);
                }
                stream.clear();
            }
            Err(_) => break,
        }
    }
}
