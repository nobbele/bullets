#[derive(Debug, PartialEq)]
pub enum Type {
    Float,
    String,
    //    Bool,
}

#[derive(Debug, PartialEq)]
pub struct Name(pub String);

#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: Name,
    pub r#type: Type,
}

#[derive(Debug, PartialEq)]
pub enum Op2 {
    // precedence level 1
    Mul,
    Div,
    Mod,
    // precedence level 2
    Add,
    Sub,
    // precedence level 3
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Var(Name),   // foo
    State(Name), // $pos_x
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Float(f32),
    String(String),
    Symbol(Symbol),
    Op2(Op2, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Body {
    LexicalDefine(Symbol, Expr),
    ProcCall(Name, Vec<Expr>),
    If(Expr, Vec<Body>, Vec<Body>),
    Return(Option<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum SyntaxTree {
    GlobalDefine(Symbol, Expr),
    DefProc(Name, Vec<Arg>, Vec<Body>),
}
