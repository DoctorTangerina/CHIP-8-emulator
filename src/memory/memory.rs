const MEMORY_TOTAL_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Memory {
    _mem: Box<[u8; MEMORY_TOTAL_SIZE]>,
}

impl Memory {
    pub fn init_memory() -> Self {
        Self {
            _mem: vec![0u8; MEMORY_TOTAL_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    pub fn fetch_op(&self, addr: usize) -> u16 {
        ((self._mem[addr] as u16) << 8) | self._mem[addr + 1] as u16
    }

    pub fn write(&mut self, start: usize, code: &[u8]) -> std::io::Result<()> {
        let end = start
            .checked_add(code.len())
            .filter(|&end| end <= MEMORY_TOTAL_SIZE)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Out of memory")
            })?;

        self._mem[start..end].copy_from_slice(code);
        Ok(())
    }

    pub fn read(&self, at: usize, size: usize, rc: &mut Vec<u8>) -> std::io::Result<()> {
        let end = at
            .checked_add(size)
            .filter(|&end| end <= MEMORY_TOTAL_SIZE)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Out of memory")
            })?;

        rc[0..size].copy_from_slice(&self._mem[at..end]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8 , 9, 10];

    fn init() -> Memory {
        Memory::init_memory()
    }

    #[test]
    fn test_write() {
        let mut mem = init();
        
        mem
            .write(0, &BYTES)
            .expect("Error writing bytes");

        assert!(mem._mem[0..10] == BYTES);
    }

    #[test]
    fn test_read_exact() {
        let mut mem = init();
        let mut rc: Vec<u8> = vec![0; 10];

        mem._mem[0..10].copy_from_slice(&BYTES);

        mem
            .read(0, 10, &mut rc)
            .expect("Couldn't read from memory");

        assert!(mem._mem[0..10] == rc);
    }

    #[test]
    #[should_panic(expected = "range end index 11 out of range for slice of length 10")]
    fn test_read_more_than_size() {
        let mut mem = init();
        let mut rc: Vec<u8> = vec![0; 10];

        mem._mem[0..10].copy_from_slice(&BYTES);

        mem
            .read(0, 11, &mut rc)
            .expect("Couldn't read from memory");
    }

    #[test]
    fn test_read_less_than_size() {
        let mut mem = init();
        let mut rc: Vec<u8> = vec![0; 10];

        mem._mem[0..10].copy_from_slice(&BYTES);

        mem
            .read(0, 9, &mut rc)
            .expect("Couldn't read from memory");

        assert!(mem._mem[0..9] == rc[0..9]);
        assert!(mem._mem[9] != rc[9]);
    }
}