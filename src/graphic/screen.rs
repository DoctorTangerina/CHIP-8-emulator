use crate::memory::memory::Memory;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    pixels: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
}

impl Screen {
    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn draw(&mut self, x: u8, y: u8, memory: &Memory, start: u16, height: u8) -> bool {
        let mut collision = false;

        for row in 0..usize::from(height) {
            let mut sprite = vec![0];
            memory
                .read(usize::from(start) + row, 1, &mut sprite)
                .expect("Could not read sprite data from memory");

            for column in 0..8 {
                if sprite[0] & (0x80 >> column) == 0 {
                    continue;
                }

                let screen_x = (usize::from(x) + column) % SCREEN_WIDTH;
                let screen_y = (usize::from(y) + row) % SCREEN_HEIGHT;
                let pixel = &mut self.pixels[screen_y * SCREEN_WIDTH + screen_x];
                collision |= *pixel;
                *pixel ^= true;
            }
        }

        collision
    }

    pub fn pixels(&self) -> &[bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_toggles_pixels_and_reports_collisions() {
        let mut screen = Screen::default();
        let mut memory = Memory::init_memory();
        memory.write(0x300, &[0b1100_0000]).unwrap();

        assert!(!screen.draw(0, 0, &memory, 0x300, 1));
        assert!(screen.pixels()[0]);
        assert!(screen.pixels()[1]);
        assert!(screen.draw(0, 0, &memory, 0x300, 1));
        assert!(!screen.pixels()[0]);
        assert!(!screen.pixels()[1]);
    }
}
