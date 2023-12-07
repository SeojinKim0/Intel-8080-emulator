use std::process;

use crate::cpu_state::*;

pub fn test(state: &mut State) {
    // state.registers.a = 0b11000101;
    // set_parity(state);
    println!("{}", state.memory.len());
}

// ########## HELPER FUCTIONS ############

fn get_memory(state: &mut State) -> u8 {
    state.memory[(usize::from(state.registers.h) << 8) | usize::from(state.registers.l)]
}
fn get_memory_direct(state: &mut State, address: usize) -> u8 {
    state.memory[address]
}

// TODO: refactor setting memory
fn set_memory_direct(state: &mut State, address: usize, val: u8) {
    state.memory[address] = val;
}
fn set_memory(state: &mut State, val: u8) {
    state.memory[(usize::from(state.registers.h) << 8) | usize::from(state.registers.l)] = val;
}

fn set_zero(state: &mut State) {
    state.flags.z = (state.registers.a == 0) as u8;
    state.flags.z = ((state.registers.a & 0xff) == 0) as u8
}
/// 0 when plus, 1 when minus
fn set_sign(state: &mut State) {
    state.flags.s = ((state.registers.a & 0x80) == 0x80) as u8; // 0b1000 0000
}
/// Set when it's even parity. reset otherwise
fn set_parity(state: &mut State) {
    // 1001 0110
    //      1001 >> 4
    //      1111
    //        11 >> 2
    //        00 
    //         0 >> 1
    let mut x = state.registers.a ^ (state.registers.a >> 4);
    x = x ^ (x >> 2);
    x = x ^ (x >> 1);
    if (x & 1) == 1 { // if 1, odd parity, even otherwise
        state.flags.p = 0;
    } else {
        state.flags.p = 1;
    }
}
fn set_carry_add(state: &mut State, result: u16) {
    state.flags.cy = (result > 0xff) as u8;
}

// should be right but it gives different val
fn set_carry_sub(state: &mut State, subtrahend: u8) {
    state.flags.cy = (state.registers.a < subtrahend) as u8;
}
fn set_aux_carry(state: &mut State) {
    // should take u8 and compare with 0xf
    // TODO: Implement later for DAA
}

fn reset_carry(state: &mut State) {
    state.flags.cy = 0;
}

fn reset_aux_carry(state: &mut State) {
    state.flags.ac = 0;
}
/*
    ************************************************************
    *                                                          *
    *                  Data Transfer Group                     *
    *                                                          *
    ************************************************************
*/
pub fn sta (state: &mut State) {
    println!("STA");
    let address = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
    set_memory_direct(state, address, state.registers.a);
    state.registers.pc += 3;
}
/// Load Accumulator direct
pub fn lda (state: &mut State) {
    println!("LDA");
    let address = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
    state.registers.a = get_memory_direct(state, address);
    state.registers.pc += 3;
}
/// Load H and L direct
/// L <- (adr); H<-(adr+1)
pub fn lhld (state: &mut State) {
    println!("LHLD");
    let address = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
    state.registers.l = get_memory_direct(state, address);
    state.registers.h = get_memory_direct(state, address+1);
    state.registers.pc += 3;
}
///Store Hand L direct
///(adr) <-L; (adr+1)<-H
pub fn shld(state: &mut State) {
    println!("SHLD");
    let address = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
    set_memory_direct(state, address, state.registers.l);
    set_memory_direct(state, address+1, state.registers.h);
    state.registers.pc += 3;
}

pub fn ldax(state: &mut State, opcode: u8) {
    println!("LDAX");
    let mut address = 0;
    match opcode {
        0x0a => address = (state.registers.b as usize) << 8 | state.registers.c as usize,
        0x1a => address = (state.registers.d as usize) << 8 | state.registers.e as usize,
        _ => panic!("Wrong opcode in ldax"),
    }
    state.registers.a = get_memory_direct(state, address);
    state.registers.pc += 1;
}
pub fn stax(state: &mut State, opcode: u8) {
    println!("STAX");
    let mut address = 0;
    match opcode {
        0x02 => address = (state.registers.b as usize) << 8 | state.registers.c as usize,
        0x12 => address = (state.registers.d as usize) << 8 | state.registers.e as usize,
        _ => panic!("Wrong opcode in stax"),
    }
    set_memory_direct(state, address, state.registers.a);
    state.registers.pc += 1;
}
pub fn xchg (state: &mut State) {
    println!("XCHG");
    let tmp_h = state.registers.h;
    let tmp_l = state.registers.l;
    state.registers.h = state.registers.d;
    state.registers.l = state.registers.e;
    state.registers.d = tmp_h;
    state.registers.e = tmp_l;
    state.registers.pc += 1;
}

