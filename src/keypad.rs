use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keypad {
    keys: [bool; 16],
    waiting_for_keypress: bool,
    last_key_pressed: usize
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; 16],
            waiting_for_keypress: false,
            last_key_pressed: 0x10
        }
    }
    
    pub fn get_key(&mut self, key: usize) -> bool {
        return self.keys[key];
    }

    pub fn handle_event(&mut self, event: &Event) {
        let event_keycode: Keycode;
        
        match event {
            Event::KeyDown {keycode, ..} => { 
                self.keys[Keypad::map_key(keycode.unwrap())] = true; 
            },
            Event::KeyUp {keycode, ..} => { 
                self.keys[Keypad::map_key(keycode.unwrap())] = false; 
            },
            _ => {}
        }
    }

    fn map_key(keycode: Keycode) -> usize {
        println!("yay");
        match keycode {
            Keycode::Num1 => 1,
            Keycode::Num2 => 2,
            Keycode::Num3 => 3,
            Keycode::Num4 => 0xC,
            Keycode::Q => 4,
            Keycode::W => 5,
            Keycode::E => 6,
            Keycode::R => 0xD,
            Keycode::A => 7,
            Keycode::S => 8,
            Keycode::D => 9,
            Keycode::F => 0xE,
            Keycode::Z => 0xA,
            Keycode::X => 0,
            Keycode::C => 0xB,
            Keycode::V => 0xF,
            _ => 0
        }
    }

    /// Return 0x10 if a key has not been pressed since first wait,
    /// otherwise it will return the number of the last key pressed
    pub fn wait_for_keypress(&mut self) -> u8 {
        if self.waiting_for_keypress {
            if self.last_key_pressed != 0x10 {
                self.waiting_for_keypress = false;
            }
        }
        else {
            self.last_key_pressed = 0x10;
            self.waiting_for_keypress = true;
        }

        self.last_key_pressed as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_key() {
        let mut keypad = Keypad::new();
        keypad.keys[2] = true;
        
        assert!(keypad.get_key(2));
        assert!(!keypad.get_key(1));
    }

    #[test]
    pub fn test_wait_for_keypress() {
        let mut keypad = Keypad::new();
        assert_eq!(keypad.wait_for_keypress(), 0x10);
        assert!(keypad.waiting_for_keypress);
        assert_eq!(keypad.wait_for_keypress(), 0x10);
        keypad.last_key_pressed = 1;
        assert_eq!(keypad.wait_for_keypress(), 1);
        assert!(!keypad.waiting_for_keypress);
    }
}
