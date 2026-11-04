use std::collections::BTreeMap;

use crate::cpu::{cpu::SimCPU, op_table::CodeMask};

pub(super) struct Ctx<'a> {
    cpu: &'a mut SimCPU,
    args: u16
}

impl<'a> Ctx<'a> {
    pub fn init(cpu: &'a mut SimCPU, i: u16) -> Self {
        Ctx {
            cpu: cpu,
            args: i
        }
    }
}

pub(super) fn init_functions() -> BTreeMap<CodeMask, fn(&mut Ctx<'_>)> {
    let func_ls: BTreeMap<CodeMask, fn(&mut Ctx<'_>)> = BTreeMap::from(
        [
        (CodeMask::Mccall, mccall as fn(&mut Ctx<'_>)),
        (CodeMask::Clear, clear),
        (CodeMask::Ret, ret),
        (CodeMask::Goto, goto),
        (CodeMask::Call, call),
        (CodeMask::Eq, eq),
        (CodeMask::Neq, neq),
        (CodeMask::Req, req),
        (CodeMask::Set, set),
        (CodeMask::Sum, sum),
        (CodeMask::Mov, mov),
        (CodeMask::Or, or),
        (CodeMask::And, and),
        (CodeMask::Xor, xor),
        (CodeMask::Add, add),
        (CodeMask::Sub, sub),
        (CodeMask::Shfr, shfr),
        (CodeMask::Subl, subl),
        (CodeMask::Shfl, shfl),
        (CodeMask::Rneq, rneq),
        (CodeMask::Addr, addr),
        (CodeMask::Jmp, jmp),
        (CodeMask::Rand, rand),
        (CodeMask::Draw, draw),
        (CodeMask::Prsd, prsd),
        (CodeMask::Nprsd, nprsd),
        (CodeMask::Gettm, gettm),
        (CodeMask::Getin, getin),
        (CodeMask::Settm, settm),
        (CodeMask::Sndtm, sndtm),
        (CodeMask::Iat, iat),
        (CodeMask::Isprt, isprt),
        (CodeMask::Bcd, bcd),
        (CodeMask::Str, str),
        (CodeMask::Load, load),
    ]);

    func_ls
} 

fn mccall(ctx: &mut Ctx) {
    unimplemented!()
}

fn clear(ctx: &mut Ctx) {
    unimplemented!()
}

fn ret(ctx: &mut Ctx) {
    unimplemented!()
}

fn goto(ctx: &mut Ctx) {
    ctx.cpu.set_pc_address(ctx.args & 0x0FFF);
}

fn call(ctx: &mut Ctx) {
    unimplemented!()
}

fn eq(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let n: u8 = (ctx.args & 0xFF) as u8;

    if ctx.cpu.get_value_at_reg(x) == n {
        ctx.cpu.skip_next_instruction();
    }
}

fn neq(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let n: u8 = (ctx.args & 0xFF) as u8;

    if ctx.cpu.get_value_at_reg(x) != n {
        ctx.cpu.skip_next_instruction();
    }
}

fn req(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    if ctx.cpu.get_value_at_reg(x) != ctx.cpu.get_value_at_reg(y) {
        ctx.cpu.skip_next_instruction();
    }    
}

fn set(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let n: u8 = (ctx.args & 0xFF) as u8;

    ctx.cpu.set_value_at_reg(x, n);
}

fn sum(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let n: u8 = (ctx.args & 0xFF) as u8;

    let (sum, _) = ctx.cpu.get_value_at_reg(x).overflowing_add(n);
    
    ctx.cpu.set_value_at_reg(x, sum);
}

fn mov(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    ctx.cpu.set_value_at_reg(x, ctx.cpu.get_value_at_reg(y));
}

fn or(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    ctx.cpu.set_value_at_reg(x, ctx.cpu.get_value_at_reg(x) | ctx.cpu.get_value_at_reg(y));
}

fn and(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    ctx.cpu.set_value_at_reg(x, ctx.cpu.get_value_at_reg(x) & ctx.cpu.get_value_at_reg(y));
}

fn xor(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    ctx.cpu.set_value_at_reg(x, ctx.cpu.get_value_at_reg(x) ^ ctx.cpu.get_value_at_reg(y));
}

fn add(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let (sum, carry) = ctx.cpu.get_value_at_reg(x).overflowing_add(ctx.cpu.get_value_at_reg(y));

    if carry {ctx.cpu.set_value_at_reg(0xF, 1);}
    ctx.cpu.set_value_at_reg(x, sum);
}

fn sub(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let (sub, carry) = ctx.cpu.get_value_at_reg(x).overflowing_sub(ctx.cpu.get_value_at_reg(y));

    if carry {ctx.cpu.set_value_at_reg(0xF, 1);}
    ctx.cpu.set_value_at_reg(x, sub);
}

fn shfr(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    //let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let u: u8 = ctx.cpu.get_value_at_reg(x);
    ctx.cpu.set_value_at_reg(0xF, u & 0x1);
    ctx.cpu.set_value_at_reg(x, u >> 1);
}

fn subl(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let (sub, carry) = ctx.cpu.get_value_at_reg(y).overflowing_sub(ctx.cpu.get_value_at_reg(x));

    if carry {ctx.cpu.set_value_at_reg(0xF, 1);}
    ctx.cpu.set_value_at_reg(x, sub);
}

fn shfl(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    //let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let u: u8 = ctx.cpu.get_value_at_reg(x);
    ctx.cpu.set_value_at_reg(0xF, u & 0x80);
    ctx.cpu.set_value_at_reg(x, u << 1);
}

fn rneq(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    if ctx.cpu.get_value_at_reg(x) != ctx.cpu.get_value_at_reg(y) {
        ctx.cpu.skip_next_instruction();
    }
}

fn addr(ctx: &mut Ctx) {
    let n : u16 = ctx.args & 0xFFF;

    ctx.cpu.set_i_address(n);
}

fn jmp(ctx: &mut Ctx) {
    let n : u16 = ctx.args & 0xFFF;

    ctx.cpu.set_pc_address(n + ctx.cpu.get_value_at_reg(0) as u16);
}

fn rand(ctx: &mut Ctx) {
    unimplemented!()
}

fn draw(ctx: &mut Ctx) {
    unimplemented!()
}

fn prsd(ctx: &mut Ctx) {
    unimplemented!()
}

fn nprsd(ctx: &mut Ctx) {
    unimplemented!()
}

fn gettm(ctx: &mut Ctx) {
    unimplemented!()
}

fn getin(ctx: &mut Ctx) {
    unimplemented!()
}

fn settm(ctx: &mut Ctx) {
    unimplemented!()
}

fn sndtm(ctx: &mut Ctx) {
    unimplemented!()
}

fn iat(ctx: &mut Ctx) {
    unimplemented!()
}

fn isprt(ctx: &mut Ctx) {
    unimplemented!()
}

fn bcd(ctx: &mut Ctx) {
    unimplemented!()
}

fn str(ctx: &mut Ctx) {
    unimplemented!()
}

fn load(ctx: &mut Ctx) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_func() {
        let opcode: u16 = 0x0020;

        let mut cpu = SimCPU::init_cpu();
        let mut ctx: Ctx = Ctx::init(&mut cpu, opcode);

        // V0 = 0x20
        set(&mut ctx);

        assert_eq!(ctx.cpu.get_value_at_reg(0), 0x20)
    }
}