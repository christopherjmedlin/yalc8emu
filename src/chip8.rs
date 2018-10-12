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
    pub fn new() -> Self {
        let mut cpu = Chip8 {
            ram: [0; RAM_SIZE],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0
        };
        for (i, &font) in fonts::FONTS.iter().enumerate() {
            cpu.ram[i] = font;     
        }

        cpu
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &b) in rom.iter().enumerate() {
            self.ram[i + 0x200] = rom[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
        let mut chip8 = Chip8::new();
        // just some arbitrary values for testing
        let rom: [u8; 7] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x70, 0x10 
        ];

        chip8.load_rom(&rom);
        
        assert_eq!(chip8.ram[0x200], 0xFF);
        assert_eq!(chip8.ram[0x206], 0x10);
        assert_eq!(chip8.ram[0x207], 0);
    }
}
