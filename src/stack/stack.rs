use crate::memory::memory::Memory;

pub struct Stack {
    base: usize,
    max: usize,
    top: usize
}

impl Stack {
    pub fn init_stack() -> Self {
        Stack {
            base: 0xEA0,
            max: 0xEFF,
            top: 0xEA0
        }
    }

    fn check_limits(&self) -> bool {
        self.top < self.base || self.top > self.max
    }

    pub fn pop(&mut self, mem: &Memory) -> u16 {
        self.top -= 2;
        let addr: u16 = mem.fetch_op(self.top);
        
        if self.check_limits() { panic!("Stack overflow") }
        
        addr
    }

    pub fn push(&mut self, mem: &mut Memory, addr: u16) {
        let v: Vec<u8> = vec![(addr & 0xFF00 >> 8) as u8, addr as u8];
        mem.write(self.top, &v).expect("Stack overflow");
        self.top += 2;

        if self.check_limits() { panic!("Stack overflow") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Stack, Memory) {
        (Stack::init_stack(), Memory::init_memory())
    }

    #[test]
    fn test_push() {
        let (mut stack, mut memory) = setup();

        stack.push(&mut memory, 0x0100);

        let mut rc: Vec<u8> = vec![0; 2];
        memory.read(stack.base, 2, &mut rc).expect("Error reading");
        print!("stack top: {}, {:?}", stack.top, rc);
        assert_eq!(rc[0], 0x01);
    }

    #[test]
    fn test_pop() {

    }

    #[test]
    fn test_overflow() {

    }

    #[test]
    fn test_underflow() {

    }
}
