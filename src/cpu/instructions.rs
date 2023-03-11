use std::fmt::Pointer;

use crate::cpu::cpu;

//arithmetic
// signed addition
pub fn add(c: & mut cpu::CPU, data: (usize, usize, usize, usize)){
    unsafe{
        c.r[data.0] = std::mem::transmute::<i32, u32>(
            std::mem::transmute::<u32, i32>(c.r[data.1]) +
                std::mem::transmute::<u32, i32>(c.r[data.2]));
    }
}

//signed immediate addiiton
pub fn addi(c: &mut cpu::CPU, data:(usize, usize, u16)){
    c.r[data.0] = ((c.r[data.1] as i32) + (data.2 as i32)) as u32;
}

//unsigned addition
pub fn addu(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] + c.r[data.2];
}

// unsigned immediate addition
pub fn addiu(c: &mut cpu::CPU, data: (usize, usize, u16)){
    c.r[data.0] = c.r[data.1] + data.2 as u32;
}

// count leading ones
pub fn clo(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    let num = c.r[data.1];
    c.r[data.0] = (0..32).fold(0, |acc, elem| acc + ((num >> elem) & 1));
}

// count leading 0s
pub fn clz(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    let num = c.r[data.1];
    // basically just invert the counter for when we do clo
    c.r[data.0] = (0..32).fold(0, |acc, elem| acc + !((num >> elem) & 1));
}

// need to figure out how to convert this
// load address
// lui -> lor
//pub fn la(c: &mut cpu::CPU, data: (usize, usize, usize, u16)){
//    c.r[data[0]] = data[3];
//}
//
//// load immediate
// see above
//pub fn li(c: &mut cpu::CPU, data: (usize, usize, usize, u16)){
//    c.r[data[0]] = data[3];
//}

// load upper immediate
pub fn lui(c: &mut cpu::CPU, data: (usize, usize, u16)){
    c.r[data.0] = (data.2 as u32) << 16;
}

// moves source to destination
pub fn mov(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1];
}

// negate move unsigned
pub fn negu(c: &mut cpu::CPU, data:(usize, usize, usize, usize)){
    c.r[data.0] = !c.r[data.1];
}

// sign extend byte
pub fn seb(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] & 0xFF;
}

// sign extend half word
pub fn seh(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] & 0xFFFF;
}

// subtraction
pub fn sub(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    unsafe{
        c.r[data.0] = std::mem::transmute::<i32, u32>(
            (c.r[data.1] as i32) - (c.r[data.2] as i32));
    }
}

// subtraction unsigned
pub fn subu(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] - c.r[data.2];
}

// rotate right
pub fn rotr(c: &mut cpu::CPU, data:(usize, usize, usize, usize)){
    c.r[data.0] = (c.r[data.1] << (32 - data.3 + 1)) | (c.r[data.1] >> data.3);
}

// rotate right variable
pub fn rotrv(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    let rot = c.r[data.2] & 0x1F;
    c.r[data.0] = (c.r[data.1] << (32 - rot + 1)) | (c.r[data.1] >> rot);
}

// shift left 
pub fn sll(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] << data.3;
}

// shift left variable
pub fn sllv(c: &mut cpu::CPU, data: (usize, usize, usize)){
    let shift = c.r[data.2] & 0x1F;
    c.r[data.0] = c.r[data.1] << shift;
}

// shift right
pub fn sra(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    unsafe{
        c.r[data.0] = std::mem::transmute::<i32, u32>(
            std::mem::transmute::<u32, i32>(c.r[data.1])
        ) >> data.3;
    }
    c.r[data.0] = c.r[data.1] >> data.3;
}

// shift right variable
pub fn srav(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    let shift = c.r[data.2] & 0x1F;
    unsafe{
        c.r[data.0] = std::mem::transmute::<i32, u32>(
            std::mem::transmute::<u32, i32>(c.r[data.1])
        ) >> shift;
    }
}

// shift right logical
pub fn srl(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    // default u32 shift is logical
    c.r[data.0] = c.r[data.1] >> data.3;
}

// shift right logical variable
pub fn srlv(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    let shift = c.r[data.2] & 0x1F;
    c.r[data.0] = c.r[data.1] >> shift;
}

// AND
pub fn and(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] & c.r[data.2];
}

// and immediate
pub fn andi(c: &mut cpu::CPU, data: (usize, usize, u16)){
    c.r[data.0] = c.r[data.1] & (data.2 as u32);
}

// extract
pub fn ext(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    // TODO: test this one since its pretty weird
    // do i need to shift it back after?
    let mask = ((1 << data.3) - 1) << data.2;
    c.r[data.0] = c.r[data.1] & mask;
}

// insert
pub fn ins(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    let mask = (1 << data.3) -1;
    c.r[data.0] |= (c.r[data.1] & mask) << data.2;
}

// nor
pub fn nor(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = !(c.r[data.1] | c.r[data.2]);
}

// NOT
pub fn not(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = !c.r[data.1];
}

// or
pub fn or(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] | c.r[data.2];
}

// ori 
pub fn ori(c: &mut cpu::CPU, data: (usize, usize, u16)){
    c.r[data.0] = c.r[data.1] | (data.2 as u32);
}

// xor
pub fn xor(c: &mut cpu::CPU, data: (usize, usize, usize, usize)){
    c.r[data.0] = c.r[data.1] ^ c.r[data.2];
}

// xori
pub fn xori(c: &mut cpu::CPU, data: (usize, usize, u16)){
    c.r[data.0] = c.r[data.1] ^ (data.2 as u32);
}

mod tests{
    use super::*;

    #[test]
    fn test_ext(){
        // am i interpreting this instruction correctly?
        let mut c = cpu::CPU::init(1);
        c.r[1] = 0b11011100000000000000000000000000;
        ext(&mut c, (0, 1, 25, 3));
        assert!(c.r[0] == 0b00001100000000000000000000000000);
    }

    #[test]
    fn test_ins(){
        let mut c = cpu::CPU::init(1);
        c.r[0] = 0;
        c.r[1] = 0b1111;
        ins(&mut c, (0, 1, 2, 4));
        assert!(c.r[0] == 0b111100);
    }
}
