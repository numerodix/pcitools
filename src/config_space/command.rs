use alloc::vec;
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

struct FieldDescriptor {
    len: usize,
    name: &'static str,
    is_reserved: bool,
}

const COMMAND_FIELDS: [FieldDescriptor; 12] = [
    // I/O Space
    FieldDescriptor {
        len: 1,
        name: "I/O",
        is_reserved: false,
    },
    // Memory Space
    FieldDescriptor {
        len: 1,
        name: "Mem",
        is_reserved: false,
    },
    FieldDescriptor {
        len: 1,
        name: "BusMaster",
        is_reserved: false,
    },
    // Special Cycles
    FieldDescriptor {
        len: 1,
        name: "SpecCycle",
        is_reserved: false,
    },
    // Memory Write and Invalidate Enable
    FieldDescriptor {
        len: 1,
        name: "MemWINV",
        is_reserved: false,
    },
    // VGA Palette Snoop
    FieldDescriptor {
        len: 1,
        name: "VGASnoop",
        is_reserved: false,
    },
    // Parity Error Response
    FieldDescriptor {
        len: 1,
        name: "ParErr",
        is_reserved: false,
    },
    FieldDescriptor {
        len: 1,
        name: "Reserved 1",
        is_reserved: true,
    },
    // SERR# Enable
    FieldDescriptor {
        len: 1,
        name: "SERR",
        is_reserved: false,
    },
    // Fast Back-to-Back Enable
    FieldDescriptor {
        len: 1,
        name: "FastB2B",
        is_reserved: false,
    },
    // Interrupt Disable
    FieldDescriptor {
        len: 1,
        name: "DisINTx",
        is_reserved: false,
    },
    FieldDescriptor {
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

        // consider extracting to a function
        let enabled_joined = fields_enabled.join(" ");
        let disabled_joined = fields_disabled.join(" ");
        let pair = vec![enabled_joined, disabled_joined];
        let line = pair.join(" ");
        line.trim().to_string()
    }
}
