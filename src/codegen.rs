use std::io::{Result, Write};
use crate::{ast::UnaryOp::*, ir::*};

pub fn write_asm(p: &IrProg, w: &mut impl Write) -> Result<()> {
  let f = &p.func;
  writeln!(w, ".global {}", f.name)?;
  writeln!(w, "{}:", f.name)?;
  for s in &f.stmts {
    match s {
      IrStmt::Ldc(x) => {
        writeln!(w, "  li t0, {}", x)?;
        writeln!(w, "  sd t0, -8(sp)")?;
        writeln!(w, "  add sp, sp, -8")?;
      }
      IrStmt::Unary(op) => {
        writeln!(w, "  ld t0, 0(sp)")?;
        let op = match op { Neg => "neg", BNot => "not", LNot => "seqz" };
        writeln!(w, "  {} t0, t0", op)?;
        writeln!(w, "  sd t0, 0(sp)")?;
      }
      IrStmt::Ret => {
        writeln!(w, "  ld a0, 0(sp)")?;
        writeln!(w, "  add sp, sp, 8")?;
        writeln!(w, "  ret")?;
      }
    }
  }
  Ok(())
}
