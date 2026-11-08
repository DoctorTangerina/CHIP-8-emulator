use std::collections::BTreeMap;

use crate::{
    cpu::{cpu::SimCPU, op_table::CodeMask},
    graphic::screen::Screen,
    input::io::IO,
    memory::memory::Memory,
    sound::sound::Sound,
    stack::stack::Stack,
    timer::timer::Timer,
};

pub(super) struct Ctx<'a> {
    cpu: &'a mut SimCPU,
    stack: &'a mut Stack,
    memory: &'a mut Memory,
    screen: &'a mut Screen,
    io: &'a mut IO,
    timer: &'a mut Timer,
    sound: &'a mut Sound,
    args: u16,
}

impl<'a> Ctx<'a> {
    pub fn init(
        cpu: &'a mut SimCPU,
        stack: &'a mut Stack,
        memory: &'a mut Memory,
        screen: &'a mut Screen,
        io: &'a mut IO,
        timer: &'a mut Timer,
        sound: &'a mut Sound,
        i: u16,
    ) -> Self {
        Self {
            cpu: cpu,
            stack: stack,
            memory: memory,
            screen: screen,
            io: io,
            timer: timer,
            sound: sound,
            args: i,
        }
    }
}

pub(super) fn init_functions() -> BTreeMap<CodeMask, fn(&mut Ctx<'_>)> {
    let func_ls: BTreeMap<CodeMask, fn(&mut Ctx<'_>)> = BTreeMap::from([
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
    goto(ctx);
}

fn clear(ctx: &mut Ctx) {
    ctx.screen.clear(ctx.memory);
}

fn ret(ctx: &mut Ctx) {
    ctx.cpu.set_pc_address(ctx.stack.pop(ctx.memory));
}

fn goto(ctx: &mut Ctx) {
    ctx.cpu.set_pc_address(ctx.args & 0x0FFF);
}

fn call(ctx: &mut Ctx) {
    ctx.stack.push(ctx.memory, ctx.cpu.get_pc_address());
    ctx.cpu.set_pc_address(ctx.args & 0x0FFF);
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

    if ctx.cpu.get_value_at_reg(x) == ctx.cpu.get_value_at_reg(y) {
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

    ctx.cpu.set_value_at_reg(0xF, 0);
    ctx.cpu
        .set_value_at_reg(x, ctx.cpu.get_value_at_reg(x) | ctx.cpu.get_value_at_reg(y));
}

fn and(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    ctx.cpu.set_value_at_reg(0xF, 0);
    ctx.cpu
        .set_value_at_reg(x, ctx.cpu.get_value_at_reg(x) & ctx.cpu.get_value_at_reg(y));
}

fn xor(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    ctx.cpu.set_value_at_reg(0xF, 0);
    ctx.cpu
        .set_value_at_reg(x, ctx.cpu.get_value_at_reg(x) ^ ctx.cpu.get_value_at_reg(y));
}

fn add(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let (sum, carry) = ctx
        .cpu
        .get_value_at_reg(x)
        .overflowing_add(ctx.cpu.get_value_at_reg(y));

    ctx.cpu.set_value_at_reg(x, sum);
    ctx.cpu.set_value_at_reg(0xF, u8::from(carry));
}

fn sub(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let (sub, borrow) = ctx
        .cpu
        .get_value_at_reg(x)
        .overflowing_sub(ctx.cpu.get_value_at_reg(y));

    ctx.cpu.set_value_at_reg(x, sub);
    ctx.cpu.set_value_at_reg(0xF, u8::from(!borrow));
}

fn shfr(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let u: u8 = ctx.cpu.get_value_at_reg(y);
    ctx.cpu.set_value_at_reg(x, u >> 1);
    ctx.cpu.set_value_at_reg(0xF, u & 0x1);
}

fn subl(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let (sub, borrow) = ctx
        .cpu
        .get_value_at_reg(y)
        .overflowing_sub(ctx.cpu.get_value_at_reg(x));

    ctx.cpu.set_value_at_reg(x, sub);
    ctx.cpu.set_value_at_reg(0xF, u8::from(!borrow));
}

fn shfl(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    let u: u8 = ctx.cpu.get_value_at_reg(y);
    ctx.cpu.set_value_at_reg(x, u << 1);
    ctx.cpu.set_value_at_reg(0xF, u8::from(u & 0x80 != 0));
}

fn rneq(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let y: u8 = ((ctx.args & 0xF0) >> 4) as u8;

    if ctx.cpu.get_value_at_reg(x) != ctx.cpu.get_value_at_reg(y) {
        ctx.cpu.skip_next_instruction();
    }
}

fn addr(ctx: &mut Ctx) {
    let n: u16 = ctx.args & 0xFFF;

    ctx.cpu.set_i_address(n);
}

fn jmp(ctx: &mut Ctx) {
    let n: u16 = ctx.args & 0xFFF;

    ctx.cpu
        .set_pc_address(n + ctx.cpu.get_value_at_reg(0) as u16);
}

fn rand(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let n: u8 = (ctx.args & 0xFF) as u8;

    ctx.cpu.set_value_at_reg(x, rand::random::<u8>() & n);
}

fn draw(ctx: &mut Ctx) {
    if !ctx.cpu.is_waiting() {
       ctx.timer.clear_vblank();
        ctx.cpu.wait();
        return;
    }
    if !ctx.timer.get_vblank() {
        return;
    }
    ctx.cpu.unblock();

    let x = ((ctx.args & 0xF00) >> 8) as u8;
    let y = ((ctx.args & 0xF0) >> 4) as u8;
    let height = (ctx.args & 0xF) as u8;
    let collision = ctx.screen.draw(
        ctx.cpu.get_value_at_reg(x),
        ctx.cpu.get_value_at_reg(y),
        ctx.memory,
        ctx.cpu.get_i_address(),
        height,
    );
    ctx.cpu.set_value_at_reg(0xF, u8::from(collision));
}

fn prsd(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let key: u8 = ctx.cpu.get_value_at_reg(x);

    if ctx.io.is_key_pressed(key) {
        ctx.cpu.skip_next_instruction();
    }
}

fn nprsd(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;
    let key: u8 = ctx.cpu.get_value_at_reg(x);

    if !ctx.io.is_key_pressed(key) {
        ctx.cpu.skip_next_instruction();
    }
}

fn gettm(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;

    ctx.cpu.set_value_at_reg(x, ctx.timer.get_timer());
}

fn getin(ctx: &mut Ctx) {
    if !ctx.cpu.is_waiting() {
        ctx.cpu.wait();
        ctx.io.set_read();
    } else if !ctx.io.is_reading() {
        let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;

        ctx.cpu.unblock();
        ctx.cpu.set_value_at_reg(x, ctx.io.get_key_read());
    }
}

fn settm(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;

    ctx.timer.set_timer(ctx.cpu.get_value_at_reg(x));
}

fn sndtm(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;

    ctx.sound.set_timer(ctx.cpu.get_value_at_reg(x));
}

fn iat(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;

    ctx.cpu
        .set_i_address(ctx.cpu.get_i_address() + (ctx.cpu.get_value_at_reg(x) as u16));
}

fn isprt(ctx: &mut Ctx) {
    let x: u8 = ((ctx.args & 0xF00) >> 8) as u8;

    ctx.cpu
        .set_i_address(ctx.cpu.get_value_at_reg(x) as u16 * 5);
}

fn bcd(ctx: &mut Ctx) {
    let x = ((ctx.args & 0xF00) >> 8) as u8;
    let value = ctx.cpu.get_value_at_reg(x);

    let digits = [value / 100, (value / 10) % 10, value % 10];
    let i = ctx.cpu.get_i_address() as usize;

    ctx.memory
        .write(i, &digits)
        .expect("Could not write BCD digits to memory");
}

fn str(ctx: &mut Ctx) {
    let x = ((ctx.args & 0xF00) >> 8) as u8;
    let index = ctx.cpu.get_i_address() as usize;

    let mut v: Vec<u8> = vec![0; (x + 1) as usize];
    for i in 0..=x {
        v[i as usize] = ctx.cpu.get_value_at_reg(i);
    }

    ctx.memory
        .write(index, &v)
        .expect("Couldn't write to memory");

    ctx.cpu.set_i_address((index + usize::from(x) + 1) as u16);
}

fn load(ctx: &mut Ctx) {
    let x = ((ctx.args & 0xF00) >> 8) as u8;
    let index = ctx.cpu.get_i_address() as usize;

    let mut v: Vec<u8> = vec![0; (x + 1) as usize];
    ctx.memory
        .read(index, v.len(), &mut v)
        .expect("Couldn't read from memory");

    ctx.cpu.set_i_address((index + usize::from(x) + 1) as u16);
    for i in 0..=x {
        ctx.cpu.set_value_at_reg(i, v[i as usize]);
    }
}

#[cfg(test)]
mod tests {
    use winit::keyboard::KeyCode;

    use super::*;

    fn run_with_registers(
        handler: fn(&mut Ctx<'_>),
        opcode: u16,
        registers: &[(u8, u8)],
    ) -> SimCPU {
        let (mut cpu, mut mem, mut stack, mut screen, mut io, mut timer, mut sound) = init();
        for &(register, value) in registers {
            cpu.set_value_at_reg(register, value);
        }

        let mut ctx: Ctx = Ctx::init(
            &mut cpu,
            &mut stack,
            &mut mem,
            &mut screen,
            &mut io,
            &mut timer,
            &mut sound,
            opcode,
        );
        handler(&mut ctx);
        drop(ctx);
        cpu
    }

    fn init() -> (SimCPU, Memory, Stack, Screen, IO, Timer, Sound) {
        (
            SimCPU::init_cpu(),
            Memory::init_memory(),
            Stack::init_stack(),
            Screen::default(),
            IO::init_io(),
            Timer::init_timer(),
            Sound::init_sound(),
        )
    }

    #[test]
    fn test_set_func() {
        let opcode: u16 = 0x0020;

        let (mut cpu, mut mem, mut stack, mut screen, mut io, mut timer, mut sound) = init();
        let mut ctx: Ctx = Ctx::init(
            &mut cpu,
            &mut stack,
            &mut mem,
            &mut screen,
            &mut io,
            &mut timer,
            &mut sound,
            opcode,
        );

        // V0 = 0x20
        set(&mut ctx);

        assert_eq!(ctx.cpu.get_value_at_reg(0), 0x20)
    }

    #[test]
    fn arithmetic_and_shift_opcodes_set_flags_for_both_outcomes() {
        let cpu = run_with_registers(add, 0x8014, &[(0, 0xFF), (1, 1)]);
        assert_eq!(cpu.get_value_at_reg(0), 0);
        assert_eq!(cpu.get_value_at_reg(0xF), 1);

        let cpu = run_with_registers(add, 0x8014, &[(0, 2), (1, 1), (0xF, 1)]);
        assert_eq!(cpu.get_value_at_reg(0), 3);
        assert_eq!(cpu.get_value_at_reg(0xF), 0);

        let cpu = run_with_registers(sub, 0x8015, &[(0, 5), (1, 3)]);
        assert_eq!(cpu.get_value_at_reg(0), 2);
        assert_eq!(cpu.get_value_at_reg(0xF), 1);

        let cpu = run_with_registers(sub, 0x8015, &[(0, 3), (1, 5)]);
        assert_eq!(cpu.get_value_at_reg(0), 0xFE);
        assert_eq!(cpu.get_value_at_reg(0xF), 0);

        let cpu = run_with_registers(subl, 0x8017, &[(0, 3), (1, 5)]);
        assert_eq!(cpu.get_value_at_reg(0), 2);
        assert_eq!(cpu.get_value_at_reg(0xF), 1);

        let cpu = run_with_registers(subl, 0x8017, &[(0, 5), (1, 3)]);
        assert_eq!(cpu.get_value_at_reg(0), 0xFE);
        assert_eq!(cpu.get_value_at_reg(0xF), 0);

        let cpu = run_with_registers(shfl, 0x800E, &[(0, 0x81)]);
        assert_eq!(cpu.get_value_at_reg(0), 0x02);
        assert_eq!(cpu.get_value_at_reg(0xF), 1);

        let cpu = run_with_registers(shfl, 0x800E, &[(0, 0x01), (0xF, 1)]);
        assert_eq!(cpu.get_value_at_reg(0), 0x02);
        assert_eq!(cpu.get_value_at_reg(0xF), 0);
    }

    #[test]
    fn test_getin_func() {
        let opcode: u16 = 0xF10A;

        let (mut cpu, mut mem, mut stack, mut screen, mut io, mut timer, mut sound) = init();
        let mut ctx: Ctx = Ctx::init(
            &mut cpu,
            &mut stack,
            &mut mem,
            &mut screen,
            &mut io,
            &mut timer,
            &mut sound,
            opcode,
        );

        assert_eq!(ctx.cpu.is_waiting(), false);

        getin(&mut ctx);

        assert_eq!(ctx.cpu.is_waiting(), true);

        ctx.io.set_key_pressed(KeyCode::Digit4, true);

        getin(&mut ctx);

        assert_eq!(ctx.cpu.is_waiting(), false);
        assert_eq!(ctx.cpu.get_value_at_reg(1), 0xC);
    }
}
