#[test]
fn check_parse() {
  use crate::value::*;
  let opam_str = r#"
    opam-version: "2.0"
    depends: [
      "lalrpop-util" {>= "0.19.4"}
    ]
  "#;
  let opam_data = OpamFile {
    file_contents: vec![
      OpamFileItem::Variable(
        (5, 24),
        "opam-version".to_string(),
        Value {
          kind: ValueKind::String("2.0".to_string()),
          pos: (19, 24),
        },
      ),
      OpamFileItem::Variable(
        (29, 80),
        "depends".to_string(),
        Value {
          kind: ValueKind::List(vec![Value {
            kind: ValueKind::Option(
              Box::new(Value {
                kind: ValueKind::String("lalrpop-util".to_string()),
                pos: (46, 60),
              }),
              vec![Value {
                kind: ValueKind::PrefixRelOp(
                  RelOp {
                    kind: RelOpKind::Geq,
                    pos: (62, 64),
                  },
                  Box::new(Value {
                    kind: ValueKind::String("0.19.4".to_string()),
                    pos: (65, 73),
                  }),
                ),
                pos: (62, 73),
              }],
            ),
            pos: (46, 74),
          }]),
          pos: (38, 80),
        },
      ),
    ],
  };
  assert_eq!(crate::parse(opam_str).unwrap(), opam_data);
}
