#[derive(Default)]
struct Clock {
    m: u16, // TODO we are assuming m and t are unsigned 16-bit.
    t: u16
}

#[derive(Default)]
struct Registers {
    // 8-bit registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    l: u8,
    f: u8,
    // 16-bit registers
    pc: u16,
    sp: u16,
    // Last clock state
    m: u16,
    t: u16
}

struct MMU {
    ram: Vec<u8>
}

impl MMU {
    fn new(capacity: usize) -> MMU {
        let mut mmu = MMU {
            ram: vec![]
        };

        mmu.ram.reserve(capacity);
        for i in 0..capacity {
            mmu.ram.push(0);
        }

        mmu
    }

    // Retrieve a byte from memory, indexed according to bytes
    fn rb(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    // Retrieve a 16-bit word from memory, indexed according to words
    fn rw(&self, addr: usize) -> u16 {
        let big = self.ram[addr*2];
        let little = self.ram[addr*2+1];

        // Concatenate both bytes into one word
        let mut result: u16 = (big as u16) << 8;
        result |= little as u16;

        result
    }

    fn wb(&mut self, addr: usize, val: u8) {
        self.ram[addr] = val;
    }

    fn ww(&mut self, addr: usize, val: u16) {
        let big = val >> 8;
        let little = val & 255;

        self.ram[addr*2] = big as u8;
        self.ram[addr*2+1] = little as u8;
    }
}

struct Z80 {
    clock: Clock,
    r: Registers
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            clock: Clock {
                m: 0,
                t: 0
            },
            r: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                l: 0,
                f: 0,
                pc: 0,
                sp: 0,
                m: 0,
                t: 0
            }
        }
    }

    // Add E to A, leaving result in A and checking for a carry
    fn add_r_e(&mut self) {
        // Perform addition with extra bits for the carry bit
        let sum: u16 = (self.r.a + self.r.e).into();
        self.r.f = 0; // Clear flags

        // Check for 0
        if sum & 255 == 0 {
            // Set the zero flag (10000000)
            self.r.f |= 0x80;
        }

        // Check for a carry
        if sum > 255 {
            // Set the underflow/overflow flag (00010000)
            self.r.f |= 0x10; 
            self.r.a = 0;
        }

        self.r.a = (sum & 255) as u8;
        // TODO why are 1 and 4 magic numbers for this?
        self.r.m = 1;
        self.r.t = 4;
    }

    // Compare B to A, settings flags for the result
    fn cp_r_b(&mut self) {
        // Subtract B from A in a temp copy
        let temp: i8 = self.r.a as i8 - self.r.b as i8;
        // Set the subtraction flag (01000000)
        self.r.f |= 0x40;
        // Check for zero
        // NOTE -127 is used because it is 11111111 for i8
        if temp & -127 == 0 {
            // Set the zero flag (10000000)
            self.r.f |= 0x80;
        }
        // Check for underflow
        if temp < 0 {
            // Set the underflow/overflow flag (00010000)
        }
        self.r.m = 1; 
        self.r.t = 4;
    }

    fn nop(&mut self) {
        self.r.m = 1; 
        self.r.t = 4;
    }
}


fn main() {
    println!("Hello, world!");
    let processor = Z80::new();
}