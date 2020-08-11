#[derive(Debug, Clone)]
pub struct Prog {
  pub func: Func,
}

#[derive(Debug, Clone)]
pub struct Func {
  pub name: String,
  pub stmts: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub enum Stmt {
  Ret(Expr),
  Def(String, Option<Expr>),
  Expr(Expr),
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

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
}

#[derive(Debug, Clone)]
pub enum Expr {
  Int(i32),
  Unary(UnaryOp, Box<Expr>),
  Binary(BinaryOp, Box<Expr>, Box<Expr>),
  Var(String),
  Assign(String, Box<Expr>),
  Condition(Box<Expr>, Box<Expr>, Box<Expr>),
}
