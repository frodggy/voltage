use std::{env::args, fs};

use voltage_lexer::Lexer;
use voltage_parser::Parser;

fn read_to_char_vec() -> Vec<char> {
    fs::read_to_string(args().nth(1).unwrap())
        .unwrap()
        .chars()
        .collect()
}

fn main() {
    let source = read_to_char_vec();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();
    println!("{:#?}", tokens)
    // let mut parser = Parser::new(tokens);
    // let ast = parser.parse();
    // cfg_if::cfg_if! {
    //     if #[cfg(feature = "builtin")] {
    //         use voltage_codegen::builtin::Engine;

    //         let mut engine = Engine::new();
    //         let contents = engine.exectute(ast);
    //         #[cfg(feature = "json_abi")]
    //         fs::write("./a.out", contents).unwrap()
    //     }
    // }
}
