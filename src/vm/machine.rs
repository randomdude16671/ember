pub type Word = i32;

pub struct Machine {
    ip: usize,
    stack: Vec<Word>,
    memory: Vec<Word>,
    program: Vec<u8>,
    halted: bool,
}

impl Machine {
    pub fn new(program: Vec<u8>, memsize: usize) -> Self {
        Self {
            ip: 0,
            stack: Vec::with_capacity(256),
            memory: vec![0; memsize],
            program,
            halted: false,
        }
    }

    #[inline(always)]
    fn fetch_u8(&mut self) -> u8 {
        // Safe indexing with panic on OOB (or return Result)
        let b = *self
            .program
            .get(self.ip)
            .expect("fetch_u8: program counter out of bounds");
        self.ip += 1;
        b
    }
    #[inline(always)]
    fn fetch_u16(&mut self) -> u16 {
        // Fixed bug: u16 is 2 bytes, not 4
        if self.ip + 2 > self.program.len() {
            panic!("fetch_u16: unexpected end of program");
        }
        let lo = self.program[self.ip] as u16;
        let hi = self.program[self.ip + 1] as u16;
        self.ip += 2;
        u16::from_le_bytes([lo as u8, hi as u8])
    }
    #[inline(always)]
    fn fetch_i32(&mut self) -> i32 {
        if self.ip + 4 > self.program.len() {
            panic!("fetch_i32: unexpected end of program");
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.program[self.ip..self.ip + 4]);
        self.ip += 4;
        i32::from_le_bytes(bytes)
    }
    #[inline(always)]
    fn push(&mut self, v: Word) {
        self.stack.push(v);
    }
    #[inline(always)]
    fn pop(&mut self) -> Word {
        self.stack
            .pop()
            .unwrap_or_else(|| panic!("pop: stack underflow"))
    }
    #[inline(always)]
    fn peek(&self) -> Word {
        *self
            .stack
            .last()
            .unwrap_or_else(|| panic!("peek: stack empty"))
    }

    pub fn run_loop(&mut self) {
        while !self.halted {
            println!("IP={}, Stack={:?}", self.ip, self.stack.clone()); // DEBUG
            let op = self.fetch_u8();
            match op {
                0x00 => { /* NO-OP */ }
                0x01 => {
                    let imm = self.fetch_i32();
                    self.push(imm);
                }
                0x02 => {
                    self.pop();
                }
                0x03 => {
                    let v = self.peek();
                    self.push(v);
                }
                0x04 => {
                    // SWAP: require at least 2 elements
                    let n = self.stack.len();
                    if n < 2 {
                        panic!("swap: need at least 2 values");
                    }
                    self.stack.swap(n - 1, n - 2);
                }
                // Arithmetic ops
                0x10 => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b.wrapping_add(a));
                }
                0x11 => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b.wrapping_sub(a));
                }
                0x12 => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b.wrapping_mul(a));
                }
                0x13 => {
                    let a = self.pop();
                    let b = self.pop();
                    if a == 0 {
                        panic!("divide by zero");
                    }
                    self.push(b.wrapping_div(a));
                }
                // Memory ops
                0x20 => {
                    let addr = self.fetch_u8() as usize;
                    let val = self.memory.get(addr).copied().unwrap_or(0);
                    self.push(val);
                }
                0x21 => {
                    let addr = self.fetch_u8() as usize;
                    let v = self.pop();
                    if addr >= self.memory.len() {
                        panic!("memory store out of bounds");
                    }
                    self.memory[addr] = v;
                }
                0x22 => {
                    let addr = self.pop() as usize;
                    let val = *self
                        .memory
                        .get(addr)
                        .unwrap_or_else(|| panic!("memory load indirect OOB"));
                    self.push(val);
                }
                0x23 => {
                    let addr = self.pop() as usize;
                    let val = self.pop();
                    if addr >= self.memory.len() {
                        panic!("memory store indirect OOB");
                    }
                    self.memory[addr] = val;
                }
                // Control flow
                0x30 => {
                    let target = self.fetch_u16() as usize;
                    // Optional: check target < program.len()
                    self.ip = target;
                }
                0x31 => {
                    let target = self.fetch_u16() as usize;
                    let cond = self.pop();
                    if cond == 0 {
                        self.ip = target;
                    }
                }
                0x32 => {
                    let target = self.fetch_u16() as usize;
                    let cond = self.pop();
                    if cond != 0 {
                        self.ip = target;
                    }
                }
                0x40 => {
                    let target = self.fetch_u16() as usize;
                    let ret = self.ip as i32;
                    self.push(ret);
                    self.ip = target;
                }
                0x41 => {
                    let ret = self.pop();
                    self.ip = ret as usize;
                }
                0x50 => {
                    let v = self.pop();
                    println!("{}", v);
                }
                0xFF => {
                    self.halted = true;
                }
                other => {
                    panic!("unknown opcode {:#x} at {}", other, self.ip - 1);
                }
            }
        }
    }
}
