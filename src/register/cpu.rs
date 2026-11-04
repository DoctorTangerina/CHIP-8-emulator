use std::collections::BTreeMap;

use crate::register::op_table::{CodeMask, MatchMask};
use crate::register::funcs::{Ctx, init_functions};
use crate::register::registers::{Register, AddrRegister};

pub struct SimCPU {
    Registers: Vec<Register>,
    AddressRegister: AddrRegister,
    ProgramCounter: AddrRegister,
    OperationList: BTreeMap<CodeMask, fn(&mut Ctx<'_>)>
}

impl SimCPU {
    pub fn init_cpu() -> Self {
        SimCPU {
            Registers: (0..0xF).map(|_i: i32| Register::init()).collect(),
            AddressRegister: AddrRegister::init(),
            ProgramCounter: AddrRegister::init(),
            OperationList: init_functions()
        }
    }

    pub fn RunNextOp(&mut self) {
        let addr: u16 = self.ProgramCounter.getAddr();

        // TODO:
        // fetch from memory at `addr` 2 bytes
        //mock instruct
        let code: u16 = 0x6020;
        
        let handler = *self.OperationList
            .get(&MatchMask(code).expect("Operation unknown"))
            .expect("Function not found");
        
        {
            let mut ctx: Ctx = Ctx::init(self, code);
            handler(&mut ctx);
        }

        self.SetPcAddress(addr + 2);
    }

    pub fn GetValueAtReg(&self, i: u8) -> u8 {
        self.Registers[usize::from(i)].getValue()
    }

    pub fn SetValueAtReg(&mut self, i: u8, n: u8) {
        self.Registers[usize::from(i)].setValue(n);
    }

    pub fn SetIAddress(&mut self, i: u16) {
        self.AddressRegister.setAddr(i);
    }

    pub fn SetPcAddress(&mut self, i: u16) {
        self.ProgramCounter.setAddr(i);
    }

    pub fn SkipNextInstruction(&mut self) {
        self.AddressRegister.setAddr(self.AddressRegister.getAddr() + 2);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cpu() {
        let mut cpu: SimCPU = SimCPU::init_cpu();

        cpu.RunNextOp();

        assert_eq!(cpu.GetValueAtReg(0), 0x20);
    }
}