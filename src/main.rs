use emulator_chip_8::graphic::display::Display;
use emulator_chip_8::input::read_code::load_program;

use std::env::args;

fn main() {
    let (mut cpu, mut mem, stack, io, timer, sound) = emulator_chip_8::init_sim();

    let rom_path = args().nth(1).expect("Usage: emulator_CHIP-8 <rom-path>");

    load_program(&mut mem, &rom_path).expect("Could not load rom");
    
    Display::run(cpu, mem, stack, io, timer, sound);
}
