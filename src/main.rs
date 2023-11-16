#![allow(warnings)]

use crate::state::read_file_to_memory;

pub mod state;

fn main() {
    let state = &mut state::State::new();
    read_file_to_memory();
    dbg!(&state);
    state::emulate8080(state);
}
