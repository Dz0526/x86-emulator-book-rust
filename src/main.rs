extern crate variant_count;

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use variant_count::VariantCount;

#[derive(VariantCount)]
enum Registers {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
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

    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
        let imm = self.get_code32(1);
        self.registers[reg as usize] = imm;
        self.eip += 5;
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1) as u32;
        self.eip = self.eip.wrapping_add(diff + 2);
    }

    fn get_code8(&self, index: i32) -> u8 {
        self.memory[(self.eip + index as u32) as usize]
    }

    fn get_sign_code8(&self, index: i32) -> i8 {
        self.memory[(self.eip + index as u32) as usize] as i8
    }

    fn get_code32(&self, index: i32) -> u32 {
        let mut ret = 0u32;
        for i in 0..3 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8);
        }

        ret
    }

    // for enum
    fn dump_registers(&self) {
        for r in 0..7 {
            println!("{:?} = {:>08x}", &r, self.registers[r as usize]);
        }

        println!("EIP = {:>08X}", self.eip);
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
    const PROGRAM_HEAD: usize = 0x7C00;
    const PROGRAM_SIZE: usize = 512;

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut emu = Emulator::new(MEMORY_SIZE, 0, PROGRAM_HEAD as u32);

    let file = File::open(file_name).expect("File not found");
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; PROGRAM_SIZE];
    reader
        .read(&mut buf)
        .expect(&format!("File {} cannot read", file_name));

    emu.memory[..PROGRAM_SIZE].copy_from_slice(&buf);

    let instructions = InstructionFunc::new();

    while emu.eip < MEMORY_SIZE as u32 {
        let code = emu.get_code8(0) as usize;
        println!("EIP = {:X}, Code = {:>02X}", emu.eip, code);

        if let Some(f) = instructions[code] {
            f(&mut emu);
        } else {
            println!("\nNot implemented");
            break;
        }

        if emu.eip == 0x0000 {
            println!("\nend of program");
            break;
        }
    }

    emu.dump_registers();
}
