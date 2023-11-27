use crate::cpu_state::*;

pub fn test(state: &mut State) {
    state.registers.a = 0b11000101;
    set_parity(state);
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
    // let binary_repr = format!("{:b}", state.registers.a);
    // let binary_sum = binary_repr.chars()
    //     .filter_map(|x| Some((x.to_digit(2).unwrap()==1) as usize))
    //     .sum::<usize>();
    // if binary_sum % 2 == 0 {
    //     state.flags.p = 1;
    // } else {
    //     state.flags.p = 0;
    // }
    //
    // 1001 0110
    //      1001 >> 4
    //      1111
    //        11 >> 2
    //        00 
    //         0 >> 1
    let mut x = state.registers.a ^ (state.registers.a >> 4);
    x = x ^ (x >> 2);
    x = x ^ (x >> 1);
    if (x & 1) == 1 { // check the last bit. if 1, odd parity, even otherwise
        state.flags.p = 0;
    } else {
        state.flags.p = 1;
    }
}
fn set_carry(state: &mut State, result: u16) {
    state.flags.cy = (result > 0xff) as u8;
}
fn set_aux_carry(state: &mut State) {
    // should take u8 and compare with 0xf
    // TODO: Implement for DAA
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
pub fn mov(state: &mut State, opcode: u8) {
    println!("MOV");

}
pub fn lxi(state: &mut State, opcode: u8) {
    println!("LXI");
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
pub fn adi(state: &mut State) {
    println!("ADI");
    let result : u16 = u16::from(state.registers.a) + u16::from(state.memory[state.registers.pc+1]);
    state.registers.a = result as u8;
    set_zero(state);
    set_sign(state);
    set_parity(state);
    set_carry(state, result);
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
pub fn hlt(state: &mut State){

}
