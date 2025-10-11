use std::fs;

struct AsmToBinary {
    pub fname: String,
    buffer: String,
    opcodes: Vec<u8>,
}

impl AsmToBinary {
    fn new(file_name: &String) -> Self {
        let buf = fs::read_to_string(file_name).unwrap_or_else(|_x| String::from("error"));
        Self {
            fname: file_name.clone(),
            buffer: buf,
            opcodes: Vec::new(),
        }
    }
}
