use crate::instructions::*;
use crate::cpu_state::*;

use std::process;
use std::fs;


fn unimplemented_instruction(state: &State) {
    println!("Error: Unimplemented instruction");
    // println!("memory length: {:x}, pc val: {:x}", state.memory.len(), state.registers.pc);
    // println!("instruction: {:x} {:x} {:x}", state.memory[state.registers.pc],  state.memory[state.registers.pc+1],  state.memory[state.registers.pc+2]);
    process::exit(1);
}

pub fn read_file_to_memory(state: &mut State, file: &str, mem_addr: usize) {
    let file_content = fs::read(file).unwrap();
    state.memory.resize(mem_addr + file_content.len(), 0);
    state.memory[mem_addr.. mem_addr+ file_content.len()].copy_from_slice(&file_content);
    state.registers.pc = mem_addr;

}

// TODO: match in hex or binary?
pub fn emulate8080(state: &mut State) {
    let opcode = state.memory[state.registers.pc];
    // println!("current instruction: {:x}", &opcode);
    println!("current instruction: {:#x} {:#x} {:#x}", state.memory[state.registers.pc],  state.memory[state.registers.pc+1],  state.memory[state.registers.pc+2]);
    match opcode {
        0x00 => (),
        0x01 | 0x11 | 0x21 | 0x31=> lxi(state, opcode),
        0x05 => unimplemented_instruction(state),
        0x06 => unimplemented_instruction(state),
        0x09 => unimplemented_instruction(state),
        0x0d => unimplemented_instruction(state),
        0x0e => unimplemented_instruction(state),
        0x0f => unimplemented_instruction(state),
        0x13 => unimplemented_instruction(state),
        0x19 => unimplemented_instruction(state),
        0x1a => unimplemented_instruction(state),
        0x23 => unimplemented_instruction(state),
        0x26 => unimplemented_instruction(state),
        0x29 => unimplemented_instruction(state),
        0x32 => unimplemented_instruction(state),
        0x36 => unimplemented_instruction(state),
        0x3a => unimplemented_instruction(state),
        0x3e => unimplemented_instruction(state),
        0x40..=0x75 | 0x77..=0x7f => mov(state, opcode),
        0x76 => hlt(state),
        0x80..=0x87 => add(state, opcode),
        0xa7 => unimplemented_instruction(state),
        0xaf => unimplemented_instruction(state),
        0xc1 => unimplemented_instruction(state),
        0xc2 => unimplemented_instruction(state),
        0xc3 => jmp(state),
        0xc5 => unimplemented_instruction(state),
        0xc9 => unimplemented_instruction(state),
        0xcd => unimplemented_instruction(state),
        0xd1 => unimplemented_instruction(state),
        0xd3 => unimplemented_instruction(state),
        0xd5 => unimplemented_instruction(state),
        0xe1 => unimplemented_instruction(state),
        0xe5 => unimplemented_instruction(state),
        0xe6 => ani(state),
        0xed => unimplemented_instruction(state),
        0xf1 => unimplemented_instruction(state),
        0xf5 => unimplemented_instruction(state),
        0xfb => unimplemented_instruction(state),
        0xfe => unimplemented_instruction(state),
        _ => {
            unimplemented_instruction(state)
        },
    }
    // state.registers.pc +=1;
    /* print status */
    println!("##### REGISTERS #####");
    println!("A: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {:#x}, PC: {:#x}",
            state.registers.a, state.registers.b, state.registers.c, state.registers.d,
    state.registers.e, state.registers.h, state.registers.l, state.registers.sp, state.registers.pc);
    println!("##### FLAGS #####");
    println!("S: {}, Z: {}, P: {}, CY: {}, AC: {}\n",
            state.flags.s, state.flags.z, state.flags.p, state.flags.cy, state.flags.ac);
}

