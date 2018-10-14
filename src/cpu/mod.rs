const RAM_SIZE: usize = 4096;

use fonts;
use std::num::Wrapping;
use rand;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    stack: [usize; 16],
    v: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
    should_increment_pc: bool
}

impl Chip8 {
    /// Constructs a new virtual chip8 CPU, with standard chip8 fonts loaded
    /// into memory
    pub fn new() -> Self {
        let mut cpu = Chip8 {
            ram: [0; RAM_SIZE],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            should_increment_pc: false
        };
        for (i, &font) in fonts::FONTS.iter().enumerate() {
            cpu.ram[i] = font;     
        }

        cpu
    }
    
    /// Loads a program in the form of a u8 array into the chip8 memory
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &b) in rom.iter().enumerate() {
            self.ram[i + 0x200] = rom[i];
        }
    } 

    pub fn run_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xf000) >> 12 as u8,
            (opcode & 0x0f00) >> 8 as u8,
            (opcode & 0x00f0) >> 4 as u8,
            (opcode & 0x000f) as u8
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        
        let pc_change = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0) => self.op_8xy0(x, y),
            (0x8, _, _, 1) => self.op_8xy1(x, y),
            (0x8, _, _, 2) => self.op_8xy2(x, y),
            (0x8, _, _, 3) => self.op_8xy3(x, y),
            (0x8, _, _, 4) => self.op_8xy4(x, y),
            (0x8, _, _, 5) => self.op_8xy5(x, y),
            (0x8, _, _, 6) => self.op_8xy6(x),
            (0x8, _, _, 7) => self.op_8xy7(x, y),
            (0x8, _, _, 8) => self.op_8xy8(x),
            (0x9, _, _, 0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_Annn(nnn),
            (0xB, _, _, _) => self.op_Bnnn(nnn),
            (0xC, _, _, _) => self.op_Cxkk(x, kk),
            (0xD, _, _, _) => self.op_Dxyn(x, y, n),
            (0xE, _, 9, 0xE) => self.op_Ex9E(x),
            (0xE, _, 0xA, 1) => self.op_ExA1(x),
            (0xF, _, 0, 7) => self.op_Fx07(x),
            (0xF, _, 0, 0xA) => self.op_Fx0A(x),
            (0xF, _, 1, 5) => self.op_Fx15(x),
            (0xF, _, 1, 8) => self.op_Fx18(x),
            (0xF, _, 1, 0xE) => self.op_Fx1E(x),
            (0xF, _, 2, 9) => self.op_Fx29(x),
            (0xF, _, 3, 3) => self.op_Fx33(x),
            (0xF, _, 5, 5) => self.op_Fx55(x),
            (0xF, _, 6, 5) => self.op_Fx65(x),
            _ => self.unimplemented(opcode)
        };

        self.pc += pc_change;
    }

    // Returns the opcode at the program counter
    fn get_opcode(&mut self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }
    
    // Clear display
    fn op_00E0(&mut self) -> (usize) {
        //unimplemented
        2
    }
    
    // Return from subroutine
    fn op_00EE(&mut self) -> (usize) {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
        0
    }
    
    // Jump to address nnn
    fn op_1nnn(&mut self, nnn: usize) -> (usize) {
        self.pc = nnn;
        0
    }
    
    // Jump to subroutine at nnn
    fn op_2nnn(&mut self, nnn: usize) -> (usize) {
        self.sp += 1;
        // store next instruction address on stack
        self.stack[self.sp] = self.pc + 2;
        self.pc = nnn;
        0
    }

    // Skip if Vx equals kk
    fn op_3xkk(&mut self, x: usize, kk: u8) -> (usize) {
        if self.v[x] == kk {
            return 4
        }
        2
    }

    // Skip if Vx DOESNT equal kk
    fn op_4xkk(&mut self, x: usize, kk: u8) -> (usize) {
        if self.v[x] != kk {
            return 4
        }
        2
    }

    // Skip if register x equals register y
    fn op_5xy0(&mut self, x: usize, y: usize) -> (usize) {
        if self.v[x] == self.v[y] {
            return 4
        }
        2
    }

    // Put value kk into register Vx
    fn op_6xkk(&mut self, x: usize, kk: u8) -> (usize) {
        self.v[x] = kk;
        2
    }

    // Add value kk to register Vx
    fn op_7xkk(&mut self, x: usize, kk: u8) -> (usize) {
        self.v[x] += kk;
        2
    }

    // Stores value of register Vy in register Vx
    fn op_8xy0(&mut self, x: usize, y: usize) -> (usize) {
        self.v[x] = self.v[y];
        2
    }
    
    // Performs bitwise OR on Vx and Vy and stores result in Vx
    fn op_8xy1(&mut self, x: usize, y: usize) -> (usize) {
        self.v[x] |= self.v[y];
        2
    }
    
    // Bitwise AND on Vx and Vy
    fn op_8xy2(&mut self, x: usize, y: usize) -> (usize) {
        self.v[x] &= self.v[y];
        2
    }
   
    // XOR Vx and Vy
    fn op_8xy3(&mut self, x: usize, y: usize) -> (usize) {
        self.v[x] ^= self.v[y];
        2
    }

    // Add Vx and Vy and set VF to 1 if result greater than FF
    fn op_8xy4(&mut self, x: usize, y: usize) -> (usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let sum = vx + vy;
        if sum > 0xFF {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[x] = sum as u8;
        2
    }

    // Subtract Vy from Vx and set VF to 1 if Vx > Vy
    fn op_8xy5(&mut self, x: usize, y: usize) -> (usize) {
        if self.v[x] > self.v[y] {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        2
    }

    // Shift Vx right by 1 and set VF to 1 if Vx is odd, 0 if even
    //
    // TODO: According to Wikipedia and Reddit some programs utilize Vy in this fashion:
    // Vx=Vy=Vy>>1
    // I should implement this in the form of an optional command line argument.
    fn op_8xy6(&mut self, x: usize) -> (usize) {
        self.v[0xF] = if self.v[x] & 1 == 1 {1} else {0};
        self.v[x] >>= 1;
        2
    }

    // Subtract Vx from Vy and set VF to 1 if Vy > Vx
    // Basically 8xy5 but inverse
    fn op_8xy7(&mut self, x: usize, y: usize) -> (usize) {
        if self.v[y] > self.v[x] {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[y] = self.v[y].wrapping_sub(self.v[x]);
        2
    }
    
    // Same as op_8xy6 but left shift
    fn op_8xy8(&mut self, x: usize) -> (usize) {
        self.v[0xF] = if self.v[x] & 1 == 1 {1} else {0};
        self.v[x] <<= 1;
        2
    }

    // Skip next instruction if Vx != Vy
    fn op_9xy0(&mut self, x: usize, y: usize) -> (usize) {
        if self.v[x] != self.v[y] {
            return 4
        }
        2
    }
    
    // Set I to nnn
    fn op_Annn(&mut self, nnn: usize) -> (usize) {
        self.i = nnn;
        2
    }

    // Jump to nnn + V0
    fn op_Bnnn(&mut self, nnn: usize) -> (usize) {
        self.pc = nnn + (self.v[0] as usize);
        0
    }

    // Store random byte ANDed by kk in Vx
    fn op_Cxkk(&mut self, x: usize, kk: u8) -> (usize) {
        let rn: u8 = rand::random();
        self.v[x] = rn & kk;
        2
    }

    // Display n-byte sprite starting at memory location I at (Vx, Vy)
    fn op_Dxyn(&mut self, x: usize, y: usize, n: usize) -> (usize) {
        // unimplemented
        2
    }

    // Skip next instruction if key with the value of Vx is pressed
    fn op_Ex9E (&mut self, x: usize) -> (usize) {
        // unimplemented
        2
    }
    
    // Skip next instruction if key with value of Vx is NOT pressed
    fn op_ExA1(&mut self, x: usize) -> (usize) {
        // unimplemented
        2
    }

    // Set Vx = delay timer value
    fn op_Fx07(&mut self, x: usize) -> (usize) {
        // unimplemented
        2
    }

    // Wait for key press, store value in Vx
    fn op_Fx0A(&mut self, x: usize) -> (usize) {
        // unimplemented
        2
    }

    // Set delay timer = Vx
    fn op_Fx15(&mut self, x: usize) -> (usize) {
        // unimplemented
        2
    }

    // Set sound timer = Vx
    fn op_Fx18(&mut self, x: usize) -> (usize) {
        // unimplemented
        2
    }

    // Set I = I + Vx
    fn op_Fx1E(&mut self, x: usize) -> (usize) {
        self.i = self.v[x] as usize + self.i;
        2
    }
    
    // Set I = location of sprite for digit Vx
    fn op_Fx29(&mut self, x: usize) -> (usize) {
        self.i = (self.v[x] * 5) as usize;
        2
    }
    
    // Store BCD representation of Vx in ram starting at I
    fn op_Fx33(&mut self, x: usize) -> (usize) {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
        2
    }
    
    // Store registers into ram
    fn op_Fx55(&mut self, x: usize) -> (usize) {
        // prevent out of bounds
        let end = if (x > 0xf) {0xf} else {x};

        for addr in 0..end + 1 {
            self.ram[self.i + addr] = self.v[addr];
        }
        2
    }

    // Read ram into registers
    fn op_Fx65(&mut self, x: usize) -> (usize) {
        // prevent out of bounds
        let end = if (x > 0xf) {0xf} else {x};

        for addr in 0..end + 1 {
            self.v[addr] = self.ram[self.i + addr];
        }
        2
    }

    fn unimplemented(&mut self, opcode: u16) -> (usize) {
        println!("WARNING: unimplemented opcode: 0x{:x}", opcode);
        2
    }
}   

#[cfg(test)]
mod tests;
