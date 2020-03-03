mod cpu;
mod font_set;

use cpu::Cpu;
fn main() {
  let mut chip8 = Cpu::new();

  chip8.initialize();
  chip8.emulate_cycle();
}