pub fn mvi(state: &mut State, opcode: u8) {
    println!("MVI");
    match opcode {
        0x06 => state.registers.b = state.memory[state.registers.pc+1],
        0x0e => state.registers.c = state.memory[state.registers.pc+1],
        0x16 => state.registers.d = state.memory[state.registers.pc+1],
        0x1e => state.registers.e = state.memory[state.registers.pc+1],
        0x26 => state.registers.h = state.memory[state.registers.pc+1],
        0x2e => state.registers.l = state.memory[state.registers.pc+1],
        0x36 => set_memory(state, state.memory[state.registers.pc+1]),
        0x3e => state.registers.a = state.memory[state.registers.pc+1],
        _ => panic!("Wrong opcode in mvi"),
    }
    state.registers.pc += 2;
}
pub fn mov(state: &mut State, opcode: u8) {
    println!("MOV");
    match opcode {
        0x40 => state.registers.b = state.registers.b,
        0x41 => state.registers.b = state.registers.c,
        0x42 => state.registers.b = state.registers.d,
        0x43 => state.registers.b = state.registers.e,
        0x44 => state.registers.b = state.registers.h,
        0x45 => state.registers.b = state.registers.l,
        0x46 => state.registers.b = get_memory(state),
        0x47 => state.registers.b = state.registers.a,

        0x48 => state.registers.c = state.registers.b,
        0x49 => state.registers.c = state.registers.c,
        0x4a => state.registers.c = state.registers.d,
        0x4b => state.registers.c = state.registers.e,
        0x4c => state.registers.c = state.registers.h,
        0x4d => state.registers.c = state.registers.l,
        0x4e => state.registers.c = get_memory(state),
        0x4f => state.registers.c = state.registers.a,

        0x50 => state.registers.d = state.registers.b,
        0x51 => state.registers.d = state.registers.c,
        0x52 => state.registers.d = state.registers.d,
        0x53 => state.registers.d = state.registers.e,
        0x54 => state.registers.d = state.registers.h,
        0x55 => state.registers.d = state.registers.l,
        0x56 => state.registers.d = get_memory(state),
        0x57 => state.registers.d = state.registers.a,

        0x58 => state.registers.e = state.registers.b,
        0x59 => state.registers.e = state.registers.c,
        0x5a => state.registers.e = state.registers.d,
        0x5b => state.registers.e = state.registers.e,
        0x5c => state.registers.e = state.registers.h,
        0x5d => state.registers.e = state.registers.l,
        0x5e => state.registers.e = get_memory(state),
        0x5f => state.registers.e = state.registers.a,

        0x60 => state.registers.h = state.registers.b,
        0x61 => state.registers.h = state.registers.c,
        0x62 => state.registers.h = state.registers.d,
        0x63 => state.registers.h = state.registers.e,
        0x64 => state.registers.h = state.registers.h,
        0x65 => state.registers.h = state.registers.l,
        0x66 => state.registers.h = get_memory(state),
        0x67 => state.registers.h = state.registers.a,

        0x68 => state.registers.l = state.registers.b,
        0x69 => state.registers.l = state.registers.c,
        0x6a => state.registers.l = state.registers.d,
        0x6b => state.registers.l = state.registers.e,
        0x6c => state.registers.l = state.registers.h,
        0x6d => state.registers.l = state.registers.l,
        0x6e => state.registers.l = get_memory(state),
        0x6f => state.registers.l = state.registers.a,

        0x70 => set_memory(state, state.registers.b),
        0x71 => set_memory(state,  state.registers.c),
        0x72 => set_memory(state, state.registers.d),
        0x73 => set_memory(state, state.registers.e),
        0x74 => set_memory(state, state.registers.h),
        0x75 => set_memory(state, state.registers.l),
        0x77 => set_memory(state, state.registers.a),

        0x78 => state.registers.a = state.registers.b,
        0x79 => state.registers.a = state.registers.c,
        0x7a => state.registers.a = state.registers.d,
        0x7b => state.registers.a = state.registers.e,
        0x7c => state.registers.a = state.registers.h,
        0x7d => state.registers.a = state.registers.l,
        0x7e => state.registers.a = get_memory(state),
        0x7f => state.registers.a = state.registers.a,
        _ => panic!("Wrong opcode in mov"),
    }
    state.registers.pc += 1;
}
pub fn lxi(state: &mut State, opcode: u8) {
    println!("LXI");
    match opcode {
        0x01 => {
            state.registers.b = state.memory[state.registers.pc+2];
            state.registers.c = state.memory[state.registers.pc+1];
        }
        0x11 => {
            state.registers.d = state.memory[state.registers.pc+2];
            state.registers.e = state.memory[state.registers.pc+1];
        }
        0x21 => {
            state.registers.h = state.memory[state.registers.pc+2];
            state.registers.l = state.memory[state.registers.pc+1];
        }
        0x31 => {
            state.registers.sp = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
        }
        _ => todo!(),
    }
    state.registers.pc+=3;
}

