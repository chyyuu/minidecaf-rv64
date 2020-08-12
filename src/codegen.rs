use std::io::{Result, Write};
use crate::{ast::{UnaryOp::*, BinaryOp::*}, ir::*};

pub fn write_asm(p: &IrProg, w: &mut impl Write) -> Result<()> {
  const SLOT: usize = 8;
  for f in &p.funcs {
    if f.is_decl { continue; }
    writeln!(w, ".global {}", f.name)?;
    writeln!(w, "{}:", f.name)?;
    writeln!(w, "  sd s0, -{}(sp)", (f.var_cnt + 1) * SLOT)?;
    writeln!(w, "  sd ra, -{}(sp)", (f.var_cnt + 2) * SLOT)?;
    writeln!(w, "  add s0, sp, {}", f.param_cnt * SLOT)?;
    writeln!(w, "  add sp, sp, -{}", (f.var_cnt + 2) * SLOT)?;
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
      IrStmt::Binary(op) => {
        writeln!(w, "  ld t0, 0(sp)")?; // rhs
        writeln!(w, "  ld t1, 8(sp)")?; // lhs
        writeln!(w, "  add sp, sp, 8")?;
        match op {
          Add => writeln!(w, "  add t0, t1, t0")?,
          Sub => writeln!(w, "  sub t0, t1, t0")?,
          Mul => writeln!(w, "  mul t0, t1, t0")?,
          Div => writeln!(w, "  div t0, t1, t0")?,
          Mod => writeln!(w, "  rem t0, t1, t0")?,
          Lt => writeln!(w, "  slt t0, t1, t0")?,
          Le => {
            writeln!(w, "  slt t0, t0, t1")?;
            writeln!(w, "  xor t0, t0, 1")?;
          }
          Ge => {
            writeln!(w, "  slt t0, t1, t0")?;
            writeln!(w, "  xor t0, t0, 1")?;
          }
          Gt => writeln!(w, "  slt t0, t0, t1")?,
          Eq => {
            writeln!(w, "  xor t0, t0, t1")?;
            writeln!(w, "  seqz t0, t0")?;
          }
          Ne => {
            writeln!(w, "  xor t0, t0, t1")?;
            writeln!(w, "  snez t0, t0")?;
          }
          And => {
            writeln!(w, "  snez t0, t0")?;
            writeln!(w, "  snez t1, t1")?;
            writeln!(w, "  and t0, t0, t1")?;
          }
          Or => {
            writeln!(w, "  or t0, t0, t1")?;
            writeln!(w, "  snez t0, t0")?;
          }
        };
        writeln!(w, "  sd t0, 0(sp)")?;
      }
      IrStmt::Load(id) => {
          writeln!(w, "  ld t0, -{}(s0)", (id + 1) * SLOT)?;
        writeln!(w, "  sd t0, -8(sp)")?;
        writeln!(w, "  add sp, sp, -8")?;
      }
      IrStmt::Store(id) => {
        writeln!(w, "  ld t0, 0(sp)")?;
        writeln!(w, "  add sp, sp, 8")?;
          writeln!(w, "  sd t0, -{}(s0)", (id + 1) * SLOT)?;
      }
      IrStmt::Label(x) => writeln!(w, ".L.{}.{}:", f.name, x)?,
      IrStmt::Bz(x) => {
        writeln!(w, "  ld t0, 0(sp)")?;
        writeln!(w, "  add sp, sp, 8")?;
        writeln!(w, "  beqz t0, .L.{}.{}", f.name, x)?;
      }
      IrStmt::Bnz(x) => {
        writeln!(w, "  ld t0, 0(sp)")?;
        writeln!(w, "  add sp, sp, 8")?;
        writeln!(w, "  bnez t0, .L.{}.{}", f.name, x)?;
      }
      IrStmt::Jump(x) => writeln!(w, "  j .L.{}.{}", f.name, x)?,
        IrStmt::Call(x) => {
          writeln!(w, "  jal {}", p.funcs[*x].name)?;
          writeln!(w, "  sd a0, -8(sp)")?;
          writeln!(w, "  add sp, sp, -8")?;
        }
      IrStmt::Pop => writeln!(w, "  add sp, sp, 8")?,
        IrStmt::Ret => {
          writeln!(w, "  ld a0, 0(sp)")?;
          writeln!(w, "  mv sp, s0")?;
          writeln!(w, "  ld s0, -{}(sp)", (f.param_cnt + f.var_cnt + 1) * SLOT)?;
          writeln!(w, "  ld ra, -{}(sp)", (f.param_cnt + f.var_cnt + 2) * SLOT)?;
          writeln!(w, "  jr ra")?;
        }
      }
    }
  }
  Ok(())
}
