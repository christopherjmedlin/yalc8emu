const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

pub struct Display {
    pixels: [[bool; 64]; 32],
    changed: bool
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            changed: true
        }
    }
    
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> (bool) {
        let mut y_coord: usize;
        let mut x_coord: usize;
        let mut pixels_cleared: bool = false;
        let mut pixel_value: bool;

        
        for i in 0..sprite.len() {
            for j in 0..8 {
                y_coord = (i + y) % (DISPLAY_HEIGHT);
                x_coord = (j + x) % (DISPLAY_WIDTH);
                
                println!("{}, {}", x_coord, y_coord);
                if self.pixels[y_coord][x_coord] == true {
                    pixels_cleared = true;
                }

                pixel_value = sprite[i] & 0x80 >> j != 0;
                self.pixels[y_coord][x_coord] = pixel_value;
            }
        }

        pixels_cleared
    }
    
    pub fn clear(&mut self) {
        self.pixels = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw() {
        let mut display = Display::new();
        let sprite = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        
        assert!(!display.draw(2, 5, &sprite));
        // test collision
        assert!(display.draw(5, 5, &sprite));

        assert_eq!(display.pixels[5][5..9], [true, true, true, true]);
        assert_eq!(display.pixels[7][5..9], [true, false, false, true]);
        assert_eq!(display.pixels[9][5..9], [true, true, true, true]);
    }

    #[test]
    fn test_draw_wrap_around() {
        let mut display = Display::new();
        let sprite = [0xF0, 0x90, 0x90, 0x90, 0xF0];

        let pixels_cleared = display.draw(63, 31, &sprite);
        
        assert_eq!(display.pixels[31][63], true);
        assert_eq!(display.pixels[31][0], true);
        assert_eq!(display.pixels[3][0], true);
    }

    #[test]
    fn test_clear() {
        let mut display = Display::new();
        display.pixels[0][0] = true;
        display.clear();

        assert!(!display.pixels[0][0]);
    }
}
