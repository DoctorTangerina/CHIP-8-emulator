use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::cpu::funcs::{Ctx, init_functions};
use crate::cpu::op_table::{CodeMask, match_mask};
use crate::cpu::registers::{AddrRegister, Register};

use crate::graphic::screen::Screen;
use crate::input::io::IO;
use crate::memory::memory::Memory;
use crate::sound::sound::Sound;
use crate::stack::stack::Stack;
use crate::timer::timer::Timer;

static TRACE_INSTRUCTIONS: OnceLock<bool> = OnceLock::new();

#[derive(Debug)]
pub struct SimCPU {
    regs: Vec<Register>,
    addr_reg: AddrRegister,
    prog_counter: AddrRegister,
    op_list: BTreeMap<CodeMask, fn(&mut Ctx<'_>)>,
    waiting: bool,
}

impl SimCPU {
    pub fn init_cpu() -> Self {
        Self {
            regs: (0..0x10).map(|_i: i32| Register::init()).collect(),
            addr_reg: AddrRegister::init(),
            prog_counter: AddrRegister::init(),
            op_list: init_functions(),
            waiting: false,
        }
    }

    pub fn is_waiting(&self) -> bool {
        self.waiting
    }

    pub fn wait(&mut self) {
        self.waiting = true;
    }

    pub fn unblock(&mut self) {
        self.waiting = false;
    }

    pub fn run_next_op(
        &mut self,
        stack: &mut Stack,
        mem: &mut Memory,
        screen: &mut Screen,
        io: &mut IO,
        timer: &mut Timer,
        sound: &mut Sound,
    ) {
        let addr: u16 = self.get_pc_address();
        self.set_pc_address(addr + 2);

        let code: u16 = mem.fetch_op(usize::from(addr));

        let operation = match_mask(code).expect("Operation unknown");
        let handler = *self.op_list.get(&operation).expect("Function not found");

        if *TRACE_INSTRUCTIONS.get_or_init(|| std::env::var_os("CHIP8_TRACE").is_some()) {
            println!(
                "instruction: {:#06X} (handler: {:#?}), address: {:#06X}",
                code, operation, addr
            );
        }

        {
            let mut ctx: Ctx = Ctx::init(self, stack, mem, screen, io, timer, sound, code);
            handler(&mut ctx);
        }

        if self.is_waiting() {
            self.set_pc_address(addr);
        }
    }

    pub fn get_value_at_reg(&self, i: u8) -> u8 {
        self.regs[usize::from(i)].get_value()
    }

    pub fn set_value_at_reg(&mut self, i: u8, n: u8) {
        self.regs[usize::from(i)].set_value(n);
    }

    pub fn get_i_address(&self) -> u16 {
        self.addr_reg.get_addr()
    }

    pub fn set_i_address(&mut self, i: u16) {
        self.addr_reg.set_addr(i);
    }

    pub fn get_pc_address(&self) -> u16 {
        self.prog_counter.get_addr()
    }

    pub fn set_pc_address(&mut self, i: u16) {
        self.prog_counter.set_addr(i);
    }

    pub fn skip_next_instruction(&mut self) {
        self.set_pc_address(self.get_pc_address() + 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let code: Vec<u8> = vec![
            0x60, 0x20, // V0 = 0x20
            0x61, 0x12, // V1 = 0x12
            0x80, 0x14, // V0 += V1
        ];

        let mut cpu: SimCPU = SimCPU::init_cpu();
        let mut mem: Memory = Memory::init_memory();
        let mut stack: Stack = Stack::init_stack();
        let mut screen = Screen::default();
        let mut io: IO = IO::init_io();
        let mut timer: Timer = Timer::init_timer();
        let mut sound: Sound = Sound::init_sound();

        mem.write(0, &code).expect("Couldn't initialize code");

        let mut i: usize = usize::from(cpu.get_pc_address());
        while i < code.len() {
            cpu.run_next_op(
                &mut stack,
                &mut mem,
                &mut screen,
                &mut io,
                &mut timer,
                &mut sound,
            );

            i = usize::from(cpu.get_pc_address());
        }

        assert_eq!(
            cpu.get_value_at_reg(1),
            0x12,
            "value in reg {} is {}, expected {}",
            1,
            cpu.get_value_at_reg(1),
            0x12
        );
        assert_eq!(cpu.get_value_at_reg(0), 0x32);
        assert_eq!(cpu.get_value_at_reg(0xF), 0);
        assert_eq!(i, 6); // 6 as in next operations
    }

    #[test]
    fn test_code_flow() {
        let code: Vec<u8> = vec![
            0x60, 0x20, // V0 = 0x20
            0x30, 0x20, // if (V0 == 0x20) skip
            0x70, 0x12, // V0 += 0x12
            0x40, 0x20, // if (V0 != 0x20) skip
            0x10, 0x0E, // goto 0x00E
            0x81, 0x00, // V1 = V0
        ];

        let mut cpu: SimCPU = SimCPU::init_cpu();
        let mut mem: Memory = Memory::init_memory();
        let mut stack: Stack = Stack::init_stack();
        let mut screen = Screen::default();
        let mut io: IO = IO::init_io();
        let mut timer: Timer = Timer::init_timer();
        let mut sound: Sound = Sound::init_sound();

        mem.write(0, &code).expect("Couldn't initialize code");

        let mut i: usize = usize::from(cpu.get_pc_address());
        while i < code.len() {
            cpu.run_next_op(
                &mut stack,
                &mut mem,
                &mut screen,
                &mut io,
                &mut timer,
                &mut sound,
            );

            i = usize::from(cpu.get_pc_address());
        }

        assert_eq!(cpu.get_value_at_reg(0), 0x20);
        assert_eq!(cpu.get_value_at_reg(1), 0);
        assert_eq!(cpu.get_pc_address(), 0xE);
    }
}