/*
    ************************************************************
    *                                                          *
    *                     Arithmetic Group                     *
    *                                                          *
    ************************************************************
*/

/// HL = HL + BC
pub fn dad (state: &mut State, opcode: u8) {
    println!("DAD");
    let lhs : u16 = u16::from(state.registers.h) << 8 | u16::from(state.registers.l);
    let mut rhs : u16 = 0;
    match opcode {
        0x09 => rhs = u16::from(state.registers.b) << 8 | u16::from(state.registers.c),
        0x19 => rhs = u16::from(state.registers.d) << 8 | u16::from(state.registers.e),
        0x29 => rhs = u16::from(state.registers.h) << 8 | u16::from(state.registers.l),
        0x39 => rhs = state.registers.sp as u16,
        _ => panic!("Wrong opcode in dad"),
    }
    let result : u32 = u32::from(lhs) + u32::from(rhs);
    state.flags.cy = (result > 0xffff) as u8;
    state.registers.h = ((result >> 8) & 0xff) as u8;
    state.registers.l = (result & 0xff) as u8;
    state.registers.pc += 1;
}

pub fn dcx(state: &mut State, opcode: u8) {
    println!("DCX");
    match opcode {
        0x0b => {
            state.registers.b = state.registers.b.wrapping_sub(1);
            state.registers.c = state.registers.c.wrapping_sub(1);
        }                                        
        0x1b => {                                
            state.registers.d = state.registers.d.wrapping_sub(1);
            state.registers.e = state.registers.e.wrapping_sub(1);
        }                                        
        0x2b => {                                
            state.registers.h = state.registers.h.wrapping_sub(1);
            state.registers.l = state.registers.l.wrapping_sub(1);
        }
        0x3b => {
            state.registers.sp = state.registers.sp.wrapping_sub(1);
        }
        _ => panic!("Wrong opcode in DCX"),
    }
    state.registers.pc += 1;
}
pub fn inx(state: &mut State, opcode: u8) {
    println!("INX");
    match opcode {
        0x03 => {
            state.registers.b = state.registers.b.wrapping_add(1);
            state.registers.c = state.registers.c.wrapping_add(1);
        }
        0x13 => {
            state.registers.d = state.registers.d.wrapping_add(1);
            state.registers.e = state.registers.e.wrapping_add(1);
        }
        0x23 => {
            state.registers.h = state.registers.h.wrapping_add(1);
            state.registers.l = state.registers.l.wrapping_add(1);
        }
        0x33 => {
            state.registers.sp = state.registers.sp.wrapping_add(1);
        }
        _ => panic!("Wrong opcode in INX"),
    }
    state.registers.pc += 1;
}
pub fn inr(state: &mut State, opcode: u8) {
    println!("INR");
    match opcode {
        0x04 => state.registers.b = state.registers.b.wrapping_add(1),
        0x0c => state.registers.c = state.registers.c.wrapping_add(1),
        0x14 => state.registers.d = state.registers.d.wrapping_add(1),
        0x1c => state.registers.e = state.registers.e.wrapping_add(1),
        0x24 => state.registers.h = state.registers.h.wrapping_add(1),
        0x2c => state.registers.l = state.registers.l.wrapping_add(1),
        0x34 => { 
            let val = get_memory(state).wrapping_add(1);
            set_memory(state, val);
        }
        0x3c => state.registers.a = state.registers.a.wrapping_add(1),
        _ => panic!("Wrong opcode in inr"),
    }
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc += 1;
}

