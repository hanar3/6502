// Follows the standard of the classic 6502 CPU chip
pub struct CPU {
  pub register_a: u8,
  pub register_x: u8,
  pub status: u8,
  pub register_y: u8,
  pub program_counter: u16,
  memory: [u8; 0xFFFF],
}

pub enum Flag {
  Negative,
  Overflow,
  Decimal,
  InterruptDisable,
  Zero,
  Carry,
}

pub enum AddressingMode {
  Immediate,
  ZeroPage,
  Absolute,
  ZeroPageX,
  ZeroPageY,
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      register_a: 0,
      register_x: 0,
      register_y: 0,
      status: 0,
      program_counter: 0,
      memory: [0; 0xFFFF],
    }
  }

  pub fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.program_counter,
      AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
      AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
      AddressingMode::ZeroPageX => {
        let base = self.mem_read(self.program_counter);
        let addr = base.wrapping_add(self.register_x) as u16;
        addr
      }
      AddressingMode::ZeroPageY => {
        let base = self.mem_read(self.program_counter);
        let addr = base.wrapping_add(self.register_y) as u16;
        addr
      }
      _ => panic!("Addressing mode not implemented!")
    }
  }

  pub fn get_flag(&self, flag: &Flag) -> u8 {
    match flag {
      Flag::Negative => self.status & 0b1000_0000,
      Flag::Overflow => self.status & 0b0100_0000,
      Flag::Decimal => self.status & 0b0000_1000,
      Flag::InterruptDisable => self.status & 0b0000_0100,
      Flag::Zero => self.status & 0b0000_0010,
      Flag::Carry => self.status & 0b0000_0001,
    }
  }

  pub fn set_flag(&mut self, flag: &Flag) {
    match flag {
      Flag::Negative => self.status = self.status | 0b1000_0000,
      Flag::Overflow => self.status = self.status | 0b0100_0000,
      Flag::Decimal => self.status = self.status | 0b0000_1000,
      Flag::InterruptDisable => self.status = self.status | 0b0000_0100,
      Flag::Zero => self.status = self.status | 0b0000_0010,
      Flag::Carry => self.status = self.status | 0b0000_0001,
    }
  }

  pub fn unset_flag(&mut self, flag: &Flag) {
    match flag {
      Flag::Negative => self.status = self.status & 0b0111_1111,
      Flag::Overflow => self.status = self.status & 0b1011_1111,
      Flag::Decimal => self.status = self.status & 0b1111_0111,
      Flag::InterruptDisable => self.status = self.status & 0b1111_1011,
      Flag::Zero => self.status = self.status & 0b1111_1101,
      Flag::Carry => self.status = self.status & 0b1111_1110,
    }
  }

  fn update_zero_and_negative_flags(&mut self, value: u8) {
    if value == 0 {
      self.set_flag(&Flag::Zero);
    } else {
      self.unset_flag(&Flag::Zero);
    }

    if value & 0b1000_0000 != 0 {
      self.set_flag(&Flag::Negative);
    } else {
      self.unset_flag(&Flag::Negative);
    }
  }

  fn lda(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    self.register_a = self.mem_read(addr);
    self.update_zero_and_negative_flags(self.register_a);
  }

  fn tax(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn inx(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_zero_and_negative_flags(self.register_x)
  }

  fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }

  fn mem_read_u16(&self, pos: u16) -> u16 {
    let lo = self.mem_read(pos) as u16;
    let hi = self.mem_read(pos + 1) as u16;
    (hi << 8) | (lo as u16)
  }

  pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.mem_write(pos, lo);
    self.mem_write(pos + 1, hi);
  }

  pub fn reset(&mut self) {
    self.register_a = 0;
    self.register_x = 0;
    self.status = 0;

    self.program_counter = self.mem_read_u16(0xFFFC);
}

  fn load(&mut self, program: Vec<u8>) {
    self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
    self.mem_write_u16(0xFFFC, 0x8000);
  }

  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run()
  }

  pub fn run(&mut self) {
    loop {
      let opcode = self.mem_read(self.program_counter);
      self.program_counter += 1;

      match opcode {
        0xA9 => {
          self.lda(&AddressingMode::Immediate);
          self.program_counter += 1;
        }

        0xA5 => {
          self.lda(&AddressingMode::ZeroPage);
          self.program_counter += 1;
        }

        0xAA => self.tax(),
        0xE8 => self.inx(),
        0x00 => {
          // BRK
          return;
        }

        _ => todo!(),
      }
    }
  }
}
