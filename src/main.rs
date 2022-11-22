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

    fn mov_r32_imm32(&mut self) {}

    fn short_jump(&mut self) {}

    fn get_code8(&self, index: i32) -> u8 {
        self.memory[(self.eip + index as u32) as usize]
    }
}

type InstructionFunc = [Option<fn(&mut Emulator)>; 256];

trait New {
    fn new() -> InstructionFunc;
}
impl New for InstructionFunc {
    fn new() -> InstructionFunc {
        let mut instructions: InstructionFunc = [None; 256];
        for i in 0..8 {
            instructions[0xB8 + i] = Some(Emulator::mov_r32_imm32);
        }
        instructions[0xEB] = Some(Emulator::short_jump);

        instructions
    }
}

fn main() {
    const MEMORY_SIZE: usize = 1_000_000;

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut emu = Emulator::new(MEMORY_SIZE, 0x0000, 512);

    let mut file = File::open(file_name).expect("File not found");
    file.read_to_end(&mut emu.memory).expect("Cannot read buf");

    let instructions = InstructionFunc::new();

    while emu.eip < MEMORY_SIZE as u32 {
        let code = emu.get_code8(0) as usize;

        if let Some(f) = instructions[code] {
            f(&mut emu);
        } else {
            println!("Not implemented");
            break;
        }

        if emu.eip == 0x0000 {
            println!("end of program");
            break;
        }
    }
}
