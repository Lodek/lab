use parser_combinators::parsers::elem;
use std::io as io;


fn main() {
    let mut stream = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut stream).unwrap();
}
