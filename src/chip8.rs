const RAM_SIZE: usize = 4096;

use fonts;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    stack: [u16; 16],
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

    // Returns the opcode at the program counter
    fn get_opcode(&mut self) -> u16 {
        return (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
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

        assert_eq!(chip8.get_opcode(), 0xFFFF)
    }
}
