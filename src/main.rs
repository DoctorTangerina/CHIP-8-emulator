use emulator_chip_8::graphic::display::Display;

use std::env::args;

fn main() {
    let rom_path = args().nth(1).expect("Usage: emulator_CHIP-8 <rom-path>");

    let (cpu, mem, stack, io, timer, sound) = emulator_chip_8::init_sim(&rom_path);

    Display::run(cpu, mem, stack, io, timer, sound);
}
