use crate::font_set::FONT_SET;
use std::fs::File;
use std::io::Read;
use rand;
use rand::Rng;
use crate::input::InputDriver;

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
  draw_flag: bool,
  // keyboard input
  pub keypad: InputDriver
}

impl Cpu {
  pub fn new(keypad: InputDriver) -> Cpu {
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
      draw_flag: false,
      keypad: keypad
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

    let nnn = opcode & 0x0FFF;
    let kk = (opcode & 0x00FF) as u8;
    let x = segs.1 as usize;
    let y = segs.2 as usize;
    let n = segs.3 as usize;

    match segs {
      (0x00, 0x00, 0x0e, 0x00) => self.run_00e0(),
      (0x00, 0x00, 0x0e, 0x0e) => self.run_00ee(),
      (0x01, _, _, _) => self.run_1nnn(nnn),
      (0x02, _, _, _) => self.run_2nnn(nnn),
      (0x03, _, _, _) => self.run_3xkk(x, kk),
      (0x04, _, _, _) => self.run_4xkk(x, kk),
      (0x05, _, _, 0x00) => self.run_5xy0(x, y),
      (0x06, _, _, _) => self.run_6xkk(x, kk),
      (0x07, _, _, _) => self.run_7xkk(x, kk),
      (0x08, _, _, 0x00) => self.run_8xy0(x, y),
      (0x08, _, _, 0x01) => self.run_8xy1(x, y),
      (0x08, _, _, 0x02) => self.run_8xy2(x, y),
      (0x08, _, _, 0x03) => self.run_8xy3(x, y),
      (0x08, _, _, 0x04) => self.run_8xy4(x, y),
      (0x08, _, _, 0x05) => self.run_8xy5(x, y),
      (0x08, _, _, 0x06) => self.run_8xy6(x),
      (0x08, _, _, 0x07) => self.run_8xy7(x, y),
      (0x08, _, _, 0x0e) => self.run_8xye(x),
      (0x09, _, _, 0x00) => self.run_9xy0(x, y),
      (0x0a, _, _, _) => self.run_annn(nnn),
      (0x0b, _, _, _) => self.run_bnnn(nnn),
      (0x0c, _, _, _) => self.run_cxkk(x, kk),
      (0x0d, _, _, _) => self.run_dxyn(x, y, n),
      (0x0e, _, 0x09, 0x0e) => self.run_ex9e(x),
      (0x0e, _, 0x0A, 0x01) => self.run_exa1(x),
      (0x0f, _, 0x00, 0x07) => self.run_fx07(x),
      (0x0f, _, 0x00, 0x0a) => self.run_fx0a(x),
      (0x0f, _, 0x01, 0x05) => self.run_fx15(x),
      (0x0f, _, 0x01, 0x08) => self.run_fx18(x),
      (0x0f, _, 0x01, 0x0e) => self.run_fx1e(x),
      (0x0f, _, 0x02, 0x09) => self.run_fx29(x),
      (0x0f, _, 0x03, 0x03) => self.run_fx33(x),
      (0x0f, _, 0x05, 0x05) => self.run_fx55(x),
      (0x0f, _, 0x06, 0x05) => self.run_fx65(x),
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

  // RET: returns from a subroutine
  pub fn run_00ee(&mut self) {
    self.pc = self.stack[self.sp as usize];
    self.sp -= 1;
  }

  // JP: jump to location addr
  pub fn run_1nnn(&mut self, nnn: u16) {
    self.pc = nnn;
  }

  // CALL: call subroutine at nnn
  pub fn run_2nnn(&mut self, nnn: u16) {
    self.sp += 1;
    self.stack[self.sp as usize] = self.pc;
    self.pc = nnn;
  }

  // SE Vx: skip next instruction if register x equals kk
  pub fn run_3xkk(&mut self, x: usize, kk: u8) {
    if self.v[x] == kk {
      self.pc += 4;
    } else {
      self.pc += 2;
    }
  }

  // SNE Vx: Skip next instruction if register x does not equal kk
  pub fn run_4xkk(&mut self, x: usize, kk: u8) {
    if self.v[x] != kk {
      self.pc += 4;
    } else {
      self.pc += 2
    }
  }

  // SE Vx Vy: Skip next instruction if regeister x does not equal register y
  pub fn run_5xy0(&mut self, x: usize, y: usize) {
    if self.v[x] == self.v[y] {
      self.pc += 4;
    } else {
      self.pc += 2;
    }
  }

  // LD Vx: set Vx equal to kk
  pub fn run_6xkk(&mut self, x: usize, kk: u8) {
    self.v[x] = kk;
    self.pc += 2
  }

  // ADD Vx: adds the value of kk to Vx
  pub fn run_7xkk(&mut self, x: usize, kk: u8) {
    let val = kk as u16;
    let vx = self.v[x] as u16;
    let result = val + vx;
    self.v[x] = result as u8;
    self.pc += 2;
  }

  // LD Vx Vy: Set value of Vx to value of Vy
  pub fn run_8xy0(&mut self, x: usize, y: usize) {
    self.v[x] = self.v[y];
    self.pc += 2;
  }

  //OR Vx Vy: Set value of Vx to bitwise OR of Vx and Vy
  pub fn run_8xy1(&mut self, x: usize, y: usize) {
    self.v[x] = self.v[x] | self.v[y];
    self.pc += 2;
  }

  //And Vx Vy: Set Vx equal to bitwise AND of Vx and Vy
  pub fn run_8xy2(&mut self, x: usize, y: usize) {
    self.v[x] = self.v[x] & self.v[y];
    self.pc += 2;
  }

  // XOR Vx Vy: Set Vx equal to bitwise XOR of Vx and Vy
  pub fn run_8xy3(&mut self, x: usize, y: usize) {
    self.v[x] = self.v[x] ^ self.v[y];
    self.pc += 2;
  }

  // ADD Vx Vy: add vy to vx, carry if over 255
  pub fn run_8xy4(&mut self, x: usize, y: usize) {
    self.v[x] = (self.v[x] + self.v[y]) as u8;
    self.v[0x0F] = if self.v[x] > 0x0F { 1 } else { 0 };
    self.pc += 2;
  }

  // SUB Vx Vy: Subtract Vx from Vy, carry if under 0
  pub fn run_8xy5(&mut self, x: usize, y: usize) {
    self.v[0x0F] = if self.v[x] > self.v[y] { 1 } else { 0 };
    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    self.pc += 2;
  }

  // SHR Vx: set Vf to last bit of Vx, then divide V[x]
  pub fn run_8xy6(&mut self, x: usize ) {
    self.v[0x0F] = self.v[x] & 1;
    self.v[x] >>= 1;
    self.pc += 2;
  }

  // SUBN Vx Vy: set Vx to Vy minus Vx, carry if under 0
  pub fn run_8xy7(&mut self, x: usize, y:usize) {
    self.v[0x0F] = if self.v[x] < self.v[y] { 1 } else { 0 };
    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    self.pc += 2;
  }

  // SHL Vx: set Vf to most sig digit of Vx, then multiply Vx by 2
  pub fn run_8xye(&mut self, x: usize) {
    self.v[0x0F] = (self.v[x] & 0b10000000) >> 7;
    self.v[x] <<= 1;
    self.pc += 2;
  }

  // SNE Vx Vy: Skip next instruction if Vx is not equal to Vy
  pub fn run_9xy0(&mut self, x: usize, y: usize) {
    if self.v[x] != self.v[y] {
      self.pc += 4;
    } else {
      self.pc += 2;
    }
  }

  // LD I: set value of register I to nnn
  pub fn run_annn(&mut self, nnn: u16) {
    self.i = nnn;
    self.pc += 2;
  }

  // JP v0: set value of pc to nnn + v0
  pub fn run_bnnn(&mut self, nnn: u16) {
    self.pc = self.v[0] as u16 + nnn;
  }

  // RND Vx: set Vx to random byte plus kk
  pub fn run_cxkk(&mut self, x: usize, kk: u8) {
    let mut rng = rand::thread_rng();
    self.v[x] = rng.gen::<u8>() & kk;
    self.pc += 2;
  }

  // DRW: Draw pixels to the screen and check colision
  pub fn run_dxyn(&mut self, x: usize, y: usize, n: usize) {
    for byte in 0..n {
      let y = (self.v[y] as usize + byte) % 32;
      for bit in 0..8 {
        let x = (self.v[x] as usize + bit) % 64;
        let block = (self.memory[(self.i + byte as u16) as usize] >> (7 - bit)) & 1;
        self.v[0x0F] |= block & self.vram[y][x];
        self.vram[y][x] ^= block;
      }
    }
    self.draw_flag = true;
    self.pc += 2;
  }

  // SKP Vx: Skip if key from value Vx is pressed
  pub fn run_ex9e(&mut self, x: usize) {
    if self.keypad.keys[self.v[x] as usize] {
      self.pc += 2;
    }
    self.pc += 2;
  }

  // SKNP Vx: Skip if not pressed
  pub fn run_exa1(&mut self, x: usize) {
    if !self.keypad.keys[self.v[x] as usize] {
      self.pc += 2;
    }
    self.pc += 2;
  }

  // LD Vx, DT: set vx to timer value
  pub fn run_fx07(&mut self, x: usize) {
    self.v[x] = self.delay_timer;
    self.pc += 2;
  }

  //LD Vx, K: wait for keypress, set Vx to key value
  pub fn run_fx0a(&mut self, x: usize) {
    for (i,key) in self.keypad.keys.iter().enumerate() {
      if *key {
        self.v[x] = i as u8;
        self.pc += 2;
      }
    }
  }

  // LD DT, Vx: set delay timer to Vx
  pub fn run_fx15(&mut self, x: usize) {
    self.delay_timer = self.v[x];
    self.pc += 2;
  }

  // LD ST, Vx Set sound timer = Vx
  pub fn run_fx18(&mut self, x: usize) {
    self.sound_timer = self.v[x];
    self.pc += 2;
  }

  // ADD I, Vx: Set I = I + Vx
  pub fn run_fx1e(&mut self, x: usize) {
    self.i += self.v[x] as u16;
    self.pc += 2;
  }

  // LD F, Vx: Set I = location of sprite for digit Vx
  pub fn run_fx29(&mut self, x: usize) {
    self.i = self.v[x] as u16 * 5;
    self.pc += 2;
  }

  // LD B, Vx: Store BCD representation of Vx in memory locations I, I+1, and I+2
  pub fn run_fx33(&mut self, x: usize) {
    self.memory[self.i as usize] = self.v[x] / 100;
    self.memory[self.i as usize + 1] = (self.v[x] % 100) / 10;
    self.memory[self.i as usize + 2] = self.v[x] % 10;
    self.pc += 2;
  }

  // LD [I], Vx: Store registers V0 through Vx in memory starting at location I
  pub fn run_fx55(&mut self, x: usize) {
    for i in 0..x + 1 {
      self.memory[self.i as usize + i] = self.v[i];
    }
    self.pc += 2;
  }

  // LD Vx, [I]: Read registers V0 through Vx from memory starting at location I
  pub fn run_fx65(&mut self, x: usize) {
    for i in 0..x + 1 {
      self.v[i] = self.memory[self.i as usize + i]
    }
    self.pc += 2;
  }
}