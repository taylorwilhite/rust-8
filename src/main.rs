mod cpu;
mod font_set;

use cpu::Cpu;
use std::env;
fn main() {
  let mut chip8 = Cpu::new();
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Please supply a filename!");
  }
  let filename = &args[1];

  chip8.initialize();
  chip8.load(&filename);
  chip8.emulate_cycle();
}