pub fn dcr(state: &mut State, opcode: u8) {
    match opcode {
        0x05 => state.registers.b = state.registers.b.wrapping_sub(1),
        0x0d => state.registers.c = state.registers.c.wrapping_sub(1),
        0x15 => state.registers.d = state.registers.d.wrapping_sub(1),
        0x1d => state.registers.e = state.registers.e.wrapping_sub(1),
        0x25 => state.registers.h = state.registers.h.wrapping_sub(1),
        0x2d => state.registers.l = state.registers.l.wrapping_sub(1),
        0x35 => {
            let val = get_memory(state).wrapping_sub(1);
            set_memory(state, val);
        }
        0x3d => state.registers.a = state.registers.a.wrapping_sub(1),
        _ => panic!("Wrong opcode in dcr"),
    }
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc += 1;
}
pub fn adi(state: &mut State) {
    println!("ADI");
    let result : u16 = u16::from(state.registers.a) + u16::from(state.memory[state.registers.pc+1]);
    state.registers.a = result as u8;
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_carry_add(state, result);
    set_aux_carry(state);

    state.registers.pc += 2;
}
pub fn add(state: &mut State, opcode: u8) {
    println!("ADD");
    let mut result : u16 = 0;
    match opcode {
        0x80 => result = u16::from(state.registers.a) + u16::from(state.registers.b),
        0x81 => result = u16::from(state.registers.a) + u16::from(state.registers.c),
        0x82 => result = u16::from(state.registers.a) + u16::from(state.registers.d),
        0x83 => result = u16::from(state.registers.a) + u16::from(state.registers.e),
        0x84 => result = u16::from(state.registers.a) + u16::from(state.registers.h),
        0x85 => result = u16::from(state.registers.a) + u16::from(state.registers.l),
        0x86 => result = u16::from(state.registers.a) + get_memory(state) as u16,
        0x87 => result = u16::from(state.registers.a) + u16::from(state.registers.a),
        _ => panic!("Wrong opcode in add"),
    }
    state.registers.a = result as u8;
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_carry_add(state, result);
    set_aux_carry(state);
    state.registers.pc += 1;
}

pub fn sub(state: &mut State, opcode: u8) {
    println!("SUB");
    let mut subtrahend : u8 = 0 ;
    match opcode {
        0x90 => subtrahend = state.registers.b,
        0x91 => subtrahend = state.registers.c,
        0x92 => subtrahend = state.registers.d,
        0x93 => subtrahend = state.registers.e,
        0x94 => subtrahend = state.registers.h,
        0x95 => subtrahend = state.registers.l,
        0x96 => subtrahend = get_memory(state),
        0x97 => subtrahend = state.registers.a,
        _ => panic!("Wrong opcode in sub"),
    }
    set_carry_sub(state, subtrahend);
    state.registers.a = state.registers.a.wrapping_sub(subtrahend);
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc += 1;
}

pub fn adc (state: &mut State, opcode: u8) {
    println!("ADC");
    let mut result : u16 = 0;
    match opcode {
        0x88 => result = u16::from(state.registers.a) + u16::from(state.registers.b),
        0x89 => result = u16::from(state.registers.a) + u16::from(state.registers.c),
        0x8a => result = u16::from(state.registers.a) + u16::from(state.registers.d),
        0x8b => result = u16::from(state.registers.a) + u16::from(state.registers.e),
        0x8c => result = u16::from(state.registers.a) + u16::from(state.registers.h),
        0x8d => result = u16::from(state.registers.a) + u16::from(state.registers.l),
        0x8e => result = u16::from(state.registers.a) + get_memory(state) as u16,
        0x8f => result = u16::from(state.registers.a) + u16::from(state.registers.a),
        _ => panic!("Wrong opcode in adc"),
    }
    result += state.flags.cy as u16;
    state.registers.a = result as u8;
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_carry_add(state, result);
    set_aux_carry(state);
    state.registers.pc += 1;
}

