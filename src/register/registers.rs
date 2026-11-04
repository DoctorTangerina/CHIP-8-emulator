pub(super) struct Register {
    byte: u8
}

pub(super) struct AddrRegister {
    addr: u16
}

impl Register {
    pub fn init() -> Self {
        Register { byte: 0 }
    }

    pub fn getValue(&self) -> u8 {
        self.byte
    }

    pub fn setValue(&mut self, i: u8) {
        self.byte = i;
    }
}

impl AddrRegister {
    pub fn init() -> Self {
        AddrRegister { addr: 0 }
    }

    pub fn getAddr(&self) -> u16 {
        self.addr
    }

    pub fn setAddr(&mut self, i: u16) {
        self.addr = i;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_starts_at_zero() {
        let reg = Register::init();

        assert_eq!(reg.getValue(), 0);
    }
}