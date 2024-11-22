use core::panic;

pub type ExecutionResult = Option<()>;

// pub enum MemoryPointerResult {
//     Read(u32), // address
//     Write((u32, u8)), // (address, value)
// }

#[derive(Debug)]
pub struct Tomtel {
    a: u8, // Accumulation register
    b: u8, // Operand register
    c: u8, // Count/offset register
    d: u8,
    e: u8,
    f: u8, // Flags register
    la: u32,
    lb: u32,
    lc: u32,
    ld: u32,
    ptr: u32, // pointer to memory
    pc: u32,  // Program counter
    memory: Vec<u8>,
    output_stream: Vec<u8>,
}

impl Tomtel {
    pub fn new(memory_size: usize) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            la: 0,
            lb: 0,
            lc: 0,
            ld: 0,
            ptr: 0,
            pc: 0,
            memory: vec![0; memory_size],
            output_stream: Vec::new(),
        }
    }

    // Opcode: 0xC2 (1 byte)
    pub fn add(&mut self) -> ExecutionResult {
        self.pc += 1;
        self.a = self.a.wrapping_add(self.b);
        None
    }

    // Opcode: 0xE1 0x__ (2 bytes)
    pub fn advance_pointer(&mut self, offset: u8) -> ExecutionResult {
        self.pc += 2;
        // Not using wrapping_add because the overflow behaviour is undefined.
        self.ptr = self.ptr + offset as u32;
        None
    }

    // Opcode: 0xC1 (1 byte)
    pub fn compare(&mut self) -> ExecutionResult {
        self.pc += 1;
        self.f = if self.a == self.b { 0 } else { 0x01 };
        None
    }

    // Opcode: 0x01 (1 byte)
    pub fn halt(&mut self) -> ExecutionResult {
        self.pc += 1;
        Some(())
    }

    // Opcode: 0x21 0x__ 0x__ 0x__ 0x__ (5 bytes)
    pub fn jump_equals_zero(&mut self, address: u32) -> ExecutionResult {
        self.pc += 5;
        if self.f == 0 {
            self.pc = address;
        }
        None
    }

    // Opcode: 0x22 0x__ 0x__ 0x__ 0x__ (5 bytes)
    pub fn jump_not_equals_zero(&mut self, address: u32) -> ExecutionResult {
        self.pc += 5;
        if self.f != 0 {
            self.pc = address;
        }
        None
    }

    // Opcode: 0b01DDDSSS (1 byte)
    pub fn mv(&mut self, source: u8, destination: u8) -> ExecutionResult {
        self.pc += 1;
        let source_register = match source {
            1 => self.a,
            2 => self.b,
            3 => self.c,
            4 => self.d,
            5 => self.e,
            6 => self.f,
            7 => self.memory[self.ptr.wrapping_add(self.c as u32) as usize],
            _ => panic!("Invalid source register: {}", source),
        };

        if destination == 7 {
            println!("Writing to memory: {} -> {}", source_register, self.pc);
            self.memory[self.ptr.wrapping_add(self.c as u32) as usize] = source_register;
            return None;
        }

        match destination {
            1 => self.a = source_register,
            2 => self.b = source_register,
            3 => self.c = source_register,
            4 => self.d = source_register,
            5 => self.e = source_register,
            6 => self.f = source_register,
            _ => panic!(
                "Invalid mv destination register: {} -> {}",
                destination, self.pc
            ),
        }
        None
    }

    // Opcode: 0b10DDDSSS (1 byte)
    pub fn mv32(&mut self, source: u8, destination: u8) -> ExecutionResult {
        self.pc += 1;
        let source_register = match source {
            1 => self.la,
            2 => self.lb,
            3 => self.lc,
            4 => self.ld,
            5 => self.ptr,
            6 => self.pc,
            _ => panic!("Invalid source register: {}", source),
        };

        match destination {
            1 => self.la = source_register,
            2 => self.lb = source_register,
            3 => self.lc = source_register,
            4 => self.ld = source_register,
            5 => self.ptr = source_register,
            6 => self.pc = source_register,
            _ => panic!("Invalid mv32 destination register: {}", destination),
        }

        None
    }

    // Opcode: 0b01DDD000 0x__ (2 bytes)
    pub fn mvi(&mut self, value: u8, destination: u8) -> ExecutionResult {
        self.pc += 2;

        if destination == 7 {
            println!("Writing to memory: {} -> {}", value, self.pc);
            self.memory[self.ptr.wrapping_add(self.c as u32) as usize] = value;
            return None;
        }
        match destination {
            1 => self.a = value,
            2 => self.b = value,
            3 => self.c = value,
            4 => self.d = value,
            5 => self.e = value,
            6 => self.f = value,
            _ => panic!("Invalid mvi destination register: {}", destination),
        }

        None
    }

    // Opcode: 0b01DDD000 0x__ 0x__ 0x__ 0x__ (5 bytes)
    pub fn mvi32(&mut self, value: u32, destination: u8) -> ExecutionResult {
        self.pc += 5;
        match destination {
            1 => self.la = value,
            2 => self.lb = value,
            3 => self.lc = value,
            4 => self.ld = value,
            5 => self.ptr = value,
            6 => self.pc = value,
            _ => panic!("Invalid mvi32 destination register: {}", destination),
        }

        None
    }

    // Opcode: 0x02 (1 byte)
    pub fn output(&mut self) -> ExecutionResult {
        self.pc += 1;
        self.output_stream.push(self.a);
        None
    }

    // Opcode: 0xC3 (1 byte)
    pub fn subtract(&mut self) -> ExecutionResult {
        self.pc += 1;
        self.a = self.a.wrapping_sub(self.b);
        None
    }

    // Opcode: 0xC4 (1 byte)
    pub fn xor(&mut self) -> ExecutionResult {
        self.pc += 1;
        self.a ^= self.b;
        None
    }

    pub fn output_stream(&self) -> Vec<u8> {
        self.output_stream.clone()
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }
}
