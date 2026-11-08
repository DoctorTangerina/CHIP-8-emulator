use winit::keyboard::KeyCode;

pub struct IO {
    keys: [bool; 16],
    read: (bool, u8),
}

impl IO {
    pub fn init_io() -> Self {
        Self {
            keys: [false; 16],
            read: (false, 0),
        }
    }

    pub fn set_key_pressed(&mut self, key: KeyCode, pressed: bool) {
        if let Some(chip8_key) = self.match_key(key) {
            if self.read.0 {
                if pressed{
                    self.read.1 = chip8_key as u8;
                } else if self.read.1 == chip8_key as u8 {
                    self.read.0 = false;
                }
            } 
            self.keys[chip8_key] = pressed;
        }
    }

    pub fn is_reading(&self) -> bool {
        self.read.0
    }

    pub fn get_key_read(&self) -> u8 {
        self.read.1
    }

    pub fn set_read(&mut self) {
        self.read.0 = true;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    fn match_key(&self, key: KeyCode) -> Option<usize> {
        Some(match key {
            KeyCode::Digit1 => 0x1,
            KeyCode::Digit2 => 0x2,
            KeyCode::Digit3 => 0x3,
            KeyCode::Digit4 => 0xC,
            KeyCode::KeyQ => 0x4,
            KeyCode::KeyW => 0x5,
            KeyCode::KeyE => 0x6,
            KeyCode::KeyR => 0xD,
            KeyCode::KeyA => 0x7,
            KeyCode::KeyS => 0x8,
            KeyCode::KeyD => 0x9,
            KeyCode::KeyF => 0xE,
            KeyCode::KeyZ => 0xA,
            KeyCode::KeyX => 0x0,
            KeyCode::KeyC => 0xB,
            KeyCode::KeyV => 0xF,
            _ => return None,
        })
    }
}
