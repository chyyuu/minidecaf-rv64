use crate::ast::*;
use std::collections::HashMap;

pub struct IrProg {
  pub func: IrFunc,
}

pub struct IrFunc {
  pub name: String,
  pub var_cnt: usize,
  pub stmts: Vec<IrStmt>,
}

pub enum IrStmt {
  Ldc(i32),
  Unary(UnaryOp),
  Binary(BinaryOp),
  Load(usize),
  Store(usize),
  Label(usize),
  Bz(usize),
  Bnz(usize),
  Jump(usize),
  Pop,
  Ret,
}

pub fn ast2ir(p: &Prog) -> IrProg {
  IrProg {
    func: func(&p.func),
  }
}

#[derive(Default)]
struct FuncCtx {
  names: Vec<HashMap<String, usize>>,
  stmts: Vec<IrStmt>,
  loops: Vec<(usize, usize)>,
  var_cnt: usize,
  label_cnt: usize,
}

impl FuncCtx {
  fn new_label(&mut self) -> usize {
    (self.label_cnt, self.label_cnt += 1).0
  }

  fn lookup(&self, name: String) -> usize {
    for map in self.names.iter().rev() {
      if let Some(x) = map.get(&name) {
        return *x;
      }
    }
    panic!("variable `{}` not defined in current context", name)
  }
}

fn func(f: &Func) -> IrFunc {
  let mut ctx = FuncCtx::default();
  ctx.names.push(HashMap::new());
  for s in &f.stmts {
    stmt(&mut ctx, s);
  }
  match ctx.stmts.last() {
    Some(IrStmt::Ret) => {}
    _ => {
      ctx.stmts.push(IrStmt::Ldc(0));
      ctx.stmts.push(IrStmt::Ret);
    }
  }
  IrFunc {
    name: f.name.clone(),
    var_cnt: ctx.var_cnt,
    stmts: ctx.stmts,
  }
}

fn block(ctx: &mut FuncCtx, b: &Block) {
  ctx.names.push(HashMap::new());
  for s in &b.0 {
    stmt(ctx, s);
  }
  ctx.names.pop();
}

fn stmt(ctx: &mut FuncCtx, s: &Stmt) {
  match s {
    Stmt::Empty => {}
    Stmt::Ret(e) => {
      expr(ctx, e);
      ctx.stmts.push(IrStmt::Ret);
    }
    Stmt::Def(name, init) => {
      if ctx
        .names
        .last_mut()
        .unwrap()
        .insert(name.clone(), ctx.var_cnt)
        .is_some()
      {
        panic!("variable `{}` redefined in current context", name)
      }
      if let Some(x) = init {
        expr(ctx, x);
        ctx.stmts.push(IrStmt::Store(ctx.var_cnt));
      }
      ctx.var_cnt += 1;
    }
    Stmt::Expr(e) => {
      expr(ctx, e);
      ctx.stmts.push(IrStmt::Pop);
    }
    Stmt::If(cond, t, f) => {
      expr(ctx, cond);
      let (before_f, after_f) = (ctx.new_label(), ctx.new_label());
      ctx.stmts.push(IrStmt::Bz(before_f));
      stmt(ctx, t);
      ctx.stmts.push(IrStmt::Jump(after_f));
      ctx.stmts.push(IrStmt::Label(before_f));
      if let Some(f) = f {
        stmt(ctx, f);
      }
      ctx.stmts.push(IrStmt::Label(after_f));
    }
    Stmt::Block(b) => block(ctx, b),
    Stmt::While(cond, body) => {
      let (before_cond, after_body) = (ctx.new_label(), ctx.new_label());
      ctx.loops.push((after_body, before_cond));
      ctx.stmts.push(IrStmt::Label(before_cond));
      expr(ctx, cond);
      ctx.stmts.push(IrStmt::Bz(after_body));
      stmt(ctx, body);
      ctx.stmts.push(IrStmt::Jump(before_cond));
      ctx.stmts.push(IrStmt::Label(after_body));
      ctx.loops.pop();
    }
    Stmt::DoWhile(body, cond) => {
      let (before_body, before_cond, after_cond) =
        (ctx.new_label(), ctx.new_label(), ctx.new_label());
      ctx.loops.push((after_cond, before_cond));
      ctx.stmts.push(IrStmt::Label(before_body));
      stmt(ctx, body);
      ctx.stmts.push(IrStmt::Label(before_cond));
      expr(ctx, cond);
      ctx.stmts.push(IrStmt::Bnz(before_body));
      ctx.stmts.push(IrStmt::Label(after_cond));
      ctx.loops.pop();
    }
    Stmt::For {
      init,
      cond,
      update,
      body,
    } => {
      ctx.names.push(HashMap::new());
      if let Some(init) = init {
        stmt(ctx, init);
      }
      let (before_cond, before_update, after_body) =
        (ctx.new_label(), ctx.new_label(), ctx.new_label());
      ctx.loops.push((after_body, before_update));
      ctx.stmts.push(IrStmt::Label(before_cond));
      expr(ctx, cond.as_ref().unwrap_or(&Expr::Int(1)));
      ctx.stmts.push(IrStmt::Bz(after_body));
      stmt(ctx, body);
      ctx.stmts.push(IrStmt::Label(before_update));
      if let Some(update) = update {
        expr(ctx, update);
      }
      ctx.stmts.push(IrStmt::Jump(before_cond));
      ctx.stmts.push(IrStmt::Label(after_body));
      ctx.loops.pop();
      ctx.names.pop();
    }
    Stmt::Break => ctx
      .stmts
      .push(IrStmt::Jump(ctx.loops.last().expect("break out of loop").0)),
    Stmt::Continue => ctx.stmts.push(IrStmt::Jump(
      ctx.loops.last().expect("continue out of loop").1,
    )),
  }
}

fn expr(ctx: &mut FuncCtx, e: &Expr) {
  match e {
    Expr::Int(x) => ctx.stmts.push(IrStmt::Ldc(*x)),
    Expr::Unary(op, x) => {
      expr(ctx, x);
      ctx.stmts.push(IrStmt::Unary(*op));
    }
    Expr::Binary(op, l, r) => {
      expr(ctx, l);
      expr(ctx, r);
      ctx.stmts.push(IrStmt::Binary(*op));
    }
    Expr::Var(name) => ctx.stmts.push(IrStmt::Load(ctx.lookup(name.clone()))),
    Expr::Assign(name, r) => {
      expr(ctx, r);
      let id = ctx.lookup(name.clone());
      ctx.stmts.push(IrStmt::Store(id));
      ctx.stmts.push(IrStmt::Load(id));
    }
    Expr::Condition(cond, t, f) => {
      expr(ctx, cond);
      let (before_f, after_f) = (ctx.new_label(), ctx.new_label());
      ctx.stmts.push(IrStmt::Bz(before_f));
      expr(ctx, t);
      ctx.stmts.push(IrStmt::Jump(after_f));
      ctx.stmts.push(IrStmt::Label(before_f));
      expr(ctx, f);
      ctx.stmts.push(IrStmt::Label(after_f));
    }
  }
}
