#[derive(Debug)]
pub struct Flags {
    pub s: u8,
    pub z: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8,
}

impl Flags {
    fn new() -> Self {
        Flags { s: 0, z: 0, p: 0, cy: 0, ac: 0, pad: 0, }
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: usize,// should be u16 but set to usize for convenience
    pub pc: usize, // should be u16 but set to usize for convenience
}

#[derive(Debug)]
pub struct State {
    pub registers: Registers,
    pub memory: Vec<u8>, // process 8 bits(1 byte) at a time
    pub flags: Flags,
}

impl State {
    // TODO: set pc default to 0
    pub fn new() -> Self {
        State { registers: Registers::default(), memory: vec![0; 6400], flags: Flags::new()}
    }
}
