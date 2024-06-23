const MEMORY_SIZE: usize = u16::MAX as usize;

use super::register::Registers;

use std::io::Read;

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            register: Registers::new(),
        }
    }
}