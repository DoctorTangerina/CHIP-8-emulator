use crate::{
    cpu::cpu::SimCPU, input::io::IO, memory::memory::Memory, sound::sound::Sound,
    stack::stack::Stack, timer::timer::Timer,
};

pub mod cpu;
pub mod graphic;
pub mod input;
pub mod memory;
pub mod sound;
pub mod stack;
pub mod timer;

pub fn init_sim() -> (SimCPU, Memory, Stack, IO, Timer, Sound) {
    (
        SimCPU::init_cpu(),
        Memory::init_memory(),
        Stack::init_stack(),
        IO::init_io(),
        Timer::init_timer(),
        Sound::init_sound(),
    )
}
