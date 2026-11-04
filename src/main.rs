use emulator_CHIP_8::cpu::cpu::SimCPU;
use emulator_CHIP_8::graphic::display::Display;
use emulator_CHIP_8::input::read_code::load_program;
use emulator_CHIP_8::memory::memory::Memory;
use emulator_CHIP_8::stack::stack::Stack;

use std::env::args;

fn main() {
    let mut cpu = SimCPU::init_cpu();
    let mut mem = Memory::init_memory();
    let stack: Stack = Stack::init_stack();

    let rom_path = args().nth(1).expect("Usage: emulator_CHIP-8 <rom-path>");

    load_program(&mut mem, &rom_path).expect("Could not load rom");
    cpu.set_pc_address(0x200);
    Display::run(cpu, mem, stack);
}
