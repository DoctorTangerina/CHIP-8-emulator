use std::time::Duration;

use crate::timer::timer::Timer;

pub struct Sound {
    sound_timer: Timer,
}

impl Sound {
    pub fn init_sound() -> Self {
        Self {
            sound_timer: Timer::init_timer(),
        }
    }

    pub fn set_timer(&mut self, value: u8) {
        self.sound_timer.set_timer(value);
    }

    pub fn get_timer(&self) -> u8 {
        self.sound_timer.get_timer()
    }

    pub fn advance(&mut self, elapsed: Duration) {
        self.sound_timer.advance(elapsed);
    }

    pub fn is_active(&self) -> bool {
        self.get_timer() > 0
    }
}
