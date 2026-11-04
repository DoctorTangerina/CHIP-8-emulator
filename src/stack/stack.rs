use crate::memory::memory::Memory;

pub struct Stack {
    top: usize
}

const STACK_BASE: usize = 0xEA0;
const STACK_MAX: usize = 0xF00;

impl Stack {
    pub fn init_stack() -> Self {
        Stack {
            top: STACK_BASE
        }
    }

    fn check_limits(&self) -> bool {
        self.top < STACK_BASE || self.top > STACK_MAX
    }

    pub fn pop(&mut self, mem: &Memory) -> u16 {
        self.top -= 2;
        let addr: u16 = mem.fetch_op(self.top);
        
        if self.check_limits() { panic!("Stack overflow") }
        
        addr
    }

    pub fn push(&mut self, mem: &mut Memory, addr: u16) {
        mem.write(self.top, &addr.to_be_bytes()).expect("Stack overflow");
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
        let mut rc: Vec<u8> = vec![0; 2];

        stack.push(&mut memory, 0x0100);

        memory.read(STACK_BASE, 2, &mut rc).expect("Error reading");
        assert_eq!(rc[0], 0x01);
    }

    #[test]
    fn test_pop() {
        let (mut stack, mut memory) = setup();
        stack.push(&mut memory, 0x0100);

        let addr = stack.pop(&memory);

        assert_eq!(addr, 0x100);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_overflow() {
        let (mut stack, mut memory) = setup();
        stack.top = STACK_MAX;
        stack.push(&mut memory, 0x0100);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_underflow() {
        let (mut stack, mut memory) = setup();
        stack.pop(&mut memory);
    }
}
