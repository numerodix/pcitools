use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BitVecFieldDescriptor {
    pub len: usize,
    pub name: &'static str,
    pub is_reserved: bool,
}

pub fn format_flags_line(set: Vec<String>, cleared: Vec<String>) -> String {
    if set.len() > 1 && cleared.len() == 0 {
        set.join(" ")
    } else if set.len() == 0 && cleared.len() > 1 {
        cleared.join(" ")
    } else {
        let set_joined = set.join(" ");
        let cleared_joined = cleared.join(" ");
        format!("{} {}", set_joined, cleared_joined)
    }
}
