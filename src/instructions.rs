use crate::cpu_state::*;

pub fn test(state: &mut State) {
    state.registers.a = 0b00000101;
    set_parity(state);
}
fn set_zero(state: &mut State) {
    state.flags.z = (state.registers.a == 0) as u8;
    state.flags.z = ((state.registers.a & 0xff) == 0) as u8
}
fn set_sign(state: &mut State) {
    state.flags.s = ((state.registers.a & 0x80) == 0x80) as u8; // 0b1000 0000
}
/// Set when it's even parity. reset otherwise
fn set_parity(state: &mut State) {
    // let binary_repr = format!("{:b}", state.registers.a);
    // let binary_sum = binary_repr.chars()
    //     .filter_map(|x| Some((x.to_digit(2).unwrap()==1) as usize))
    //     .sum::<usize>();
    // if binary_sum % 2 == 0 {
    //     state.flags.p = 1;
    // } else {
    //     state.flags.p = 0;
    // }
    let mut x = state.registers.a ^ (state.registers.a >> 1);
    x = x ^ (x >> 2);
    x = x ^ (x >> 4);
    if (x & 1) == 1 {
        state.flags.p = 0;
    } else {
        state.flags.p = 1;
    }
}
fn set_carry(state: &mut State) {
    // TODO: should take actual u16 from caller function
    state.flags.cy = (u16::from(state.registers.a) > 0xff) as u8;
}
fn set_aux_carry(state: &mut State) {
    // should take u8 and compare with 0xf

}
/*
    ************************************************************
    *                                                          *
    *                  Data Transfer Group                     *
    *                                                          *
    ************************************************************
*/
pub fn mov(state: &mut State, opcode: u8) {

}
pub fn lxi(state: &mut State, opcode: u8) {
    match &opcode {
        0x01 => {
            state.registers.b = state.memory[state.registers.pc+2];
            state.registers.c = state.memory[state.registers.pc+1];
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
pub fn add(state: &mut State, opcode: u8) {
    match &opcode {
        0x80 => state.registers.a = state.registers.a + state.registers.b,
        _ => todo!(),
    }
}
/*
    ************************************************************
    *                                                          *
    *                      Logical Group                       *
    *                                                          *
    ************************************************************
*/

pub fn ani(state: &mut State) {
    state.registers.a = state.registers.a & state.memory[state.registers.pc+1];

    state.flags.cy = 0;
    state.flags.ac = 0;
    state.registers.pc+=2;
    //TODO: flags
}
/*
    ************************************************************
    *                                                          *
    *                      Branch Group                        *
    *                                                          *
    ************************************************************
*/
pub fn jmp(state: &mut State) {
    state.registers.pc = usize::from(state.memory[state.registers.pc+2]) << 8 | usize::from(state.memory[state.registers.pc+1]);
}

/*
    ************************************************************
    *                                                          *
    *           Stack, I/O and Machine Control Group           *
    *                                                          *
    ************************************************************
*/
pub fn hlt(state: &mut State){

}
