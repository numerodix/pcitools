use alloc::vec;
use alloc::{format, string::String, vec::Vec};

use crate::config_space::shared::format_flags_line;

struct FieldDescriptor {
    mask: u8,
    name: &'static str,
}

const HEADER_TYPE_FIELDS: [FieldDescriptor; 3] = [
    FieldDescriptor {
        mask: 0x01,
        name: "PCItoPCIBridge",
    },
    FieldDescriptor {
        mask: 0x02,
        name: "PCItoCardBusBridge",
    },
    FieldDescriptor {
        mask: 0xff,
        name: "MultiFunction",
    },
];

pub struct HeaderTypeRegister {
    vector: u8,
}

impl From<u8> for HeaderTypeRegister {
    fn from(value: u8) -> Self {
        Self { vector: value }
    }
}

pub struct HeaderTypePrettyPrinter {}

impl HeaderTypePrettyPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&self, cmd: &HeaderTypeRegister) -> String {
        let mut fields_enabled: Vec<String> = vec![];
        let mut fields_disabled: Vec<String> = vec![];

        for desc in HEADER_TYPE_FIELDS {
            match cmd.vector & desc.mask > 0 {
                true => {
                    let field = format!("+{}", desc.name);
                    fields_enabled.push(field);
                }
                _ => {
                    let field = format!("-{}", desc.name);
                    fields_disabled.push(field);
                }
            };
        }

        format_flags_line(fields_enabled, fields_disabled)
    }
}
