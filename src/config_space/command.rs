use alloc::vec;
use alloc::{format, string::String, vec::Vec};

use crate::config_space::shared::format_flags_line;

use super::shared::BitVecFieldDescriptor;

const COMMAND_FIELDS: [BitVecFieldDescriptor; 12] = [
    // I/O Space
    BitVecFieldDescriptor {
        len: 1,
        name: "I/O",
        is_reserved: false,
    },
    // Memory Space
    BitVecFieldDescriptor {
        len: 1,
        name: "Mem",
        is_reserved: false,
    },
    BitVecFieldDescriptor {
        len: 1,
        name: "BusMaster",
        is_reserved: false,
    },
    // Special Cycles
    BitVecFieldDescriptor {
        len: 1,
        name: "SpecCycle",
        is_reserved: false,
    },
    // Memory Write and Invalidate Enable
    BitVecFieldDescriptor {
        len: 1,
        name: "MemWINV",
        is_reserved: false,
    },
    // VGA Palette Snoop
    BitVecFieldDescriptor {
        len: 1,
        name: "VGASnoop",
        is_reserved: false,
    },
    // Parity Error Response
    BitVecFieldDescriptor {
        len: 1,
        name: "ParErr",
        is_reserved: false,
    },
    BitVecFieldDescriptor {
        len: 1,
        name: "Reserved 1",
        is_reserved: true,
    },
    // SERR# Enable
    BitVecFieldDescriptor {
        len: 1,
        name: "SERR",
        is_reserved: false,
    },
    // Fast Back-to-Back Enable
    BitVecFieldDescriptor {
        len: 1,
        name: "FastB2B",
        is_reserved: false,
    },
    // Interrupt Disable
    BitVecFieldDescriptor {
        len: 1,
        name: "DisINTx",
        is_reserved: false,
    },
    BitVecFieldDescriptor {
        len: 4,
        name: "Reserved 2",
        is_reserved: true,
    },
];

pub struct CommandRegister {
    vector: u16,
}

impl From<u16> for CommandRegister {
    fn from(value: u16) -> Self {
        Self { vector: value }
    }
}

pub struct CommandPrettyPrinter {}

impl CommandPrettyPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&self, cmd: &CommandRegister) -> String {
        let mut offset = 0;
        let mut fields_enabled: Vec<String> = vec![];
        let mut fields_disabled: Vec<String> = vec![];

        for desc in COMMAND_FIELDS {
            let enabled = (cmd.vector >> offset) & 0x1;

            if desc.is_reserved {
                offset += desc.len;
                continue;
            }

            match enabled {
                1 => {
                    let field = format!("+{}", desc.name);
                    fields_enabled.push(field);
                }
                _ => {
                    let field = format!("-{}", desc.name);
                    fields_disabled.push(field);
                }
            };

            offset += desc.len;
        }

        format_flags_line(fields_enabled, fields_disabled)
    }
}
