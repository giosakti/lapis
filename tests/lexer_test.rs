extern crate lapis_lib;

use std::fs::File;
use std::io::prelude::*;
use lapis_lib::token::Token;
use lapis_lib::lexer::Lexer;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[test]
fn next_token_test() {
    let tests = vec![
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Integer("5".to_string()),
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Integer("10".to_string()),
        Token::Definition,
        Token::Ident("add".to_string()),
        Token::LeftParenthesis,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RightParenthesis,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::End,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LeftParenthesis,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RightParenthesis,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer("5".to_string()),
        Token::Integer("5".to_string()),
        Token::LowerThan,
        Token::Integer("10".to_string()),
        Token::GreaterThan,
        Token::Integer("5".to_string()),
        Token::If,
        Token::LeftParenthesis,
        Token::Integer("5".to_string()),
        Token::LowerThan,
        Token::Integer("10".to_string()),
        Token::RightParenthesis,
        Token::Return,
        Token::True,
        Token::Else,
        Token::Return,
        Token::False,
        Token::End,
        Token::Integer("10".to_string()),
        Token::Equal,
        Token::Integer("10".to_string()),
        Token::Integer("10".to_string()),
        Token::NotEqual,
        Token::Integer("9".to_string()),
        Token::EndOfFile,
    ];

    let code_string = read_file("examples/token_example.rb".to_owned()).ok().unwrap();
    let mut l = Lexer::new(&code_string);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok, t);
    }
}
