use crate::ast::*;
use std::collections::{hash_map::Entry, HashMap};

pub struct IrProg {
  pub funcs: Vec<IrFunc>,
  pub globs: Vec<(String, Option<i32>)>,
}

pub struct IrFunc {
  pub name: String,
  pub param_cnt: usize,
  pub var_cnt: usize,
  pub is_decl: bool,
  pub stmts: Vec<IrStmt>,
}

pub enum IrStmt {
  Ldc(i32),
  Unary(UnaryOp),
  Binary(BinaryOp),
  Load(usize),
  Store(usize),
  LoadGlobal(usize),
  StoreGlobal(usize),
  Label(usize),
  Bz(usize),
  Bnz(usize),
  Jump(usize),
  Call(usize),
  Pop,
  Ret,
}

pub fn ast2ir(p: &Prog) -> IrProg {
  let mut glob2id = HashMap::new();
  let mut globs = Vec::new();
  for g in &p.globs {
    match glob2id.entry(g.0.clone()) {
      Entry::Vacant(v) => {
        v.insert(globs.len());
        globs.push(g.clone());
      }
      Entry::Occupied(o) => {
        let old = &mut globs[*o.get()];
        if old.1.is_none() {
          *old = g.clone();
        } else if g.1.is_some() {
          panic!("global variable `{}` redefined in current context", g.0)
        }
      }
    }
  }
  let mut func2id = HashMap::new();
  let mut funcs = Vec::new();
  for f in &p.funcs {
    match func2id.entry(f.name.clone()) {
      Entry::Vacant(v) => {
        v.insert(funcs.len());
        funcs.push(func(&func2id, &glob2id, f));
      }
      Entry::Occupied(o) => {
        let old = &mut funcs[*o.get()];
        if old.is_decl {
          *old = func(&func2id, &glob2id, f);
        } else if f.stmts.is_some() {
          panic!("function `{}` redefined in current context", f.name)
        }
      }
    }
  }
  IrProg { funcs, globs }
}

struct FuncCtx<'a> {
  names: Vec<HashMap<String, usize>>,
  stmts: Vec<IrStmt>,
  loops: Vec<(usize, usize)>,
  func2id: &'a HashMap<String, usize>,
  glob2id: &'a HashMap<String, usize>,
  var_cnt: usize,
  label_cnt: usize,
}

impl<'a> FuncCtx<'a> {
  fn new_label(&mut self) -> usize {
    (self.label_cnt, self.label_cnt += 1).0
  }

  fn lookup(&self, name: String) -> (bool, usize) {
    for map in self.names.iter().rev() {
      if let Some(x) = map.get(&name) {
        return (false, *x);
      }
    }
    if let Some(x) = self.glob2id.get(&name) {
      return (true, *x);
    }
    panic!("variable `{}` not defined in current context", name)
  }
}

fn func(func2id: &HashMap<String, usize>, glob2id: &HashMap<String, usize>, f: &Func) -> IrFunc {
  let mut ctx = FuncCtx {
    names: vec![HashMap::new()],
    stmts: Vec::new(),
    loops: Vec::new(),
    func2id,
    glob2id,
    var_cnt: 0,
    label_cnt: 0,
  };
  for p in &f.params {
    stmt(&mut ctx, &Stmt::Def(p.to_string(), None))
  }
  if let Some(st) = &f.stmts {
    if let Stmt::Block(Block(stmts)) = st {
      for s in stmts {
        stmt(&mut ctx, s);
      }
      match ctx.stmts.last() {
        Some(IrStmt::Ret) => {}
        _ => {
          ctx.stmts.push(IrStmt::Ldc(0));
          ctx.stmts.push(IrStmt::Ret);
        }
      }
    } else {
      stmt(&mut ctx, st);
      match ctx.stmts.last() {
        Some(IrStmt::Ret) => {}
        _ => {
          ctx.stmts.push(IrStmt::Ldc(0));
          ctx.stmts.push(IrStmt::Ret);
        }
      }
    }
  }
  IrFunc {
    name: f.name.clone(),
    param_cnt: f.params.len(),
    var_cnt: ctx.var_cnt - f.params.len(),
    is_decl: f.stmts.is_none(),
    stmts: ctx.stmts,
  }
}

fn block<'a>(ctx: &mut FuncCtx<'a>, b: &Block) {
  ctx.names.push(HashMap::new());
  for s in &b.0 {
    stmt(ctx, s);
  }
  ctx.names.pop();
}

fn stmt<'a>(ctx: &mut FuncCtx<'a>, s: &Stmt) {
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

fn expr<'a>(ctx: &mut FuncCtx<'a>, e: &Expr) {
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
    Expr::Var(name) => {
      let (is_glob, id) = ctx.lookup(name.clone());
      ctx.stmts.push(if is_glob {
        IrStmt::LoadGlobal(id)
      } else {
        IrStmt::Load(id)
      });
    }
    Expr::Assign(name, r) => {
      expr(ctx, r);
      let (is_glob, id) = ctx.lookup(name.clone());
      ctx.stmts.push(if is_glob {
        IrStmt::StoreGlobal(id)
      } else {
        IrStmt::Store(id)
      });
      ctx.stmts.push(if is_glob {
        IrStmt::LoadGlobal(id)
      } else {
        IrStmt::Load(id)
      });
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
    Expr::Call(func, args) => {
      let id = *ctx
        .func2id
        .get(func)
        .expect("function not defined in current context");
      for a in args {
        expr(ctx, a);
      }
      ctx.stmts.push(IrStmt::Call(id));
    }
  }
}
