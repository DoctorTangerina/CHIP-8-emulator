use emulator_CHIP_8::cpu::cpu::SimCPU;
use emulator_CHIP_8::input::read_code::load_program;
use emulator_CHIP_8::memory::memory::Memory;
use emulator_CHIP_8::stack::stack::Stack;

fn main() {
    let mut cpu = SimCPU::init_cpu();
    let mut mem = Memory::init_memory();
    let mut stack: Stack = Stack::init_stack();

    load_program(&mut mem, "roms/PONG").expect("Could not load rom");

    cpu.set_pc_address(0x200);
    
    loop {
        cpu.run_next_op(&mut mem);
    }
}
