#[derive(Debug, Clone)]
pub struct Prog {
  pub funcs: Vec<Func>,
}

#[derive(Debug, Clone)]
pub struct Func {
  pub name: String,
  pub params: Vec<String>,
  pub stmts: Option<Stmt>,
}
#[derive(Debug, Clone)]
pub enum Stmt {
  Empty,
  Ret(Expr),
  Def(String, Option<Expr>),
  Expr(Expr),
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),
  Block(Block),
  While(Expr, Box<Stmt>),
  DoWhile(Box<Stmt>, Expr),
  For {
    init: Option<Box<Stmt>>,
    cond: Option<Expr>,
    update: Option<Expr>,
    body: Box<Stmt>,
  },
  Continue,
  Break,
}

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Stmt>);

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
  Neg,
  BNot,
  LNot,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Lt,
  Le,
  Ge,
  Gt,
  Eq,
  Ne,
  And,
  Or,
  Mod,
}

#[derive(Debug, Clone)]
pub enum Expr {
  Int(i32),
  Unary(UnaryOp, Box<Expr>),
  Binary(BinaryOp, Box<Expr>, Box<Expr>),
  Var(String),
  Assign(String, Box<Expr>),
  Condition(Box<Expr>, Box<Expr>, Box<Expr>),
  Call(String, Vec<Expr>),
}
