extern crate sdl2;
mod cpu;
mod font_set;
mod display;
mod input;

use cpu::Cpu;
use std::env;
use std::thread;
use std::time::Duration;
use display::Display;
use input::InputDriver;
fn main() {
  let sdl_context = sdl2::init().unwrap();
  let mut chip8_display = Display::new(&sdl_context);
  let chip8_keyboard = InputDriver::new(&sdl_context);
  let mut chip8 = Cpu::new(chip8_keyboard);
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Please supply a filename!");
  }
  let filename = &args[1];

  chip8.initialize();
  chip8.load(&filename);
  while let Ok(_keypad) = chip8.keypad.poll() {
    let vram_info = chip8.emulate_cycle();
    if vram_info.draw {
      chip8_display.draw(&vram_info.vram);
    }

    thread::sleep(Duration::from_millis(2))
  }
  
}
