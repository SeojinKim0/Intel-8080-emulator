use std::process;

use crate::cpu_state::*;

pub fn test(state: &mut State) {
    // state.registers.a = 0b11000101;
    // set_parity(state);
    println!("{}", state.memory.len());
}

// ########## HELPER FUCTIONS ############
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
        0x36 => state.memory[usize::from(state.registers.h) << 8 | usize::from(state.registers.l)] = state.memory[state.registers.pc+1],
        0x3e => state.registers.a = state.memory[state.registers.pc+1],
        _ => println!("Wrong opcode in mvi"),
    }
    state.registers.pc += 2;
}
pub fn mov(state: &mut State, opcode: u8) {
    println!("MOV");

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
    match &opcode {
        0x80 => state.registers.a += state.registers.b,
        _ => todo!(),
    }
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
        _ => println!("Wrong opcode in jmp_cond"),
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
        _ => println!("Wrong opcode in call_cond"),
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
        _ => println!("Wrong opcode in ret_cond"),
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
        }
        _ => todo!(),
    }
    state.registers.sp -=2;
    state.registers.pc += 1;

}
pub fn hlt(state: &mut State){

}
