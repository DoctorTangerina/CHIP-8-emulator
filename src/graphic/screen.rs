use crate::memory::memory::Memory;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const START_SCREEN_BUFFER: usize = 0xF00;

pub struct Screen {
    base: usize,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            base: START_SCREEN_BUFFER,
        }
    }
}

impl Screen {
    pub fn clear(&mut self, mem: &mut Memory) {
        mem.write(self.base, &vec![0; 256])
            .expect("Couldn't clear frame buffer");
    }

    pub fn draw(&mut self, x: u8, y: u8, memory: &mut Memory, start: u16, height: u8) -> bool {
        let mut collision: bool = false;
        // The starting coordinates wrap, but the sprite itself is clipped at the edges.
        let screen_x = (usize::from(x)) % SCREEN_WIDTH;
        let start_y = (usize::from(y)) % SCREEN_HEIGHT;

        for row in 0..usize::from(height) {
            let screen_y = start_y + row;
            // Clip rows past the bottom edge instead of wrapping them.
            if screen_y >= SCREEN_HEIGHT {
                break;
            }

            let mut sprite = vec![0];
            memory
                .read(usize::from(start) + row, 1, &mut sprite)
                .expect("Could not read sprite data from memory");

            // The sprite row may straddle two framebuffer bytes when x isn't byte-aligned.
            let shift = screen_x % 8;
            let first_byte = screen_x / 8;
            let parts = [
                (first_byte, sprite[0] >> shift),
                (first_byte + 1, ((u16::from(sprite[0]) << 8) >> shift) as u8),
            ];

            for (byte_col, bits) in parts {
                // Clip pixels past the right edge instead of wrapping them.
                if bits == 0 || byte_col >= SCREEN_WIDTH / 8 {
                    continue;
                }
                let addr = self.base + screen_y * (SCREEN_WIDTH / 8) + byte_col;

                let mut buffer = vec![0];
                memory
                    .read(addr, 1, &mut buffer)
                    .expect("Couldn't read from frame buffer");

                collision |= buffer[0] & bits != 0;
                buffer[0] ^= bits;

                memory
                    .write(addr, &buffer)
                    .expect("Couldn't write to frame buffer");
            }
        }

        collision
    }

    pub fn display_buffer_addr(&self) -> usize {
        self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_toggles_pixels_and_reports_collisions() {
        let mut screen = Screen::default();
        let mut memory = Memory::init_memory();
        let mut byxel: Vec<u8> = vec![0];
        let sprite: u8 = 0b1100_0000;
        memory.write(0x300, &[sprite]).unwrap();

        assert!(!screen.draw(0, 0, &mut memory, 0x300, 1));
        memory.read(screen.base, 1, &mut byxel).unwrap();
        assert!(
            byxel[0] & sprite == sprite,
            "byxel: {:b}, sprite {:b}",
            byxel[0],
            sprite
        );

        assert!(screen.draw(0, 0, &mut memory, 0x300, 1));
        memory.read(screen.base, 1, &mut byxel).unwrap();
        assert!(
            byxel[0] & sprite == 0,
            "byxel: {:b}, sprite {:b}",
            byxel[0],
            sprite
        );
    }

    #[test]
    fn draw_toggles_pixels_with_unaligned_byte() {
        let mut screen = Screen::default();
        let mut memory = Memory::init_memory();
        let mut byxel: Vec<u8> = vec![0; 2];
        let sprite: u8 = 0b1100_0011;
        memory.write(0x300, &[sprite]).unwrap();

        assert!(!screen.draw(3, 0, &mut memory, 0x300, 1));
        memory.read(screen.base, 2, &mut byxel).unwrap();
        assert!(
            byxel[0] & 0b0001_1000 == 0b0001_1000 && byxel[1] & 0b0110_0000 == 0b0110_0000,
            "byxel: {:08b} {:08b}, sprite {:08b}",
            byxel[0], byxel[1],
            sprite
        );
    }

    #[test]
    fn draw_clips_rows_past_bottom_edge() {
        let mut screen = Screen::default();
        let mut memory = Memory::init_memory();
        memory.write(0x300, &[0xFF, 0xFF, 0xFF]).unwrap();

        // Rows 31, 32, 33: only row 31 is on screen; 32 and 33 must not wrap to rows 0 and 1.
        assert!(!screen.draw(0, 31, &mut memory, 0x300, 3));

        let row_bytes = SCREEN_WIDTH / 8;
        let mut byxel: Vec<u8> = vec![0];
        memory.read(screen.base + 31 * row_bytes, 1, &mut byxel).unwrap();
        assert_eq!(byxel[0], 0xFF);

        let mut top: Vec<u8> = vec![0; 2 * row_bytes];
        memory.read(screen.base, 2 * row_bytes, &mut top).unwrap();
        assert!(top.iter().all(|&b| b == 0), "top rows: {:?}", top);
    }
}
