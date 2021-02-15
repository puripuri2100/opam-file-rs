#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OpamFile {
  pub file_contents: Vec<OpamFileItem>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OpamFileItem {
  Section(Pos, OpamFileSection),
  Variable(Pos, String, Value),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OpamFileSection {
  pub section_kind: String,
  pub section_name: Option<String>,
  pub section_item: Vec<OpamFileItem>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Value {
  pub kind: ValueKind,
  pub pos: Pos,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ValueKind {
  Bool(bool),
  Int(isize),
  String(String),
  RelOp(RelOp, Box<Value>, Box<Value>),
  PrefixRelOp(RelOp, Box<Value>),
  LogOp(LogOp, Box<Value>, Box<Value>),
  PfxOp(PfxOp, Box<Value>),
  Ident(String),
  List(Vec<Value>),
  Group(Vec<Value>),
  Option(Box<Value>, Vec<Value>),
  EnvBinding(Box<Value>, EnvUpdateOp, Box<Value>),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelOp {
  pub kind: RelOpKind,
  pub pos: Pos,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelOpKind {
  /// `=`
  Eq,
  /// `!=`
  Neq,
  /// `>=`
  Geq,
  /// `>`
  Gt,
  /// `<=`
  Leq,
  /// `<`
  Lt,
  /// `~`
  Sem,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LogOp {
  pub kind: LogOpKind,
  pub pos: Pos,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LogOpKind {
  /// `&`
  And,
  /// `|`
  Or,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PfxOp {
  pub kind: PfxOpKind,
  pub pos: Pos,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PfxOpKind {
  /// `!`
  Not,
  /// `?`
  Defined,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EnvUpdateOp {
  pub kind: EnvUpdateOpKind,
  pub pos: Pos,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EnvUpdateOpKind {
  ///// `=`
  //Eq,
  /// `+=`
  PlusEq,
  /// `=+`
  EqPlus,
  /// `:=`
  ColonEq,
  /// `=:`
  EqColon,
  /// `=+=`
  EqPlusEq,
}

pub type Pos = (usize, usize);

pub fn format_opam_file(input: &OpamFile) -> String {
  let file_contents = &input.file_contents;
  opam_file_item_vec_to_string(file_contents)
}

fn opam_file_item_vec_to_string(value: &Vec<OpamFileItem>) -> String {
  value
    .iter()
    .map(|file_content| {
      let str = match file_content {
        OpamFileItem::Section(_, opam_file_section) => {
          let kind = &opam_file_section.section_kind;
          let section_name = opam_file_section
            .clone()
            .section_name
            .unwrap_or_else(|| String::new());
          let section_item_str = opam_file_item_vec_to_string(&opam_file_section.section_item);
          format!("{} {} {{{}}}", kind, section_name, section_item_str)
        }
        OpamFileItem::Variable(_, ident, value) => {
          format!("{} : {}", ident, value_to_string(value))
        }
      };
      format!("{}\n", str)
    })
    .collect::<String>()
}

fn value_to_string(value: &Value) -> String {
  match &value.kind {
    ValueKind::Bool(b) => b.to_string(),
    ValueKind::Int(i) => i.to_string(),
    ValueKind::String(str) => format!("{:?}", str),
    ValueKind::Ident(str) => str.to_string(),
    ValueKind::List(lst) => {
      format!(
        "[{}]",
        lst
          .iter()
          .map(|value| format!("{} ", value_to_string(value)))
          .collect::<String>()
      )
    }
    ValueKind::Group(lst) => {
      format!(
        "({})",
        lst
          .iter()
          .map(|value| format!("{} ", value_to_string(value)))
          .collect::<String>()
      )
    }
    ValueKind::Option(v, lst) => {
      format!(
        "{} {{{}}}",
        lst
          .iter()
          .map(|value| format!("{} ", value_to_string(value)))
          .collect::<String>(),
        value_to_string(v)
      )
    }
    ValueKind::RelOp(op, l, r) => {
      format!(
        "{} {} {}",
        value_to_string(l),
        relop_to_string(&op.kind),
        value_to_string(r),
      )
    }
    ValueKind::PrefixRelOp(op, r) => {
      format!("{} {}", relop_to_string(&op.kind), value_to_string(r),)
    }
    ValueKind::LogOp(op, l, r) => {
      format!(
        "{} {} {}",
        value_to_string(l),
        logop_to_string(&op.kind),
        value_to_string(r),
      )
    }
    ValueKind::PfxOp(op, r) => {
      format!("{} {}", pfxop_to_string(&op.kind), value_to_string(r),)
    }
    ValueKind::EnvBinding(l, op, r) => {
      format!(
        "{} {} {}",
        value_to_string(l),
        envop_to_string(&op.kind),
        value_to_string(r),
      )
    }
  }
}

fn relop_to_string(op: &RelOpKind) -> String {
  match op {
    RelOpKind::Eq => "=".to_string(),
    RelOpKind::Neq => "!=".to_string(),
    RelOpKind::Geq => ">=".to_string(),
    RelOpKind::Gt => ">".to_string(),
    RelOpKind::Leq => "<=".to_string(),
    RelOpKind::Lt => "<".to_string(),
    RelOpKind::Sem => "~".to_string(),
  }
}

fn logop_to_string(op: &LogOpKind) -> String {
  match op {
    LogOpKind::And => "&".to_string(),
    LogOpKind::Or => "|".to_string(),
  }
}

fn pfxop_to_string(op: &PfxOpKind) -> String {
  match op {
    PfxOpKind::Not => "!".to_string(),
    PfxOpKind::Defined => "?".to_string(),
  }
}

fn envop_to_string(op: &EnvUpdateOpKind) -> String {
  match op {
    EnvUpdateOpKind::PlusEq => "+=".to_string(),
    EnvUpdateOpKind::EqPlus => "=+".to_string(),
    EnvUpdateOpKind::ColonEq => ":=".to_string(),
    EnvUpdateOpKind::EqColon => "=:".to_string(),
    EnvUpdateOpKind::EqPlusEq => "=+=".to_string(),
  }
}
