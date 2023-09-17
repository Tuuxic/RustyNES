use std::time::Duration;

use crate::{
    bus_mod::bus::CpuRAM,
    cpu_mod::{cpu::Cpu, cpu6502::Cpu6502, disassembler::Disassembler},
    iodevice::IODevice,
};

pub const FRAME_LENGTH: Duration = Duration::from_millis(1); // Duration::new(0, 16_666_666);

pub struct Nes {
    ram: CpuRAM,
    cpu: Cpu6502,
    //clock: &Clock
    frame_delta_time: f64,
}

impl Nes {
    pub fn new() -> Nes {
        let ram: CpuRAM = CpuRAM::new();
        let cpu: Cpu6502 = Cpu6502::new();
        Nes {
            ram,
            cpu,
            frame_delta_time: 0.0,
        }
    }

    pub fn init(&mut self) {
        let program: Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];
        let offset: u16 = 0x8000;

        for (i, num) in program.into_iter().enumerate() {
            self.ram.ram[(offset as usize) + i] = num;
        }

        self.ram.ram[0xFFFC] = 0x00;
        self.ram.ram[0xFFFD] = 0x80;
        self.reset();

    }

    pub fn update(&mut self, dt: Duration) {
        self.frame_delta_time += dt.as_secs_f64();
        if self.frame_delta_time > FRAME_LENGTH.as_secs_f64() {
            self.clock();
            self.frame_delta_time = 0.0;
        }
    }

    pub fn reset(&mut self) {
        let mut io = IODevice::new(&mut self.ram);
        self.cpu.reset(&mut io)
    }

    pub fn get_debug(&mut self) -> String {
        let mut io = IODevice::new(&mut self.ram);
        let (dissasemler, instructions) = Disassembler::dissassemble(0x0000, 0xFFFF, &mut io);
        let range: u16 = 12;
        let mut str: String = String::from(""); 

        let pc_index = dissasemler[&(self.cpu.pc)];

        for i in 1..(range + 1) {
            
            let instr = {
                if pc_index < (range + 1 - i) {
                    (instructions.len() - 1) - (((range + 1 - i) - pc_index) as usize)
                } else {
                    (pc_index - (range + 1 - i)) as usize
                }
            };
            str.push_str(&instructions[instr]);
            str.push_str("\n");
        }

        str.push_str("> ");
        str.push_str(&instructions[pc_index as usize]);
        str.push_str("\n");

        for i in 1..(range + 1) {
            let instr = (pc_index + i) as usize % instructions.len();
            str.push_str(&instructions[instr]);
            str.push_str("\n");
        }

        str
    }

    pub fn get_debug_registers(&mut self) -> String{
        let mut str = String::from("");

        str.push_str(&["Status: ", &format!("{:08b}", self.cpu.status), "\n"].join(""));
        str.push_str(&["PC: 0x", &Disassembler::hex(self.cpu.pc as u32, 4), "\n"].join(""));
        str.push_str(&["A: 0x", &Disassembler::hex(self.cpu.a as u32, 2), " [", &self.cpu.a.to_string(), "]", "\n"].join(""));
        str.push_str(&["X: 0x", &Disassembler::hex(self.cpu.x as u32, 2), " [", &self.cpu.x.to_string(), "]", "\n"].join(""));
        str.push_str(&["Y: 0x", &Disassembler::hex(self.cpu.y as u32, 2), " [", &self.cpu.y.to_string(), "]", "\n"].join(""));
        str.push_str(&["StackPtr: 0x", &Disassembler::hex(self.cpu.stkp as u32, 4)].join(""));

        str
    }

    pub fn get_debug_ram(&mut self, start: u16, rows: u32, cols: u32) -> String {
        let mut io = IODevice::new(&mut self.ram);
        let mut str = String::from("");
        let mut offset = 0;
        for _ in 0..rows {
            str.push_str(&["0x", &Disassembler::hex((start + offset) as u32, 4), ":"].join(""));
            for _ in 0..cols {
                str.push_str(&[" ", &Disassembler::hex(io.read(start + offset) as u32, 2)].join(""));
                offset += 1;
            }
            str.push_str("\n");
        }

        str

    }

    fn clock(&mut self) {
        let mut io = IODevice::new(&mut self.ram);
        // self.cpu.clock(&mut self.ram);
        self.cpu.clock(&mut io);
    }
}
