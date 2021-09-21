use std::fmt;
use std::fs;
use std::path;
use std::u8;

use lspci::Address;
use lspci::ConfigSpace;
use lspci::ConfigSpacePrettyPrinter;

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

    let printer = ConfigSpacePrettyPrinter::new();

    for address in addresses {
        println!(
            "{:02x}:{:02x}.{:x} ",
            address.bus, address.device, address.function
        );

        let conf = scanner.load_space(&address);

        let block = printer.print(&conf);
        let lines = block.split("\n");
        for line in lines {
            println!("  {}", line);
        }
    }
}
