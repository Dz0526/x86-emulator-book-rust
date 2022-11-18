extern crate variant_count;

use std::{env, fs::File, io::Read};

use variant_count::VariantCount;

#[derive(VariantCount)]
enum Registers {
    ESP,
}

// can enum length std::mem::variant_count but unstable
struct Emulator {
    eip: u32,
    eflags: u32,
    registers: [u32; Registers::VARIANT_COUNT],
    memory: Vec<u8>,
}

impl Emulator {
    fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        let mut emu = Emulator {
            eip: (eip),
            eflags: (0),
            registers: ([0; Registers::VARIANT_COUNT]),
            memory: vec![0; size],
        };
        emu.registers[Registers::ESP as usize] = esp;

        emu
    }
}

fn main() {
    const MEMORY_SIZE: usize = 1_000_000;

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut emu = Emulator::new(MEMORY_SIZE, 0x0000, 512);

    let mut file = File::open(file_name).expect("File not found");
    file.read_to_end(&mut emu.memory).expect("Cannot read buf");
}
