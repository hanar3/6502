use crate::AddressingMode;
use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Add};
pub struct OpCode {
    pub hex: u8,
    pub name: &'static str,
    pub size: usize,
    pub cycles: usize,
    pub addressing_mode: AddressingMode,
}

impl OpCode {
    fn new(
        hex: u8,
        name: &'static str,
        bytes: usize,
        cycles: usize,
        addressing_mode: AddressingMode,
    ) -> OpCode {
        return OpCode {
            hex: hex,
            name: name,
            size: bytes,
            cycles: cycles,
            addressing_mode: addressing_mode,
        };
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        // LDA
        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA5, "LDA", 2, 2, AddressingMode::ZeroPage),
        OpCode::new(0xB5, "LDA", 2, 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, "LDA", 2, 2, AddressingMode::Absolute),
        OpCode::new(0xBD, "LDA", 2, 2, AddressingMode::Absolute_X),
        OpCode::new(0xB9, "LDA", 2, 2, AddressingMode::Absolute_Y),
        OpCode::new(0xA1, "LDA", 2, 2, AddressingMode::Indirect_X),
        OpCode::new(0xB1, "LDA", 2, 2, AddressingMode::Indirect_Y),

        // TAX
        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::None),

        // INX
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::None),

        //BRK
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::Absolute),

        // STA
        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 3, 5, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 3, 5, AddressingMode::Indirect_Y),

    ];
    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OP_CODES {
            map.insert(cpuop.hex, cpuop);
        }
        map
    };
}
