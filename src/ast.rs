#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Input {
    Input(Ast),
    Interupt,
    Eof,
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ast {
    Command(Vec<String>),
    Pipe(Vec<Ast>),
}

impl Ast {
    pub fn is_empty(&self) -> bool {
        use self::Ast::*;
        match self {
            &Command(ref vec) => vec.is_empty(),
            &Pipe(ref vec) => vec.is_empty(),
        }
    }
}
