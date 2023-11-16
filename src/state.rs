use std::process;
use std::fs;

#[derive(Debug)]
struct Flags {
    s: u8,
    z: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}

impl Flags {
    fn new() -> Self {
        Flags { s: 1, z: 1, p: 1, cy: 1, ac: 1, pad: 1, }
    }
}

#[derive(Debug, Default)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

#[derive(Debug)]
pub struct State {
    registers: Registers,
    memory: Vec<u8>, // process 8 bits(1 byte) at a time
    cond: Flags,
    int_enable: u8,
}

impl State {
    pub fn new() -> Self {
        State { registers: Registers::default(), memory: vec![], cond: Flags::new(), int_enable: 0 }
    }
}

fn unimplemented_instruction(state: &State) {
    println!("Error: Unimplemented instruction");
    process::exit(1);
}

pub fn read_file_to_memory() {
    let contents = fs::read("cpudiag.bin");
    dbg!(&contents.unwrap()[0]);

}

pub fn emulate8080(state: &mut State) {
    println!("Start emulating");
    let opcode = &state.memory[usize::from(state.registers.pc)];
    match opcode {
        x00 => (),
        x01 => unimplemented_instruction(state),
        x05 => unimplemented_instruction(state),
        x06 => unimplemented_instruction(state),
        x09 => unimplemented_instruction(state),
        x0d => unimplemented_instruction(state),
        x0e => unimplemented_instruction(state),
        x0f => unimplemented_instruction(state),
        x11 => unimplemented_instruction(state),
        x13 => unimplemented_instruction(state),
        x19 => unimplemented_instruction(state),
        x1a => unimplemented_instruction(state),
        x21 => unimplemented_instruction(state),
        x23 => unimplemented_instruction(state),
        x26 => unimplemented_instruction(state),
        x29 => unimplemented_instruction(state),
        x31 => unimplemented_instruction(state),
        x32 => unimplemented_instruction(state),
        x36 => unimplemented_instruction(state),
        x3a => unimplemented_instruction(state),
        x3e => unimplemented_instruction(state),
        x56 => unimplemented_instruction(state),
        x5e => unimplemented_instruction(state),
        x66 => unimplemented_instruction(state),
        x6f => unimplemented_instruction(state),
        x77 => unimplemented_instruction(state),
        x7a => unimplemented_instruction(state),
        x7b => unimplemented_instruction(state),
        x7c => unimplemented_instruction(state),
        x7e => unimplemented_instruction(state),
        xa7 => unimplemented_instruction(state),
        xaf => unimplemented_instruction(state),
        xc1 => unimplemented_instruction(state),
        xc2 => unimplemented_instruction(state),
        xc3 => unimplemented_instruction(state),
        xc5 => unimplemented_instruction(state),
        xc9 => unimplemented_instruction(state),
        xcd => unimplemented_instruction(state),
        xd1 => unimplemented_instruction(state),
        xd3 => unimplemented_instruction(state),
        xd5 => unimplemented_instruction(state),
        xe1 => unimplemented_instruction(state),
        xe5 => unimplemented_instruction(state),
        xe6 => unimplemented_instruction(state),
        xed => unimplemented_instruction(state),
        xf1 => unimplemented_instruction(state),
        xf5 => unimplemented_instruction(state),
        xfb => unimplemented_instruction(state),
        xfe => unimplemented_instruction(state),
        _ => todo!()
    }
    state.registers.pc +=1;
}
