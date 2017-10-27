extern crate lapis_lib;

use std::io::{self, BufRead, Write};

use lapis_lib::token::Token;
use lapis_lib::lexer::Lexer;

fn main() {
  let stdin = io::stdin();

  loop {
    print!(">> ");
    // Stdout needs to be flushed, due to missing newline
    io::stdout().flush().expect("Error flushing stdout");

    let mut line = String::new();
    stdin.lock().read_line(&mut line).expect("Error reading from stdin!");

    let mut lexer = Lexer::new(&mut line);

    loop {
      let tok = lexer.next_token();
      println!("{:?}", tok);
      if tok == Token::EndOfFile {
        break;
      }
    }
  }
}
