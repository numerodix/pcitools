use std::fs;
use std::path;
use std::u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    // 256 possible buses - fits in 1 byte
    bus: u8,

    // 32 possible devices on a bus - fits in 5 bits
    device: u8,

    // 8 possible functions of a device - fits in 3 bits
    function: u8,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct ConfigSpace {
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision: u8,
    class_code1: u8,
    class_code2: u16,
    cache_line: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
    base_addr_0: u32,
    base_addr_1: u32,
    base_addr_2: u32,
    base_addr_3: u32,
    base_addr_4: u32,
    base_addr_5: u32,
    cardbus_cis_pointer: u32,
    subsystem_vendor_id: u16,
    subsystem_device_id: u16,
    expansion_rom_base_addr: u32,
    reserved1: u64,
    irq_line: u8,
    irq_pin: u8,
    min_grant: u8,
    max_latency: u8,
}

impl ConfigSpace {}

impl From<Vec<u8>> for ConfigSpace {
    fn from(bytes: Vec<u8>) -> Self {
        assert!(bytes.len() >= 64);

        let ptr = bytes.as_ptr() as *const ConfigSpace;
        let reference: &ConfigSpace = unsafe { &*ptr };

        reference.clone()
    }
}

struct Scanner {
    root_dir: String,
}

impl Scanner {
    fn new(root_dir: &str) -> Self {
        Self {
            root_dir: root_dir.to_owned(),
        }
    }

    fn scan(&self) -> Vec<Address> {
        let mut addresses = vec![];

        // enumerate pci buses
        for bus_entry in fs::read_dir(&self.root_dir).unwrap() {
            let bus_entry = bus_entry.unwrap();

            let bus_id_os_str = bus_entry.file_name();
            let bus_id = bus_id_os_str.to_str().unwrap();
            let bus_path = bus_entry.path();

            if !bus_path.is_dir() {
                continue;
            }

            // enumerate pci devices for each bus
            for device_entry in fs::read_dir(&bus_path).unwrap() {
                let device_entry = device_entry.unwrap();

                let device_func = device_entry.file_name();
                let parts: Vec<&str> = device_func.to_str().unwrap().split('.').collect();

                let bus_no: u8 = u8::from_str_radix(bus_id, 16).unwrap();
                let dev_no: u8 = u8::from_str_radix(parts[0], 16).unwrap();
                let func_no: u8 = u8::from_str_radix(parts[1], 16).unwrap();

                let address = Address::new(bus_no, dev_no, func_no);
                addresses.push(address);
            }
        }

        addresses.sort();
        addresses
    }

    fn load_space(&self, address: &Address) -> ConfigSpace {
        let bus_name = format!("{:02x}", address.bus);
        let dev_func = format!("{:02x}.{:x}", address.device, address.function);
        let path = path::Path::new(&self.root_dir)
            .join(bus_name)
            .join(dev_func);

        let bytes = fs::read(&path).unwrap();
        ConfigSpace::from(bytes)
    }
}

fn main() {
    let root_dir = "/proc/bus/pci";
    let scanner = Scanner::new(root_dir);
    let addresses = scanner.scan();

    for address in addresses {
        print!(
            "{:02x}:{:02x}.{:x} ",
            address.bus, address.device, address.function
        );
        let conf = scanner.load_space(&address);
        println!("{:?}", conf);
    }
}
