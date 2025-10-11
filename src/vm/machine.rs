use std::convert::TryInto;

pub type Word = i32;
pub type Addr = usize;

struct Machine {
    ip: usize,
    stack: Vec<Word>,
    memory: Vec<Word>,
    program: Vec<u8>,
    halted: bool,
}

impl Machine {
    fn new(program: Vec<u8>, memsize: usize) -> Self {
        Self {
            ip: 0,
            stack: Vec::with_capacity(256),
            memory: vec![0; memsize],
            program,
            halted: false,
        }
    }

    fn fetch_u8(&mut self) -> u8 {
        let b = self.program[self.ip];
        self.ip += 1;
        b
    }

    fn fetch_u16(&mut self) -> u16 {
        let bytes = &self.program[self.ip..self.ip + 2];
        self.ip += 2;
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    fn fetch_i32(&mut self) -> i32 {
        let bytes = &self.program[self.ip..self.ip + 2];
        self.ip += 2;
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn push(&mut self, v: Word) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> Word {
        self.stack.pop().expect("Stack overflow")
    }

    fn peek(&mut self) -> Word {
        *self.stack.last().expect("stack empty")
    }

    fn run(&mut self) {
        let op = self.fetch_u8();
        match op {
            0x00 => {} // NO OP
            0x01 => {
                // PUSH
                let imm = self.fetch_i32();
                self.push(imm);
            }
            0x02 => {
                // POP
                self.pop();
            }
            0x03 => {
                // DUP (duplicate)
                let v = self.peek();
                self.push(v);
            }
            0x04 => {
                // SWAP
                let n = self.stack.len();
                if n > 2 {
                    panic!("swap needs 2 values")
                };
                self.stack.swap(n - 1, n - 2);
            }
            // arithemetic ops
            0x10 => {
                // ADD
                let a = self.pop();
                let b = self.pop();
                self.push(b.wrapping_add(a));
            }
            0x11 => {
                // SUB
                let a = self.pop();
                let b = self.pop();
                self.push(b.wrapping_sub(a));
            }
            0x12 => {
                // MUL
                let a = self.pop();
                let b = self.pop();
                self.push(b.wrapping_mul(a));
            }
            0x13 => {
                // DIV
                let a = self.pop();
                let b = self.pop();
                self.push(b.wrapping_div(a));
            }

            0x20 => {
                // LOAD u16 -> push memory[addr]
                let addr = self.fetch_u8() as usize;
                self.push(self.memory.get(addr).copied().unwrap_or(0));
            }
            0x21 => {
                // STORE u16 <- pop
                let addr = self.fetch_u8() as usize;
                let v = self.pop();
                if addr >= self.memory.len() {
                    panic!("MEMORY STORE OOB");
                }
                self.memory[addr] = v;
            }
            0x30 => {
                // JMP u16
                let target = self.fetch_u16() as usize;
                self.ip = target;
            }
            0x31 => {
                // jz u16
                let target = self.fetch_u16() as usize;
                let cond = self.pop();
                if cond == 0 {
                    self.ip = target
                };
            }
            0x32 => {
                // jnz u16
                let target = self.fetch_u16() as usize;
                let cond = self.pop();
                if cond != 0 {
                    self.ip = target;
                }
            }
            0x40 => {
                // call u16 (push ret addr, jmp)
                let target = self.fetch_u16() as usize;
                // push return addr (cur IP)
                let ret = self.ip as i32;
                self.push(ret);
                self.ip = target;
            }
            0x41 => {
                let ret = self.pop();
                self.ip = ret as usize;
            }

            0x50 => {
                // PRINT (pop and print)
                let v = self.pop();
                println!("{}", v);
            }

            0xFF => {
                self.halted = true;
            }
            _ => panic!("unknown opcode {:#x} at {}", op, self.ip - 1),
        }
    }
}
