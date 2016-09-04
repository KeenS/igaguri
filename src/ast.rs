#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Input {
    Input(Ast),
    Interupt,
    Eof,
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ast {
    Command{cmd: Vec<String>, out: Option<String>},
    Pipe(Vec<Ast>),
}

impl Ast {
    pub fn is_empty(&self) -> bool {
        use self::Ast::*;
        match self {
            &Command{cmd: ref cmd, ..} => cmd.is_empty(),
            &Pipe(ref vec) => vec.is_empty(),
        }
    }
}
