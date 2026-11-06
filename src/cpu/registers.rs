#[derive(Debug)]
pub(super) struct Register {
    byte: u8,
}

#[derive(Debug)]
pub(super) struct AddrRegister {
    addr: u16,
}

impl Register {
    pub fn init() -> Self {
        Self { byte: 0 }
    }

    pub fn get_value(&self) -> u8 {
        self.byte
    }

    pub fn set_value(&mut self, i: u8) {
        self.byte = i;
    }
}

impl AddrRegister {
    pub fn init() -> Self {
        Self { addr: 0 }
    }

    pub fn get_addr(&self) -> u16 {
        self.addr
    }

    pub fn set_addr(&mut self, i: u16) {
        self.addr = i & 0xFFF;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_starts_at_zero() {
        let reg = Register::init();

        assert_eq!(reg.get_value(), 0);
    }

    #[test]
    fn register_receives_value() {
        let mut reg = Register::init();

        reg.set_value(0x2A);

        assert_eq!(reg.get_value(), 0x2A);
    }

    // u8 can't overflow without error,
    // so it nevers overflow naturally,
    // only with call to `overflowing_add`,
    // that case will be treated directly into the operations
    /* #[test]
    fn register_overflows_value() {
        let mut reg: Register = Register::init();
        let mut reg2: Register = Register::init();

        reg.set_value(1);
        reg2.set_value(0xFF);

        reg.set_value(reg.get_value() + reg2.get_value());

        assert_eq!(reg.get_value(), 0);
    }*/

    #[test]
    fn addr_starts_at_zero() {
        let addr_reg = AddrRegister::init();

        assert_eq!(addr_reg.get_addr(), 0);
    }

    #[test]
    fn addr_receives_address() {
        let mut addr_reg = AddrRegister::init();

        addr_reg.set_addr(0xF2A);

        assert_eq!(addr_reg.get_addr(), 0xF2A);
    }

    #[test]
    fn addr_overflows_address() {
        let mut addr_reg = AddrRegister::init();

        addr_reg.set_addr(0xFFFF);

        assert_eq!(addr_reg.get_addr(), 0x0FFF);
    }
}
