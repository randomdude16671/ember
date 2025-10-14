pub struct Asm {
    code: Vec<u8>,
}

impl Asm {
    fn new() -> Self {
        Self { code: Vec::new() }
    }

    fn bytes(&mut self, b: &[u8]) {
        self.code.extend_from_slice(b);
    }
    fn u8(&mut self, v: u8) {
        self.code.push(v);
    }
    fn u16(&mut self, v: u16) {
        self.code.extend_from_slice(&v.to_le_bytes());
    }
    fn i32(&mut self, v: i32) {
        self.code.extend_from_slice(&v.to_le_bytes());
    }

    fn push_i32(&mut self, v: i32) {
        self.u8(0x01);
        self.i32(v);
    }
    fn pop(&mut self) {
        self.u8(0x02);
    }
    fn dup(&mut self) {
        self.u8(0x03);
    }
    fn swap(&mut self) {
        self.u8(0x04);
    }

    fn add(&mut self) {
        self.u8(0x10);
    }
    fn sub(&mut self) {
        self.u8(0x11);
    }
    fn mul(&mut self) {
        self.u8(0x12);
    }
    fn div(&mut self) {
        self.u8(0x13);
    }

    fn load(&mut self, addr: u16) {
        self.u8(0x20);
        self.u16(addr);
    }
    fn store(&mut self, addr: u16) {
        self.u8(0x21);
        self.u16(addr);
    }

    fn jmp(&mut self, target: u16) {
        self.u8(0x30);
        self.u16(target);
    }
    fn jz(&mut self, target: u16) {
        self.u8(0x31);
        self.u16(target);
    }
    fn jnz(&mut self, target: u16) {
        self.u8(0x32);
        self.u16(target);
    }

    fn call(&mut self, target: u16) {
        self.u8(0x40);
        self.u16(target);
    }
    fn ret(&mut self) {
        self.u8(0x41);
    }

    fn print(&mut self) {
        self.u8(0x50);
    }
    fn halt(&mut self) {
        self.u8(0xFF);
    }

    fn finish(self) -> Vec<u8> {
        self.code
    }
}
