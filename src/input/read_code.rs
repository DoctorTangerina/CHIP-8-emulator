use crate::memory::memory::Memory;

//pub fn load;

pub fn load_program(mem: &mut Memory, path: &str) -> std::io::Result<()> {
    let code = std::fs::read(path)?;
    mem.write(0x200, &code)
}
