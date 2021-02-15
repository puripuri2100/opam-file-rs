use thiserror::Error;

use super::value;

#[allow(unused)]
pub type Token = (TokenKind, usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum TokenKind {
  EOF,
  STRING(String),
  IDENT(String),
  BOOL(bool),
  INT(isize),
  LBRACKET,
  RBRACKET,
  LPAR,
  RPAR,
  LBRACE,
  RBRACE,
  COLON,
  AND,
  OR,
  RELOP(value::RelOpKind),
  PFXOP(value::PfxOpKind),
  ENVOP(value::EnvUpdateOpKind),
}

#[allow(unused)]
pub fn get_value_bool(kind: TokenKind) -> Option<bool> {
  match kind {
    TokenKind::BOOL(b) => Some(b),
    _ => None,
  }
}

#[allow(unused)]
pub fn get_value_string(kind: TokenKind) -> Option<String> {
  match kind {
    TokenKind::STRING(s) => Some(s),
    TokenKind::IDENT(s) => Some(s),
    _ => None,
  }
}

#[allow(unused)]
pub fn get_value_isize(kind: TokenKind) -> Option<isize> {
  match kind {
    TokenKind::INT(i) => Some(i),
    _ => None,
  }
}

#[allow(unused)]
pub fn get_value_pfxop(kind: TokenKind) -> Option<value::PfxOpKind> {
  match kind {
    TokenKind::PFXOP(p) => Some(p),
    _ => None,
  }
}

#[allow(unused)]
pub fn get_value_relop(kind: TokenKind) -> Option<value::RelOpKind> {
  match kind {
    TokenKind::RELOP(r) => Some(r),
    _ => None,
  }
}

#[allow(unused)]
pub fn get_value_env(kind: TokenKind) -> Option<value::EnvUpdateOpKind> {
  match kind {
    TokenKind::ENVOP(e) => Some(e),
    _ => None,
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum LexErrorKind {
  #[error("invalid char: {0}")]
  InvalidChar(char),
  #[error("EOF")]
  Eof,
}

pub type LexError = (LexErrorKind, usize, usize);

fn error_invalid_char(c: char, start: usize, end: usize) -> LexError {
  (LexErrorKind::InvalidChar(c), start, end)
}

fn error_eof(start: usize) -> LexError {
  (LexErrorKind::Eof, start, start + 1)
}

#[allow(unused)]
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
  let mut tokens = Vec::new();
  let input = input.chars().collect::<Vec<_>>();
  let mut pos = 0;
  let (token_lst, new_pos) = lex_token(&input, pos)?;
  pos = new_pos;
  let mut l = token_lst;
  tokens.append(&mut l);
  tokens.push((TokenKind::EOF, pos, pos + 1));
  Ok(tokens)
}

fn lex_token(input: &[char], pos: usize) -> Result<(Vec<Token>, usize), LexError> {
  let mut pos = pos;
  let mut tokens = Vec::new();
  while pos < input.len() {
    match input[pos] {
      ':' => match input.get(pos + 1) {
        Some('=') => {
          tokens.push((
            TokenKind::ENVOP(value::EnvUpdateOpKind::ColonEq),
            pos,
            pos + 2,
          ));
          pos += 2;
        }
        _ => {
          tokens.push((TokenKind::COLON, pos, pos + 1));
          pos += 1;
        }
      },
      '{' => {
        tokens.push((TokenKind::LBRACE, pos, pos + 1));
        pos += 1;
      }
      '}' => {
        tokens.push((TokenKind::RBRACE, pos, pos + 1));
        pos += 1;
      }
      '[' => {
        tokens.push((TokenKind::LBRACKET, pos, pos + 1));
        pos += 1;
      }
      ']' => {
        tokens.push((TokenKind::RBRACKET, pos, pos + 1));
        pos += 1;
      }
      '(' => match input.get(pos + 1) {
        Some('*') => {
          let new_pos = lex_comment(1, input, pos + 1);
          pos = new_pos;
        }
        _ => {
          tokens.push((TokenKind::LPAR, pos, pos + 1));
          pos += 1;
        }
      },
      ')' => {
        tokens.push((TokenKind::RPAR, pos, pos + 1));
        pos += 1;
      }
      '#' => {
        let new_pos = lex_line_comment(input, pos + 1);
        pos = new_pos;
      }
      '&' => {
        tokens.push((TokenKind::AND, pos, pos + 1));
        pos += 1;
      }
      '|' => {
        tokens.push((TokenKind::OR, pos, pos + 1));
        pos += 1;
      }
      '?' => {
        tokens.push((TokenKind::PFXOP(value::PfxOpKind::Defined), pos, pos + 1));
        pos += 1;
      }
      '!' => match input.get(pos + 1) {
        Some('=') => {
          tokens.push((TokenKind::RELOP(value::RelOpKind::Neq), pos, pos + 2));
          pos += 2;
        }
        _ => {
          tokens.push((TokenKind::PFXOP(value::PfxOpKind::Not), pos, pos + 1));
          pos += 1;
        }
      },
      '>' => match input.get(pos + 1) {
        Some('=') => {
          tokens.push((TokenKind::RELOP(value::RelOpKind::Geq), pos, pos + 2));
          pos += 2;
        }
        _ => {
          tokens.push((TokenKind::RELOP(value::RelOpKind::Gt), pos, pos + 1));
          pos += 1;
        }
      },
      '<' => match input.get(pos + 1) {
        Some('=') => {
          tokens.push((TokenKind::RELOP(value::RelOpKind::Leq), pos, pos + 2));
          pos += 2;
        }
        _ => {
          tokens.push((TokenKind::RELOP(value::RelOpKind::Lt), pos, pos + 1));
          pos += 1;
        }
      },
      '~' => {
        tokens.push((TokenKind::RELOP(value::RelOpKind::Sem), pos, pos + 1));
        pos += 1;
      }
      '=' => match input.get(pos + 1) {
        Some(':') => {
          tokens.push((
            TokenKind::ENVOP(value::EnvUpdateOpKind::EqColon),
            pos,
            pos + 2,
          ));
          pos += 2;
        }
        Some('+') => match input.get(pos + 2) {
          Some('=') => {
            tokens.push((
              TokenKind::ENVOP(value::EnvUpdateOpKind::EqPlusEq),
              pos,
              pos + 3,
            ));
            pos += 3;
          }
          _ => {
            tokens.push((
              TokenKind::ENVOP(value::EnvUpdateOpKind::EqPlus),
              pos,
              pos + 2,
            ));
            pos += 2;
          }
        },
        _ => {
          tokens.push((TokenKind::RELOP(value::RelOpKind::Eq), pos, pos + 1));
          pos += 1;
        }
      },
      '+' => match input.get(pos + 1) {
        Some('=') => {
          tokens.push((
            TokenKind::ENVOP(value::EnvUpdateOpKind::PlusEq),
            pos,
            pos + 2,
          ));
          pos += 2;
        }
        _ => return Err(error_invalid_char('+', pos, pos + 1)),
      },
      '"' => match (input.get(pos + 1), input.get(pos + 2)) {
        (Some('"'), Some('"')) => {
          let (token, new_pos) = lex_string_triple(input, pos + 3)?;
          tokens.push(token);
          pos = new_pos;
        }
        _ => {
          let (token, new_pos) = lex_string(input, pos)?;
          tokens.push(token);
          pos = new_pos;
        }
      },
      ' ' | '\n' | '\t' | '\r' => {
        pos += 1;
      }
      '-' => match input.get(pos + 1) {
        Some(c) if c.is_ascii_digit() => {
          let (token, new_pos) = lex_int(true, input, pos);
          tokens.push(token);
          pos = new_pos;
        }
        _ => return Err(error_invalid_char('-', pos, pos + 1)),
      },
      c if c.is_ascii_digit() => {
        let (token, new_pos) = lex_int(false, input, pos);
        tokens.push(token);
        pos = new_pos;
      }
      c if c.is_ascii_alphabetic() => {
        let (token, new_pos) = lex_ident(input, pos);
        tokens.push(token);
        pos = new_pos;
      }
      c => return Err(error_invalid_char(c, pos, pos + 1)),
    }
  }
  Ok((tokens, pos))
}

fn lex_comment(depth: usize, input: &[char], pos: usize) -> usize {
  let mut pos = pos;
  while pos < input.len() {
    match input[pos] {
      '*' => match input.get(pos + 1) {
        Some(')') => {
          pos += 1;
          break;
        }
        _ => {
          pos += 1;
        }
      },
      '(' => match input.get(pos + 1) {
        Some('*') => {
          pos += 1;
          let new_pos = lex_comment(depth + 1, input, pos);
          pos = new_pos;
        }
        _ => {
          pos += 1;
        }
      },
      _ => pos += 1,
    };
  }
  pos
}

fn lex_line_comment(input: &[char], pos: usize) -> usize {
  let mut pos = pos;
  while pos < input.len() && !('\n' == input[pos]) {
    pos += 1;
  }
  pos
}

fn lex_string(input: &[char], pos: usize) -> Result<(Token, usize), LexError> {
  let mut str = String::new();
  let start = pos + 1;
  let mut s_pos = start;
  loop {
    match input.get(s_pos) {
      None => return Err(error_eof(pos)),
      Some(c) => match c {
        '\\' => {
          let (escape_str, new_pos) = lex_escape(input, pos)?;
          s_pos = new_pos;
          str.push_str(&escape_str)
        }
        '"' => {
          s_pos += 1;
          break;
        }
        _ => {
          s_pos = s_pos + 1;
          str.push(*c)
        }
      },
    }
  }
  Ok(((TokenKind::STRING(str), start - 1, s_pos), s_pos))
}

fn lex_string_triple(input: &[char], pos: usize) -> Result<(Token, usize), LexError> {
  let mut str = String::new();
  let start = pos;
  let mut s_pos = start;
  loop {
    match input.get(s_pos) {
      None => return Err(error_eof(pos)),
      Some(c) => match c {
        '\\' => {
          let (escape_str, new_pos) = lex_escape(input, pos)?;
          s_pos = new_pos;
          str.push_str(&escape_str)
        }
        '"' => match (input.get(s_pos + 1), input.get(s_pos + 2)) {
          (Some('"'), Some('"')) => {
            s_pos += 3;
            break;
          }
          (Some('"'), _) => {
            s_pos += 2;
            str.push_str("\"\"");
          }
          _ => {
            s_pos += 1;
            str.push_str("\"");
          }
        },
        _ => {
          str.push(*c);
          s_pos = s_pos + 1;
        }
      },
    }
  }
  Ok(((TokenKind::STRING(str), start, s_pos), s_pos))
}

fn lex_escape(input: &[char], pos: usize) -> Result<(String, usize), LexError> {
  match input.get(pos + 1) {
    Some('\\') => Ok(("\\".to_string(), pos + 2)),
    Some('"') => Ok(("\"".to_string(), pos + 2)),
    Some('\'') => Ok(("\'".to_string(), pos + 2)),
    Some('n') => Ok(("\n".to_string(), pos + 2)),
    Some('r') => Ok(("\r".to_string(), pos + 2)),
    Some('t') => Ok(("\t".to_string(), pos + 2)),
    Some('b') => Ok(("\u{0008}".to_string(), pos + 2)),
    Some('x') => match (input.get(pos + 2), input.get(pos + 3)) {
      (Some(c1), Some(c2)) if (c1.is_ascii_hexdigit() && c2.is_ascii_hexdigit()) => {
        let hex = vec![*c1, *c2].iter().collect::<String>();
        let hex_i64 = i64::from_str_radix(&hex, 16).unwrap();
        let str = String::from_utf8(vec![hex_i64 as u8]).unwrap();
        Ok((str, pos + 4))
      }
      _ => Err(error_invalid_char('x', pos, pos + 1)),
    },
    Some(c) if c.is_ascii_digit() => match (input.get(pos + 2), input.get(pos + 3)) {
      (Some(c1), Some(c2)) if (c1.is_ascii_digit() && c2.is_ascii_digit()) => {
        let hex = vec![*c, *c1, *c2].iter().collect::<String>();
        let hex_i64 = i64::from_str_radix(&hex, 10).unwrap();
        let str = String::from_utf8(vec![hex_i64 as u8]).unwrap();
        Ok((str, pos + 4))
      }
      _ => Err(error_invalid_char(*c, pos, pos + 1)),
    },
    Some(c) => Err(error_invalid_char(*c, pos + 1, pos + 2)),
    None => Err(error_eof(pos + 1)),
  }
}

#[test]
fn check_lex_escape_n() {
  assert_eq!(
    lex_escape(&"\\n".chars().collect::<Vec<_>>(), 0),
    Ok(("\n".to_string(), 2))
  )
}

#[test]
fn check_lex_escape_hex_unicode() {
  assert_eq!(
    lex_escape(&"\\x4E".chars().collect::<Vec<_>>(), 0),
    Ok(("N".to_string(), 4))
  )
}

#[test]
fn check_lex_escape_digit_unicode() {
  assert_eq!(
    lex_escape(&"\\078".chars().collect::<Vec<_>>(), 0),
    Ok(("N".to_string(), 4))
  )
}

fn lex_int(is_minus: bool, input: &[char], pos: usize) -> (Token, usize) {
  let start = pos;
  let mut pos = pos;
  let mut str = if is_minus {
    String::new()
  } else {
    "-".to_string()
  };
  while pos < input.len() {
    if input[pos].is_ascii_digit() {
      str.push(input[pos]);
      pos += 1;
    } else {
      break;
    }
  }
  let int = str.parse::<isize>().unwrap();
  ((TokenKind::INT(int), start, pos), pos)
}

fn lex_ident(input: &[char], pos: usize) -> (Token, usize) {
  let start = pos;
  let mut pos = pos;
  let mut str = String::new();
  while pos < input.len() {
    let c = input[pos];
    if c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' || c == '-' {
      str.push(c);
      pos += 1;
    } else {
      break;
    }
  }
  if str == "true".to_string() {
    ((TokenKind::BOOL(true), start, pos), pos)
  } else if str == "false".to_string() {
    ((TokenKind::BOOL(false), start, pos), pos)
  } else {
    ((TokenKind::IDENT(str), start, pos), pos)
  }
}
