use std::str::Chars;
use std::iter::Peekable;

use token;
use token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LowerThan,
            Some('>') => Token::GreaterThan,
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,

            Some(ch @ _) => {
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    token::lookup_ident(&literal)
                } else if ch.is_numeric() {
                    Token::Integer(self.read_number(ch))
                } else {
                    Token::Illegal
                }
            }

            // EOF
            None => Token::EndOfFile,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn peek_char_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_char_is_letter() {
            ident.push(self.read_char().unwrap());
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        number
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