pub fn sbb(state: &mut State, opcode: u8) {
    println!("SBB");
    let mut subtrahend : u8 = 0 ;
    match opcode {
        0x98 => subtrahend = state.registers.b,
        0x99 => subtrahend = state.registers.c,
        0x9a => subtrahend = state.registers.d,
        0x9b => subtrahend = state.registers.e,
        0x9c => subtrahend = state.registers.h,
        0x9d => subtrahend = state.registers.l,
        0x9e => subtrahend = get_memory(state),
        0x9f => subtrahend = state.registers.a,
        _ => panic!("Wrong opcode in sub"),
    }
    subtrahend += state.flags.cy;
    set_carry_sub(state, subtrahend);
    state.registers.a = state.registers.a.wrapping_sub(subtrahend);
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc += 1;
}
pub fn aci(state: &mut State) {
    println!("ACI");
    let result : u16 = u16::from(state.registers.a) + u16::from(state.memory[state.registers.pc+1]) + u16::from(state.flags.cy);
    state.registers.a = result as u8;
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_carry_add(state, result);
    set_aux_carry(state);
    state.registers.pc += 2;
}

pub fn sbi(state: &mut State) {
    println!("SBI");
    let subtrahend : u8 = state.memory[state.registers.pc+1] + state.flags.cy;
    set_carry_sub(state, subtrahend);

    state.registers.a = state.registers.a.wrapping_sub(subtrahend);
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc+=2;
}
pub fn sui(state: &mut State) {
    println!("SUI");
    let subtrahend : u8 = state.memory[state.registers.pc+1];
    set_carry_sub(state, subtrahend);

    // to prevent overflow
    // uses two's complement
    state.registers.a = state.registers.a.wrapping_sub(subtrahend);
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc+=2;
}
/*
    ************************************************************
    *                                                          *
    *                      Logical Group                       *
    *                                                          *
    ************************************************************
*/

pub fn stc (state: &mut State) {
    println!("STC");
    state.flags.cy = 1;
    state.registers.pc += 1;
}

pub fn cmc (state: &mut State) {
    println!("CMC");
    state.flags.cy = !state.flags.cy;
    state.registers.pc += 1;
}

pub fn cma (state: &mut State) {
    println!("CMA");
    state.registers.a = !state.registers.a;
    state.registers.pc += 1;
}

pub fn rlc (state: &mut State) {
    println!("RLC");
    state.flags.cy = state.registers.a >> 7;
    state.registers.a = state.registers.a << 1 | (state.flags.cy & 1);
    state.registers.pc += 1;
}

pub fn rrc(state: &mut State) {
    println!("RRC");
    state.flags.cy = state.registers.a & 1;
    state.registers.a = state.registers.a >> 1 | (state.flags.cy << 7);
    state.registers.pc += 1;
}

pub fn rar(state: &mut State) {
    println!("RAR");
    let tmp = state.flags.cy;
    state.flags.cy = state.registers.a & 1;
    state.registers.a = (state.registers.a >> 1) | (tmp << 7);
    state.registers.pc += 1;
}

pub fn ral(state: &mut State) {
    println!("RAL");
    let tmp = state.flags.cy;
    state.flags.cy = state.registers.a >> 7;
    state.registers.a = state.registers.a << 1 | (tmp & 1);
    state.registers.pc += 1;
}

pub fn ora(state: &mut State, opcode: u8) {
    println!("ORA");
    match opcode {
        0xb0 => state.registers.a |= state.registers.b,
        0xb1 => state.registers.a |= state.registers.c,
        0xb2 => state.registers.a |= state.registers.d,
        0xb3 => state.registers.a |= state.registers.e,
        0xb4 => state.registers.a |= state.registers.h,
        0xb5 => state.registers.a |= state.registers.l,
        0xb6 => state.registers.a |= get_memory(state),
        0xb7 => state.registers.a |= state.registers.a,
        _ => panic!("Wrong opcode in ora"),
    }
    set_zero(state);
    set_sign(state);
    set_parity(state);
    reset_carry(state);
    reset_aux_carry(state);
    state.registers.pc += 1;
}

