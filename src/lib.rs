/*!
# opam-file-rs: Parser and printer for the opam file syntax with Rust
[![crates.io][crates-badge]][crates]
[![docs.rs][docs-badge]][docs]
[![Build Status][ci-badge]][ci]
[![source badge][source-badge]][source]
[![license badge][license-badge]][license]
[crates]: https://crates.io/crates/opam-file-rs
[crates-badge]: https://img.shields.io/crates/v/opam-file-rs
[docs]: https://docs.rs/opam-file-rs/
[docs-badge]: https://img.shields.io/badge/docs.rs-opam-file_rs-blue
[ci]: https://github.com/puripuri2100/opam-file-rs/actions?query=workflow%3ACI
[ci-badge]: https://github.com/puripuri2100/opam-file-rs/workflows/CI/badge.svg?branch=master
[source]: https://github.com/puripuri2100/opam-file-rs
[source-badge]: https://img.shields.io/badge/source-github-blue
[license]: https://github.com/puripuri2100/opam-file-rs/blob/master/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue
# Parsing OPAM
Parse OPAM file.
```rust, ignore
use opam_file_rs;
fn main () {
  let opam = r#"
    opam-verion: "2.0"
    version: "0.1.0"
    name: "opam-file-rs"
    dev-repo: "git+https://github.com/puripuri2100/opam-file-rs"
    license: "MIT"
    maintainer: "Naoki Kaneko <puripuri2100@gmail.com>"
    depends: [
      "lalrpop-util" {>= "0.19.4"}
      "thiserror" {>= "1.0.23"}
    ]
  "#;
  assert!(opam_file_rs::parse(opam).is_ok());
}
```
# Convert to a OPAM file format.
A data structure can be converted to an OPAM file format by `value::format_opam_file`.
```rust, ignore
use opam_file_rs;
fn main() {
  let opam_str = r#"
    opam-verion: "2.0"
    version: "0.1.0"
    name: "opam-file-rs"
    dev-repo: "git+https://github.com/puripuri2100/opam-file-rs"
    license: "MIT"
    maintainer: "Naoki Kaneko <puripuri2100@gmail.com>"
    depends: [
      "lalrpop-util" {>= "0.19.4"}
      "thiserror" {>= "1.0.23"}
    ]
  "#;
  let opam = opam_file_rs::parse(opam_str).unwrap();
  println!("{}", opam_file_rs::value::format_opam_file(opam));
}
```
---
(c) 2021 Naoki Kaneko (a.k.a. "puripuri2100")
*/



#[macro_use]
extern crate lalrpop_util;

use thiserror::Error;

mod lexer;
pub mod value;

mod tests;

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
