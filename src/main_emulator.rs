use crate::instructions::*;
use crate::cpu_state::*;

use std::process;
use std::fs;


fn unimplemented_instruction(state: &State) {
    println!("Error: Unimplemented instruction");
    process::exit(1);
}

pub fn read_file_to_memory(state: &mut State, file: &str, mem_addr: usize) {
    let file_content = fs::read(file).unwrap();
    // state.memory.resize(mem_addr + file_content.len(), 0);
    state.memory[mem_addr.. mem_addr+ file_content.len()].copy_from_slice(&file_content);
    state.registers.pc = mem_addr;
}

pub fn emulate8080(state: &mut State) {
    let opcode = state.memory[state.registers.pc];
    println!("current instruction: {:#x} {:#x} {:#x}", state.memory[state.registers.pc],  state.memory[state.registers.pc+1],  state.memory[state.registers.pc+2]);
    // println!("TEST!");
    // test(state);
    if state.registers.pc == 0x0005 {
        process::exit(1);
    }
    match opcode {
        0x00 => (),
        0x01 | 0x11 | 0x21 | 0x31=> lxi(state, opcode),
        0x05 => unimplemented_instruction(state),
        0x06 | 0x0e | 0x16 |
        0x1e | 0x26 | 0x2e |
        0x36 | 0x3e => mvi(state, opcode),
        0x09 => unimplemented_instruction(state),
        0x0d => unimplemented_instruction(state),
        0x0f => unimplemented_instruction(state),
        0x13 => unimplemented_instruction(state),
        0x19 => unimplemented_instruction(state),
        0x1a => unimplemented_instruction(state),
        0x23 => unimplemented_instruction(state),
        0x29 => unimplemented_instruction(state),
        0x32 => unimplemented_instruction(state),
        0x3a => unimplemented_instruction(state),
        0x40..=0x75 | 0x77..=0x7f => mov(state, opcode),
        0x76 => hlt(state),
        0x80..=0x87 => add(state, opcode),
        0xa7 => unimplemented_instruction(state),
        0xaf => unimplemented_instruction(state),
        0xc0 | 0xc8 | 0xc9 |
        0xd0 | 0xd8 | 0xe0 |
        0xe8 | 0xf0 | 0xf8 => ret_cond(state, opcode),
        0xc1 => unimplemented_instruction(state),
        0xc2 | 0xc3 | 0xca |
        0xd2 | 0xda | 0xe2 | 
        0xea | 0xf2 | 0xfa => jmp_cond(state, opcode),
        0xc4 | 0xcc | 0xcd |
        0xd4 | 0xdc | 0xe4 |
        0xec | 0xf4 | 0xfc => call_cond(state, opcode),
        0xc5 | 0xd5 | 0xe5 | 0xf5 => push(state, opcode),
        0xc6 => adi(state),
        0xce => aci(state),
        0xd1 => unimplemented_instruction(state),
        0xd3 => unimplemented_instruction(state),
        0xd6 => sui(state),
        0xde => sbi(state),
        0xe1 => unimplemented_instruction(state),
        0xe6 => ani(state),
        // 0xeb => xchg(state),
        0xed => unimplemented_instruction(state),
        0xee => xri(state),
        0xf1 => unimplemented_instruction(state),
        0xf6 => ori(state),
        0xfb => unimplemented_instruction(state),
        0xfe => cpi(state),
        _ => {
            unimplemented_instruction(state)
        },
    }
    /* print status */
    println!("##### REGISTERS #####");
    println!("A: {:#x}, B: {:#x}, C: {:#x}, D: {:#x}, E: {:#x}, H: {:#x}, L: {:#x}, SP: {:#x}, PC: {:#x}",
            state.registers.a, state.registers.b, state.registers.c, state.registers.d,
    state.registers.e, state.registers.h, state.registers.l, state.registers.sp, state.registers.pc);
    println!("##### FLAGS #####");
    println!("S: {}, Z: {}, P: {}, CY: {}, AC: {}\n",
            state.flags.s, state.flags.z, state.flags.p, state.flags.cy, state.flags.ac);
    // if opcode == 0xce {
    //     process::exit(1);
    // }
}

