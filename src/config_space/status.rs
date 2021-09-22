use alloc::vec;
use alloc::{format, string::String, vec::Vec};

use crate::config_space::shared::format_flags_line;

use super::shared::BitVecFieldDescriptor;

const STATUS_FIELDS: [BitVecFieldDescriptor; 13] = [
    BitVecFieldDescriptor {
        len: 3,
        name: "Reserved 1",
        is_reserved: true,
    },
    // Interrupt status
    BitVecFieldDescriptor {
        len: 1,
        name: "INTx",
        is_reserved: false,
    },
    // Capabilities List
    BitVecFieldDescriptor {
        len: 1,
        name: "Cap",
        is_reserved: false,
    },
    // 66 MHz Capable
    BitVecFieldDescriptor {
        len: 1,
        name: "66MHz",
        is_reserved: false,
    },
    // Reserved
    BitVecFieldDescriptor {
        len: 1,
        name: "Reserved 2",
        is_reserved: true,
    },
    // Fast Back-to-Back Capable
    BitVecFieldDescriptor {
        len: 1,
        name: "FastB2B",
        is_reserved: false,
    },
    // Master Data Parity Error
    BitVecFieldDescriptor {
        len: 1,
        name: "MasterDataParErr",
        is_reserved: false,
    },
    // DEVSEL Timing
    BitVecFieldDescriptor {
        len: 1,
        name: "DEVSEL",
        is_reserved: false,
    },
    // Signaled Target Abort
    BitVecFieldDescriptor {
        len: 1,
        name: "SigTAbrt",
        is_reserved: false,
    },
    // Received Target Abort
    BitVecFieldDescriptor {
        len: 1,
        name: "RecvTAbrt",
        is_reserved: false,
    },
    // Received Master Abort
    BitVecFieldDescriptor {
        len: 1,
        name: "RecvMAbrt",
        is_reserved: false,
    },
    // Signaled System Error
    BitVecFieldDescriptor {
        len: 1,
        name: "SigSysErr",
        is_reserved: false,
    },
    // Detected Parity Error
    BitVecFieldDescriptor {
        len: 1,
        name: "ParErr",
        is_reserved: false,
    },
];

pub struct StatusRegister {
    vector: u16,
}

impl From<u16> for StatusRegister {
    fn from(value: u16) -> Self {
        Self { vector: value }
    }
}

pub struct StatusPrettyPrinter {}

impl StatusPrettyPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&self, cmd: &StatusRegister) -> String {
        let mut offset = 0;
        let mut fields_enabled: Vec<String> = vec![];
        let mut fields_disabled: Vec<String> = vec![];

        for desc in STATUS_FIELDS {
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