pub fn cmp(state: &mut State, opcode: u8) {
    println!("CMP");
    let mut subtrahend : u8 = 0 ;
    match opcode {
        0xb8 => subtrahend = state.registers.b,
        0xb9 => subtrahend = state.registers.c,
        0xba => subtrahend = state.registers.d,
        0xbb => subtrahend = state.registers.e,
        0xbc => subtrahend = state.registers.h,
        0xbd => subtrahend = state.registers.l,
        0xbe => subtrahend = get_memory(state),
        0xbf => subtrahend = state.registers.a,
        _ => panic!("Wrong opcode in cmp"),
    }
    set_carry_sub(state, subtrahend);
    let tmp_a = state.registers.a;
    state.registers.a = state.registers.a.wrapping_sub(subtrahend);
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.a = tmp_a;
    state.registers.pc += 1;
}

pub fn ana(state: &mut State, opcode: u8) {
    println!("ANA");
    match opcode {
        0xa0 => state.registers.a &= state.registers.b,
        0xa1 => state.registers.a &= state.registers.c,
        0xa2 => state.registers.a &= state.registers.d,
        0xa3 => state.registers.a &= state.registers.e,
        0xa4 => state.registers.a &= state.registers.h,
        0xa5 => state.registers.a &= state.registers.l,
        0xa6 => state.registers.a &= get_memory(state),
        0xa7 => state.registers.a &= state.registers.a,
        _ => panic!("Wrong opcode in ana"),
    }
    set_zero(state);
    set_sign(state);
    set_parity(state);
    reset_carry(state);
    set_aux_carry(state);
    state.registers.pc += 1;
}

pub fn xra(state: &mut State, opcode: u8) {
    println!("XRA");
    match opcode {
        0xa8 => state.registers.a ^= state.registers.b,
        0xa9 => state.registers.a ^= state.registers.c,
        0xaa => state.registers.a ^= state.registers.d,
        0xab => state.registers.a ^= state.registers.e,
        0xac => state.registers.a ^= state.registers.h,
        0xad => state.registers.a ^= state.registers.l,
        0xae => state.registers.a ^= get_memory(state),
        0xaf => state.registers.a ^= state.registers.a,
        _ => panic!("Wrong opcode in xra"),
    }
    set_zero(state);
    set_sign(state);
    set_parity(state);
    reset_carry(state);
    reset_aux_carry(state);
    state.registers.pc += 1;

}
pub fn ani(state: &mut State) {
    println!("ANI");
    state.registers.a = state.registers.a & state.memory[state.registers.pc+1];
    set_zero(state);
    set_sign(state);
    set_parity(state);
    reset_carry(state);
    reset_aux_carry(state);
    state.registers.pc+=2;
}

/// doesn't store the result
pub fn cpi(state: &mut State) {
    println!("CPI");
    let subtrahend : u8 = state.memory[state.registers.pc+1];
    set_carry_sub(state, subtrahend);

    let tmp_a = state.registers.a;
    // to prevent overflow
    state.registers.a = state.registers.a.wrapping_sub(state.memory[state.registers.pc+1]);
    // println!("A val : {:b}", state.registers.a);
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_aux_carry(state);
    state.registers.pc+=2;

    // restore the value
    state.registers.a = tmp_a;
}

pub fn ori(state: &mut State) {
    println!("ORI");
    state.registers.a = state.registers.a | state.memory[state.registers.pc+1];
    set_zero(state);
    set_sign(state);
    set_parity(state);
    reset_carry(state);
    reset_aux_carry(state);
    state.registers.pc+=2;
}

pub fn xri(state: &mut State) {
    println!("XRI");
    state.registers.a = state.registers.a ^ state.memory[state.registers.pc+1];
    set_zero(state);
    set_sign(state);
    set_parity(state);
    reset_carry(state);
    reset_aux_carry(state);
    state.registers.pc+=2;
}
/*
    ************************************************************
    *                                                          *
    *                      Branch Group                        *
    *                                                          *
    ************************************************************
*/

