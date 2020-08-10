use std::io::{Result, Write};
use crate::ir::*;

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
      IrStmt::Ret => {
        writeln!(w, "  ld a0, 0(sp)")?;
        writeln!(w, "  add sp, sp, 8")?;
        writeln!(w, "  ret")?;
      }
    }
  }
  Ok(())
}