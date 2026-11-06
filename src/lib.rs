use crate::{
    cpu::cpu::SimCPU, graphic::font::init_font, input::io::IO, input::read_code::load_program,
    memory::memory::Memory, sound::sound::Sound, stack::stack::Stack, timer::timer::Timer,
};

pub mod cpu;
pub mod graphic;
pub mod input;
pub mod memory;
pub mod sound;
pub mod stack;
pub mod timer;

pub fn init_sim(rom_path: &String) -> (SimCPU, Memory, Stack, IO, Timer, Sound) {
    let mut cpu = SimCPU::init_cpu();

    cpu.set_pc_address(0x200);

    let mut mem = Memory::init_memory();

    load_program(&mut mem, &rom_path).expect("Could not load rom");

    init_font(&mut mem);

    (
        cpu,
        mem,
        Stack::init_stack(),
        IO::init_io(),
        Timer::init_timer(),
        Sound::init_sound(),
    )
}
