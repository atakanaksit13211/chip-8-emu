extern crate sdl2;
use log::{info, trace, warn};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
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
}

struct Display {
    scr: [bool; 64 * 32],
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

impl Machine { //IN PROGRESS
    fn simulate_instr(&mut self, amount: u32) {
        let inst: [u8; 2] = self.fetch(self.index);
        self.registers.index += 2;

        let decoded = decode(&inst);
        match decoded {}
    }

    fn fetch(&self, index: u16) {}

    fn decode(instruction: [u8; 2]) -> {}
}

fn main() {
    let mut mem: Memory = Memory::default();
    let mut reg: Registers = Registers {
        register: [0; 16],
        index: 0,
    };
    let mut scr: Display = Display {
        scr: [false; 64 * 32],
    };
    let mut tim: Timers = Timers {};
    let mut test: Machine = Machine {
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

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
