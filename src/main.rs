use std::fs;
use std::u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PciAddress {
    bus: u8,
    device: u8,
    function: u8,
}

impl PciAddress {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function,
        }
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

    fn scan(&self) -> Vec<PciAddress> {
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
                let device_path = device_entry.path();

                let parts: Vec<&str> = device_func.to_str().unwrap().split('.').collect();
                println!("{:?}", device_path);

                let bus_no: u8 = u8::from_str_radix(bus_id, 16).unwrap();
                let dev_no: u8 = u8::from_str_radix(parts[0], 16).unwrap();
                let func_no: u8 = u8::from_str_radix(parts[1], 16).unwrap();

                let address = PciAddress::new(bus_no, dev_no, func_no);
                addresses.push(address);
            }
        }

        addresses.sort();

        addresses
    }
}

fn main() {
    let root_dir = "/proc/bus/pci";
    let scanner = Scanner::new(root_dir);
    let addresses = scanner.scan();

    for address in addresses {
        println!("{:02x}:{:02x}.{:x}", address.bus, address.device, address.function);
    }
}
