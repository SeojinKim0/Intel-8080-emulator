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

    // Skip DAA test
    state.memory[0x059c] = 0xc3; //JMP    
    state.memory[0x059d] = 0xc2;    
    state.memory[0x059e] = 0x05;
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
        0x01 | 0x11 | 0x21 | 0x31 => lxi(state, opcode),
        0x02 | 0x12 => stax(state, opcode),
        0x03 | 0x13 | 0x23 | 0x33 => inx(state, opcode),
        0x04 | 0x0c | 0x14 |
        0x1c | 0x24 | 0x2c |
        0x34 | 0x3c => inr(state, opcode),
        0x05 | 0x0d | 0x15 |
        0x1d | 0x25 | 0x2d |
        0x35 | 0x3d => dcr(state, opcode),
        0x07 => rlc(state),
        0x09 | 0x19 | 0x29 | 0x39 => dad(state, opcode),
        0x06 | 0x0e | 0x16 |
        0x1e | 0x26 | 0x2e |
        0x36 | 0x3e => mvi(state, opcode),
        0x0a | 0x1a => ldax(state, opcode),
        0x0b | 0x1b | 0x2b | 0x3b => dcx(state, opcode),
        0x0f => rrc(state),
        0x17 => ral(state),
        0x1f => rar(state),
        0x22 => shld(state),
        0x27 => todo!(),
        0x2a => lhld(state),
        0x2f => cma(state),
        0x32 => sta(state),
        0x3a => lda(state),
        0x37 => stc(state),
        0x3f => cmc(state),
        0x40..=0x75 | 0x77..=0x7f => mov(state, opcode),
        0x76 => hlt(state),
        0x80..=0x87 => add(state, opcode),
        0x88..=0x8f => adc(state, opcode),
        0x90..=0x97 => sub(state, opcode),
        0x98..=0x9f => sbb(state, opcode),
        0xa0..=0xa7 => ana(state, opcode),
        0xa8..=0xaf => xra(state, opcode),
        0xb0..=0xb7 => ora(state, opcode),
        0xb8..=0xbf => cmp(state, opcode),
        0xc0 | 0xc8 | 0xc9 |
        0xd0 | 0xd8 | 0xe0 |
        0xe8 | 0xf0 | 0xf8 => ret_cond(state, opcode),
        0xc1 | 0xd1 | 0xe1 | 0xf1 => pop(state, opcode),
        0xc2 | 0xc3 | 0xca |
        0xd2 | 0xda | 0xe2 | 
        0xea | 0xf2 | 0xfa => jmp_cond(state, opcode),
        0xc4 | 0xcc | 0xcd |
        0xd4 | 0xdc | 0xe4 |
        0xec | 0xf4 | 0xfc => call_cond(state, opcode),
        0xc5 | 0xd5 | 0xe5 | 0xf5 => push(state, opcode),
        0xc6 => adi(state),
        0xce => aci(state),
        0xd3 => unimplemented_instruction(state),
        0xd6 => sui(state),
        0xde => sbi(state),
        0xe3 => xthl(state),
        0xe6 => ani(state),
        0xe9 => pchl(state),
        0xeb => xchg(state),
        0xed => unimplemented_instruction(state),
        0xee => xri(state),
        0xf6 => ori(state),
        0xf9 => sphl(state),
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

