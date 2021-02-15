#[macro_use]
extern crate lalrpop_util;

use thiserror::Error;

mod lexer;
pub mod value;

lalrpop_mod!(parser);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum OpamFileError {
  #[error("invalid char: {0}")]
  LexInvalidChar(char, usize, usize),
  #[error("EOF")]
  LexEof,
  #[error("parse error")]
  Parse,
}

/// See more [Common file format](https://opam.ocaml.org/doc/Manual.html#Common-file-format)
pub fn parse(input: &str) -> Result<value::OpamFile, OpamFileError> {
  let lex_result = lexer::lex(input);
  let lex = match lex_result {
    Ok(lex) => lex,
    Err((lexer::LexErrorKind::InvalidChar(c), start, end)) => {
      return Err(OpamFileError::LexInvalidChar(c, start, end))
    }
    Err((lexer::LexErrorKind::Eof, _, _)) => return Err(OpamFileError::LexEof),
  };
  match parser::mainParser::new().parse(lex) {
    Ok(file) => Ok(file),
    Err(_) => Err(OpamFileError::Parse),
  }
}
