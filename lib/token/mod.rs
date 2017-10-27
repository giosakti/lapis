#[derive(Debug, PartialEq)]
pub enum Token {
  Illegal,
  EndOfFile,

  // Identifiers + literals
  Ident(String),
  Integer(String),

  // Operators
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,

  LowerThan,
  GreaterThan,
  Equal,
  NotEqual,

  // Delimiters
  Comma,
  Semicolon,
  LeftParenthesis,
  RightParenthesis,
  LeftBrace,
  RightBrace,

  // Keywords
  Definition,
  End,
  True,
  False,
  If,
  Else,
  Return,
}

impl Default for Token {
  fn default() -> Token {
    Token::Illegal
  }
}

pub fn lookup_ident(ident: &str) -> Token {
  match ident {
    "def" => Token::Definition,
    "end" => Token::End,
    "true" => Token::True,
    "false" => Token::False,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
    _ => Token::Ident(ident.to_string()),
  }
}
