const RAM_SIZE: usize = 4096;

use fonts;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    stack: [usize; 16],
    v: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
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
            sp: 0
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
        
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            _ => println!("Unimplemented opcode: {}", opcode)
        }
    }

    // Returns the opcode at the program counter
    fn get_opcode(&mut self) -> u16 {
        return (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }
    
    // Clear display
    fn op_00E0(&mut self) {
        //unimplemented
    }
    
    // Return from subroutine
    fn op_00EE(&mut self) {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
    }
    
    // Jump to address nnn
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }
    
    // Jump to subroutine at nnn
    fn op_2nnn(&mut self, nnn: usize) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = nnn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // helper function
    fn make_chip8_and_load_rom() -> Chip8 {
        let mut chip8 = Chip8::new();

        // just some arbitrary values for testing
        let rom: [u8; 7] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x70, 0x10
        ];
        chip8.load_rom(&rom);

        chip8
    }
    
    #[test]
    fn test_new_chip8() {
        let chip8 = Chip8::new();
        
        assert_eq!(chip8.i, 0);
        assert_eq!(chip8.stack.len(), 16);
        assert_eq!(chip8.v[0], 0);

        // test that some of the fonts are in the proper location
        assert_eq!(chip8.ram[0], 0xF0);
        assert_eq!(chip8.ram[9], 0x70);
        assert_eq!(chip8.ram[79], 0x80);
    }

    #[test]
    fn test_load_rom() {
        let chip8 = make_chip8_and_load_rom();
        
        assert_eq!(chip8.ram[0x200], 0xFF);
        assert_eq!(chip8.ram[0x206], 0x10);
        assert_eq!(chip8.ram[0x207], 0);
    }

    #[test]
    fn test_get_opcode() {
        let mut chip8 = make_chip8_and_load_rom();

        assert_eq!(chip8.get_opcode(), 0xFFFF);
    }  

    #[test]
    fn test_2nnn_and_00EE() {
        let mut chip8 = Chip8::new();

        // jump to subroutine at 0x205
        chip8.run_opcode(0x2205);
        assert_eq!(chip8.pc, 0x205);
        assert_eq!(chip8.stack[1], 0x200);

        chip8.run_opcode(0x00EE);
        assert_eq!(chip8.pc, 0x200);
        assert_eq!(chip8.sp, 0);
    }
}
