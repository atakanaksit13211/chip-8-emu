extern crate sdl2;
use log::{error, info, trace, warn};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color};
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::surface::{Surface, SurfaceRef};
use std::time::Duration;

struct Memory {
    mem: [u8; 4096],
}

impl Default for Memory {
    fn default() -> Memory {
        const FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let mut mem: Memory = Memory { mem: [0; 4096] };

        mem.mem[0x50..0xA0].copy_from_slice(&FONT[0..]);

        return mem;
    }
}

struct Registers {
    register: [u8; 16],
    index: u16,
    pc: u16,
}

struct Display {
    scr: [bool; 64 * 32],
    changed_flag: bool,
}

impl Display {
    fn clear(&mut self) {
        self.scr = [false; 64 * 32];
        self.changed_flag = true;
    }

    fn convert_u8(&self) -> [u8; 256] {
        let mut result: Vec<u8> = Vec::new();
        let mut byte: u8 = 0;
        for (i, _) in [0..255].iter().enumerate() {
            for (j, _) in [0..7].iter().enumerate() {
                byte += u8::from(self.scr[i + j]);
                byte << 1;
            }
            result.push(byte);
            byte = 0;
        }

        let result: [u8; 256] = result.try_into().expect("");
        return result;
    }

    fn convert(&self) -> [u8; 64 * 32] {
        let mut result: Vec<u8> = Vec::new();
        for (i, _) in [0..64 * 32].iter().enumerate() {
            result.push(self.scr[i] as u8);
        }

        let result: [u8; 64 * 32] = result.try_into().expect("");
        return result;
    }
}

struct Timers {}

struct Stack {
    //TODO
    stack: [u16; 16],
}

impl Stack {
    //TODO
    fn push(val: u16) {}

    fn pop() -> u16 {
        0
    }
}

struct Machine {
    memory: Memory,
    registers: Registers,
    screen: Display,
    timers: Timers,
}

impl Machine {
    //IN PROGRESS
    fn simulate_instr(&mut self) {
        let inst: [u8; 2] = self.fetch(self.registers.pc);
        self.registers.pc += 2;

        self.decode_exec(inst);
    }

    fn fetch(&self, pc: u16) -> [u8; 2] {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(self.memory.mem[pc as usize]);
        vec.push(self.memory.mem[(pc + 1) as usize]);

        let vec: [u8; 2] = vec.try_into().expect("");
        return vec;
    }

    fn decode_exec(&mut self, instruction: [u8; 2]) {
        let b1n1: u8 = instruction[0] & 0b11110000; //byte 1 nyble 1

        //let b1n2: u8 = instruction[0] & 0b0000_1111; //byte 1 nyble 2

        //let b2n1: u8 = instruction[1] & 0b1111_0000; //byte 2 nyble 1
        //let b2n2: u8 = instruction[1] & 0b0000_1111; //byte 2 nyble 2

        match b1n1 {
            0x0 => {
                let merged_inst: u16 = (u16::from(instruction[0]) << 8) + u16::from(instruction[1]);
                if merged_inst == 0x00E0 {
                    self.screen.clear();
                } else {
                    //TODO, Execute machine language subroutine at address
                }
            }
            0x1 => {
                let adress: u16 = (u16::from(instruction[0]) << 8) + u16::from(instruction[1])
                    & 0b0000_1111_1111_1111;
                self.registers.pc = adress;
            }
            0x6 => {
                let b1n2: usize = (instruction[0] & 0b0000_1111).into(); //byte 1 nyble 2
                self.registers.register[b1n2] = instruction[1];
            }
            0x7 => {
                let b1n2: usize = (instruction[0] & 0b0000_1111).into(); //byte 1 nyble 2
                self.registers.register[b1n2] += instruction[1];
            }
            0xA => {
                let adress: u16 = (u16::from(instruction[0]) << 8) + u16::from(instruction[1])
                    & 0b0000_1111_1111_1111;
                self.registers.index = adress;
            }
            0xD => {
                self.screen.changed_flag = true;

                let b1n2: u8 = instruction[0] & 0b0000_1111; //byte 1 nyble 2
                let b2n1: u8 = instruction[1] & 0b1111_0000; //byte 2 nyble 1
                let b2n2: u8 = instruction[1] & 0b0000_1111; //byte 2 nyble 2

                let coord_x = self.registers.register[usize::from(b2n2)] % 64;
                let coord_y = self.registers.register[usize::from(b2n1)] % 32;

                let y_true = coord_y * 64;

                self.registers.register[0xF] = 0;

                let mem_adress: usize = usize::from(self.registers.index);

                let mut spr_data: Vec<u8> = Vec::new();
                for n in [0..b2n2] {
                    spr_data.push(self.memory.mem[mem_adress]);
                }

                for (i, _) in [0..b2n2].iter().enumerate() {
                    if usize::from(coord_y) + i < 32 {
                        //we're writing to the screen and not OOB
                        let spr_col: u8 = spr_data[i];

                        for bit in [0..7] {
                            if usize::from(coord_x) + i < 64 {
                                //we're writing to the screen and not OOB
                                if self.screen.scr[usize::from(coord_x + y_true) + i * 64]
                                    && spr_col > 0
                                //set VF
                                {
                                    self.registers.register[0xF] = 1;
                                }
                                let mask = 1 << i;
                                let bit_is_set = (mask & spr_col) > 0;
                                self.screen.scr[usize::from(coord_x + y_true) + i * 64] ^=
                                    bit_is_set;
                            }
                        }
                    }
                }
            }
            _ => {
                error!("Unknown Instruction!!!!");
            }
        }
    }
}

fn main() {
    let mut instructions_per_second = 500;

    let mut mem: Memory = Memory::default();
    let mut reg: Registers = Registers {
        register: [0; 16],
        index: 0,
        pc: 0,
    };
    let mut scr: Display = Display {
        scr: [false; 64 * 32],
        changed_flag: false,
    };
    let mut tim: Timers = Timers {};
    let mut machine: Machine = Machine {
        memory: mem,
        registers: reg,
        screen: scr,
        timers: tim,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pico 8 test", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut corrected_scr: [u8; 64 * 32];
    let texture_creator = canvas.texture_creator();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        for _ in [0..instructions_per_second] {
            machine.simulate_instr();
        }

        if machine.screen.changed_flag {
            corrected_scr = machine.screen.convert();
            let new_surface: Surface = Surface::from_data(
                &mut corrected_scr[..],
                64,
                32,
                1,
                pixels::PixelFormatEnum::Index1MSB,
            )
            .expect("");

            let texture = Texture::from_surface(&new_surface, &texture_creator).unwrap();

            canvas.copy(&texture, None, None).expect("");
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