pub fn pchl(state: &mut State) {
    println!("PCHL");
    state.registers.pc = usize::from(state.registers.h) << 8 | usize::from(state.registers.l);
}

pub fn jmp_cond(state: &mut State, opcode: u8) {
    match opcode {
        0xc2 => jnz(state),
        0xc3 => jmp(state),
        0xca => jz(state),
        0xd2 => jnc(state),
        0xda => jc(state),
        0xe2 => jpo(state),
        0xea => jpe(state),
        0xf2 => jp(state),
        0xfa => jm(state),
        _ => panic!("Wrong opcode in jmp_cond"),
    }
}

pub fn call_cond(state: &mut State, opcode: u8) {
    match opcode {
        0xc4 => cnz(state),
        0xcc => cz(state),
        0xcd => call(state),
        0xd4 => cnc(state),
        0xdc => cc(state),
        0xe4 => cpo(state),
        0xec => cpe(state),
        0xf4 => cp(state),
        0xfc => cm(state),
        _ => panic!("Wrong opcode in call_cond"),
    }
}

pub fn ret_cond(state: &mut State, opcode: u8) {
    match opcode {
        0xc0 => rnz(state),
        0xc8 => rz(state),
        0xc9 => ret(state),
        0xd0 => rnc(state),
        0xd8 => rc(state),
        0xe0 => rpo(state),
        0xe8 => rpe(state),
        0xf0 => rp(state),
        0xf8 => rm(state),
        _ => panic!("Wrong opcode in ret_cond"),
    }
}

fn ret(state: &mut State) {
    println!("RET");
    println!("sp: {:#x}", state.registers.sp);
    println!("Memory[{:#x}]:{:#x}\nMemory[{:#x}]:{:#x}", state.registers.sp+1, state.memory[state.registers.sp+1], state.registers.sp, state.memory[state.registers.sp]);
    state.registers.pc = usize::from(state.memory[state.registers.sp+1]) << 8 | usize::from(state.memory[state.registers.sp]);
    state.registers.sp += 2;
}

fn rnz(state: &mut State) {
    println!("RNZ");
    if state.flags.z == 0 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}

fn rz(state: &mut State) {
    println!("RZ");
    if state.flags.z == 1 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}

fn rnc(state: &mut State) {
    println!("RNC");
    if state.flags.cy == 0 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}
fn rc(state: &mut State) {
    println!("RC");
    if state.flags.cy == 1 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}
fn rpo(state: &mut State) {
    println!("RPO");
    if state.flags.p == 0 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}
fn rpe(state: &mut State) {
    println!("RPE");
    if state.flags.p == 1 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}
fn rp(state: &mut State) {
    println!("RP");
    if state.flags.s == 0 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}
fn rm(state: &mut State) {
    println!("RM");
    if state.flags.s == 1 {
        ret(state);
    } else {
        state.registers.pc += 1;
    }
}


fn call(state: &mut State) {
    println!("CALL");
    let next_instr = state.registers.pc+3;
    state.memory[state.registers.sp-1] = ((next_instr >> 8) & 0xff) as u8;
    state.memory[state.registers.sp-2] = (next_instr & 0xff) as u8;
    println!("Memory[{:#x}]: {:#x}\nMemory[{:#x}]: {:#x}", state.registers.sp-1,  state.memory[state.registers.sp-1], state.registers.sp-2,state.memory[state.registers.sp-2]);

    state.registers.sp -= 2;
    jmp(state);
    // state.registers.pc = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
}

