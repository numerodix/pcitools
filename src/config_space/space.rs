use core::fmt::LowerHex;

use alloc::{format, vec};
use alloc::{string::String, vec::Vec};

use crate::config_space::command::CommandPrettyPrinter;
use crate::config_space::header_type::{HeaderTypePrettyPrinter, HeaderTypeRegister};
use crate::config_space::status::{StatusPrettyPrinter, StatusRegister};
use crate::config_space::CommandRegister;

enum FieldKind {
    BitField,
    CommandRegister,
    StatusRegister,
    HeaderTypeRegister,
    AddressField,
    IntField,
    IdField,
}

struct FieldDescriptor {
    len: usize,
    name: &'static str,
    kind: FieldKind,
}

const HeaderType00: [FieldDescriptor; 12] = [
    FieldDescriptor {
        len: 2,
        name: "vendor_id",
        kind: FieldKind::IdField,
    },
    FieldDescriptor {
        len: 2,
        name: "device_id",
        kind: FieldKind::IdField,
    },
    FieldDescriptor {
        len: 2,
        name: "command",
        kind: FieldKind::CommandRegister,
    },
    FieldDescriptor {
        len: 2,
        name: "status",
        kind: FieldKind::StatusRegister,
    },
    FieldDescriptor {
        len: 1,
        name: "revision",
        kind: FieldKind::AddressField,
    },
    FieldDescriptor {
        len: 1,
        name: "prog_if",
        kind: FieldKind::IdField,
    },
    FieldDescriptor {
        len: 1,
        name: "subclass",
        kind: FieldKind::IdField,
    },
    FieldDescriptor {
        len: 1,
        name: "class",
        kind: FieldKind::IdField,
    },
    FieldDescriptor {
        len: 1,
        name: "cache_line_size",
        kind: FieldKind::IntField,
    },
    FieldDescriptor {
        len: 1,
        name: "latency_timer",
        kind: FieldKind::IntField,
    },
    FieldDescriptor {
        len: 1,
        name: "header_type",
        kind: FieldKind::HeaderTypeRegister,
    },
    FieldDescriptor {
        len: 1,
        name: "bist",
        kind: FieldKind::BitField,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(packed)]
pub struct ConfigSpace {
    slice: Vec<u8>,
    // // byte 0
    // pub vendor_id: u16,
    // pub device_id: u16,

    // // byte 1
    // pub command: u16,
    // pub status: u16,

    // // byte 2
    // pub revision: u8,
    // pub prog_if: u8,
    // pub subclass: u8,
    // pub class: u8,

    // // byte 3
    // pub cache_line_size: u8,
    // pub latency_timer: u8,
    // pub header_type: u8,
    // pub bist: u8,

    // // byte 4
    // pub base_addr_0: u32,
    // // byte 5
    // pub base_addr_1: u32,
    // // byte 6
    // pub base_addr_2: u32,
    // // byte 7
    // pub base_addr_3: u32,
    // // byte 8
    // pub base_addr_4: u32,
    // // byte 9
    // pub base_addr_5: u32,

    // // byte 10
    // pub cardbus_cis_pointer: u32,

    // // byte 11
    // pub subsystem_vendor_id: u16,
    // pub subsystem_device_id: u16,

    // // byte 12
    // pub expansion_rom_base_addr: u32,

    // // byte 13
    // pub capabilities_pointer: u8,
    // pub reserved1: u8,
    // pub reserved2: u16,

    // // byte 14
    // pub reserved3: u32,

    // // byte 15
    // pub interrupt_line: u8,
    // pub interrupt_pin: u8,
    // pub min_grant: u8,
    // pub max_latency: u8,
}

impl From<Vec<u8>> for ConfigSpace {
    // fn from(bytes: Vec<u8>) -> Self {
    //     assert!(bytes.len() >= 64);

    //     let ptr = bytes.as_ptr() as *const ConfigSpace;
    //     let reference: &ConfigSpace = unsafe { &*ptr };

    //     reference.clone()
    // }

    fn from(bytes: Vec<u8>) -> Self {
        assert!(bytes.len() >= 64);

        ConfigSpace { slice: bytes }
    }
}

pub struct ConfigSpacePrettyPrinter {}

impl ConfigSpacePrettyPrinter {
    pub fn new() -> Self {
        Self {}
    }

    fn assemble_u16(&self, val: &[u8]) -> u16 {
        assert_eq!(2, val.len());
        (val[1] as u16) << 8 | (val[0] as u16)
    }

    fn assemble_u32(&self, val: &[u8]) -> u32 {
        assert_eq!(4, val.len());
        (val[3] as u32) << 24 | (val[0] as u32)
    }

    fn print_value(&self, desc: &FieldDescriptor, slice: &[u8]) -> String {
        match desc.kind {
            FieldKind::CommandRegister => {
                let value = self.assemble_u16(slice);
                let reg = CommandRegister::from(value as u16);
                let printer = CommandPrettyPrinter::new();
                let flags = printer.print(&reg);
                format!("{} [0x{:04x}]", flags, value)
            }
            FieldKind::StatusRegister => {
                let value = self.assemble_u16(slice);
                let reg = StatusRegister::from(value as u16);
                let printer = StatusPrettyPrinter::new();
                let flags = printer.print(&reg);
                format!("{} [0x{:04x}]", flags, value)
            }
            FieldKind::HeaderTypeRegister => {
                let value = slice[0];
                let reg = HeaderTypeRegister::from(value);
                let printer = HeaderTypePrettyPrinter::new();
                let flags = printer.print(&reg);
                format!("{} [0x{:02x}]", flags, value)
            }
            _ => match desc.len {
                1 => {
                    let value = slice[0];
                    format!("0x{:02x}", value)
                }
                2 => {
                    let value = self.assemble_u16(slice);
                    format!("0x{:04x}", value)
                }
                4 => {
                    let value = self.assemble_u32(slice);
                    format!("0x{:08x}", value)
                }
                _ => format!(""),
            },
        }
    }

    pub fn print(&self, cf: &ConfigSpace) -> String {
        let mut offset = 0;
        let mut lines: Vec<String> = vec![];

        for desc in HeaderType00 {
            let low = offset;
            let high = offset + desc.len;
            let slice = &cf.slice[low..high];

            let value_fmt = self.print_value(&desc, slice);

            let line = format!("{:<20}: {}\n", desc.name, value_fmt);
            lines.push(line);

            offset += desc.len;
        }

        lines.join("")
    }
}
