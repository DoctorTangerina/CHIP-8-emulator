const MEMORY_TOTAL_SIZE: usize = 4096;

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

        rc.copy_from_slice(&self._mem[at..end]);
        Ok(())
    }
}
