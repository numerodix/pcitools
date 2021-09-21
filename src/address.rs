#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    // 256 possible buses - fits in 1 byte
    pub bus: u8,

    // 32 possible devices on a bus - fits in 5 bits
    pub device: u8,

    // 8 possible functions of a device - fits in 3 bits
    pub function: u8,
}

impl Address {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function,
        }
    }
}
