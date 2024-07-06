use crate::bus::Bus;
use crate::cpu::instructions;
use std::collections::HashMap;

pub struct CPU {
    pub pc: u32,
    pb: u32, // program break location
    pub acc_hi: u32,
    pub acc_lo: u32,
    pub r: [u32; 32], //registers
    bus: Bus,
    reg: HashMap<u32, fn(&mut CPU, (usize, usize, usize, usize))>, // 5 + 5 + 5 + 5
    imm: HashMap<u32, fn(&mut CPU, (usize, usize, u16))>, // 5 + 5 + 16
    jmp: HashMap<u32, fn(&mut CPU, u32)>,                 // 6 + 26
}

impl CPU {
    // size indicates program size(lines of code)
    pub fn init(size: u32) -> CPU {
        let r: HashMap<
            u32,
            fn(&mut CPU, (usize, usize, usize, usize)),
        > = HashMap::from([(
            0b100000,
            instructions::add
                as fn(&mut CPU, (usize, usize, usize, usize)),
        )]);
        let i: HashMap<u32, fn(&mut CPU, (usize, usize, u16))> =
            HashMap::from([]);
        let j: HashMap<u32, fn(&mut CPU, u32)> = HashMap::from([]);
        CPU {
            pc: 0,
            pb: size,
            //sp: 0xffff - 1,
            acc_hi: 0,
            acc_lo:0,
            r: [0; 32],
            bus: Bus::init(),
            reg: r,
            imm: i,
            jmp: j,
        }
    }
    pub fn write_bus(&mut self, addr: u32, val: u32) {
        self.bus.write(addr, val);
    }
    pub fn read_bus(&self, addr: u32) -> u32 {
        self.bus.read(addr)
    }
    pub fn exec_inst(&mut self) {
        let bytecode = self.read_bus(self.pc);
        let op = bytecode >> 26; // most sig 6 bits are op code
        match op {
            0 => {
                // first 6 bits, to get the function
                let func = bytecode & 0x3F;
                // grab from arithmetic instructions and dereference whats inside
                // the option
                let inst = self.reg.get(&func).map(|x| *x);
                // grab data from register!
                let rs = ((bytecode >> 21) & 0x1F) as usize;
                let rd = ((bytecode >> 16) & 0x1F) as usize;
                let rt = ((bytecode >> 11) & 0x1F) as usize;
                let shamt = ((bytecode >> 6) & 0x1F) as usize;
                // TODO: error handler/exception handler it. this shouldn't happen though
                // because it would run into issue with the loader
                // left in as debugging for func testing though
                match inst {
                    Some(inst) => inst(self, (rs, rd, rt, shamt)),
                    None => {
                        println!("invalid op!");
                    }
                }
            }
            1..=7 => {
                let inst = self.jmp.get(&op).map(|x| *x);
                // i think this works. This should be first 26 bits.
                let addr = bytecode & 0x3FFFFFF;
                match inst {
                    Some(inst) => inst(self, addr),
                    None => println!("invalid op!"),
                }
            } // jmp,
            8..=43 => {
                let rs = ((bytecode >> 21) & 0x1F) as usize;
                let rd = ((bytecode >> 16) & 0x1F) as usize;
                let imm = ((bytecode >> 6) & 0xFFFF) as u16;
                let inst = self.imm.get(&op).map(|x| *x);

                match inst {
                    Some(inst) => inst(self, (rs, rd, imm)),
                    None => println!("invalid op!"),
                }
            } //imm
            _ => {
                println!("{}: Not a valid op!", op)
            }
        }
        // increment program counter
        self.pc += 4;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_single_reg() {
        let mut c = CPU::init(1);
        c.write_bus(c.pc, 0b000000_00000_00001_00010_00000_100000);
        c.r[1] = 1u32;
        c.r[2] = 2u32;
        c.exec_inst();
        assert!(c.r[0] == 3);
    }
}
