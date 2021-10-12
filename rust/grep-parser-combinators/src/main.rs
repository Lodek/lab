use std::io as io;
use std::env::args;
use std::sync::mpsc;
use std::thread;
use std::process::exit;

use parser_combinators::parsers::elem;

use grep_parser_combinators::regex_parser::parse_expression;
use grep_parser_combinators::transformers::expression_parser_factory;

type BoxedParser = Box<dyn for<'a> Fn(&str) -> Result<((), &str), String>>;

fn parser_from_argv() -> Result<BoxedParser, String> {
    let mut cli_args = args();
    cli_args.next(); //discard executable name
    let regex = match cli_args.next() {
        Some(arg) => arg,
        _ => return Err(String::from("Required regex as argument"))
    };

    let regex_tree = match parse_expression(&regex) {
        Ok((expr, _)) => expr,
        Err(err) => return Err(format!("Invalid regex: {}", err))
    };
    Ok(expression_parser_factory(&regex_tree))
}

/// Producer thread reads one line of input and writes it to tx
fn producer_thread(tx: mpsc::Sender<String>) {
    let mut stream = String::new();
    let stdin = io::stdin();
    loop {
        match stdin.read_line(&mut stream) {
            Ok(0) => { 
                break;
            }
            Ok(_) => {
                tx.send(stream.clone()).unwrap();
                stream.clear();
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(1);
            },
        }
    }
}

fn main() {
    let parser = match parser_from_argv() {
        Ok(parser) => parser,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    thread::spawn(move|| producer_thread(tx));
    
    loop {
        match rx.recv() {
            Ok(line) => {
                // Attemps the regex against every prefix in line
                for i in 0..line.len() {
                    if parser(&line[i..]).is_ok() {
                        print!("{}", line);
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
    exit(0);
}
