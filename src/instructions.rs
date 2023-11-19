use crate::cpu_state::*;

// • Stack, I/O and Machine Control Group -
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
