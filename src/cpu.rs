use crate::font_set::FONT_SET;
use std::fs::File;
use std::io::Read;

fn get_opcode(memory: [u8; 4096], index: u16) -> u16 {
  (memory[index as usize] as u16) << 8
    | (memory[(index + 1) as usize] as u16)
}

pub struct VramInfo {
  pub draw: bool,
  pub vram: [[u8; 64]; 32]
}
pub struct Cpu {
  // Memory
  memory: [u8; 4096],
  // Registers
  v: [u8; 16],
  // current opcode
  opcode: u16,
  // Index register
  i: u16,
  // Program Counter
  pc: u16,
  // Timers
  delay_timer: u8,
  sound_timer: u8,
  // Stack and pointer
  stack: [u16; 16],
  sp: u16,
  // Graphics (64 x 32 pixel screen)
  vram: [[u8; 64]; 32],
  // flag for screen drawing
  draw_flag: bool
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      memory: [0; 4096],
      v: [0; 16],
      opcode: 0,
      i: 0,
      pc: 0,
      delay_timer: 0,
      sound_timer: 0,
      stack: [0; 16],
      sp: 0,
      vram: [[0; 64]; 32],
      draw_flag: false
    }
  }

  // Reset Cpu to initial state
  pub fn initialize(&mut self) {
    self.memory = [0; 4096];
    self.v = [0; 16];
    self.opcode = 0;
    self.i = 0;
    self.pc = 0x200;
    self.delay_timer = 0;
    self.sound_timer = 0;
    self.stack = [0; 16];
    self.sp = 0;
    self.vram = [[0; 64]; 32];
    self.draw_flag = false;

    for i in 0..80 {
      self.memory[i] = FONT_SET[i];
    }
  }

  pub fn load(&mut self, filename: &str) {
    let mut f = File::open(filename).expect("Could not find file");
    let mut rom = [0u8; 3584];
    f.read(&mut rom).expect("error reading file");

    for (i, &byte) in rom.iter().enumerate() {
      let addr = 0x200 + i;
      self.memory[addr] = byte;
    }
  }

  pub fn decrement_timers(&mut self) {
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }
  }

  pub fn decode_opcode(&mut self, opcode: u16) {
    // TODO: Add opcode details
    let segs = (
      (opcode & 0xF000) >> 12 as u8,
      (opcode & 0x0F00) >> 8 as u8,
      (opcode & 0x00F0) >> 4 as u8,
      (opcode & 0x000F) as u8,
    );

    match segs {
      (0x00, 0x00, 0x0e, 0x00) => self.run_00e0(),
      _ => panic!("failed to account for opcode: {}", opcode)
    }
  }

  pub fn emulate_cycle(&mut self) -> VramInfo {
    self.opcode = get_opcode(self.memory, self.pc);
    self.decode_opcode(self.opcode);
    self.decrement_timers();
    VramInfo {
      draw: self.draw_flag,
      vram: self.vram
    }
  }

  // CLS: clears the screen
  pub fn run_00e0(&mut self) {
    self.vram = [[0; 64]; 32];
    self.draw_flag = true;
    self.pc += 2;
  }
}