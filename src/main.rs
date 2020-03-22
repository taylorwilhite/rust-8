extern crate sdl2;
mod cpu;
mod font_set;
mod display;

use cpu::Cpu;
use std::env;
use display::Display;
fn main() {
  let mut chip8 = Cpu::new();
  let sdl_context = sdl2::init().unwrap();
  let mut chip8_display = Display::new(&sdl_context);
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Please supply a filename!");
  }
  let filename = &args[1];

  chip8.initialize();
  chip8.load(&filename);
  let mut count = 0u32;
  loop {
    count += 1;
    if count == 1000000 {
      break;
    }
    let vram_info = chip8.emulate_cycle();
    if vram_info.draw {
      chip8_display.draw(&vram_info.vram);
    }
  }
  
}