fn cnz(state: &mut State) {
    println!("CNZ");
    if state.flags.z == 0 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cz(state: &mut State) {
    println!("CZ");
    if state.flags.z == 1 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cnc(state: &mut State) {
    println!("CNC");
    if state.flags.cy == 0 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cc(state: &mut State) {
    println!("CC");
    if state.flags.cy == 1 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cpo(state: &mut State) {
    println!("CPO");
    if state.flags.p == 0 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cpe(state: &mut State) {
    println!("CPE");
    if state.flags.p == 1 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cp(state: &mut State) {
    println!("CP");
    if state.flags.s == 0 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}
fn cm(state: &mut State) {
    println!("CM");
    if state.flags.s == 1 {
        call(state);
    } else {
        state.registers.pc += 3;
    }
}

fn jnz(state: &mut State) {
    println!("JNZ");
    if state.flags.z == 0 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}

fn jmp(state: &mut State) {
    println!("JMP");
    state.registers.pc = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
}

fn jz(state: &mut State) {
    println!("JZ");
    if state.flags.z == 1 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}

fn jnc(state: &mut State) {
    println!("JNC");
    if state.flags.cy == 0 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}

fn jc(state: &mut State) {
    println!("JC");
    if state.flags.cy == 1 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}

fn jpo(state: &mut State) {
    println!("JPO");
    if state.flags.p == 0 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}
fn jpe(state: &mut State) {
    println!("JPE");
    if state.flags.p == 1 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}
fn jp(state: &mut State) {
    println!("JP");
    if state.flags.s == 0 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}
fn jm(state: &mut State) {
    println!("JM");
    if state.flags.s == 1 {
        jmp(state);
    } else {
        state.registers.pc +=3;
    }
}
/*
    ************************************************************
    *                                                          *
    *           Stack, I/O and Machine Control Group           *
    *                                                          *
    ************************************************************
*/
pub fn pop(state: &mut State, opcode: u8) {
    println!("POP");
    match opcode {
        0xc1 => {
            state.registers.c = state.memory[state.registers.sp];
            state.registers.b = state.memory[state.registers.sp+1];
        }
        0xd1 => {
            state.registers.d = state.memory[state.registers.sp];
            state.registers.e = state.memory[state.registers.sp+1];
        }
        0xe1 => {
            state.registers.h = state.memory[state.registers.sp];
            state.registers.l = state.memory[state.registers.sp+1];
        }                    
        0xf1 => {            
            println!("{:b}", state.memory[state.registers.sp]);
            state.flags.cy = state.memory[state.registers.sp] & 1;
            state.flags.p = (state.memory[state.registers.sp] >> 2) & 1;
            state.flags.ac = (state.memory[state.registers.sp] >> 4) & 1;
            state.flags.z = (state.memory[state.registers.sp] >> 6) & 1;
            state.flags.s = (state.memory[state.registers.sp] >> 7) & 1;
            state.registers.a = state.memory[state.registers.sp+1];
        }
        _ => panic!("Wrong opcode in pop"),
    }
    state.registers.sp += 2;
    state.registers.pc += 1;
}

pub fn push(state: &mut State, opcode: u8) {
    println!("PUSH");
    match opcode {
        0xc5 => {
            state.memory[state.registers.sp-1] = state.registers.b;
            state.memory[state.registers.sp-2] = state.registers.c;
        }
        0xd5 => {
            state.memory[state.registers.sp-1] = state.registers.d;
            state.memory[state.registers.sp-2] = state.registers.e;
        }
        0xe5 => {
            state.memory[state.registers.sp-1] = state.registers.h;
            state.memory[state.registers.sp-2] = state.registers.l;
        }
        0xf5 => {
            state.memory[state.registers.sp-1] = state.registers.a;
            state.memory[state.registers.sp-2] = 0b00000010 | state.flags.cy | (state.flags.p << 2) | (state.flags.ac << 4) | (state.flags.z << 6) | (state.flags.s << 7);
            println!("{:b}", state.memory[state.registers.sp-2])
        }
        _ => todo!(),
    }
    state.registers.sp -=2;
    state.registers.pc += 1;

}

pub fn sphl(state: &mut State) {
    println!("SPHL");
    state.registers.sp = ((state.registers.h as usize) << 8) | (state.registers.l as usize);
    state.registers.pc += 1;
}

pub fn xthl(state: &mut State) {
    println!("XTHL");
    let mut tmp = state.registers.l;
    let mut address = state.registers.sp;
    state.registers.l = get_memory_direct(state, address);
    set_memory_direct(state, address, tmp);

    tmp = state.registers.h;
    address = state.registers.sp + 1;
    state.registers.h = get_memory_direct(state, address);
    set_memory_direct(state, address, tmp);
    state.registers.pc += 1;
}
pub fn hlt(state: &mut State){

}
