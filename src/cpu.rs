use crate::font_set::FONT_SET;

pub fn get_opcode(memory: [u8; 4096], index: u16) -> u16 {
  (memory[index as usize] as u16) << 8
    | (memory[(index + 1) as usize] as u16)
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
  gfx: [u8; 2048]
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
      gfx: [0; 2048]
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

    for i in 0..80 {
      self.memory[i] = FONT_SET[i];
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
  }

  pub fn emulate_cycle(&mut self) {
    self.opcode = get_opcode(self.memory, self.pc);
    self.decode_opcode(self.opcode);
    self.decrement_timers();
  }
}