use std::time::Duration;

const NANOS_PER_SECOND: u128 = 1_000_000_000;
const TIMER_HZ: u128 = 60;

pub struct Timer {
    value: u8,
    elapsed_time: u128,
    vblank: bool,
}

impl Timer {
    pub fn init_timer() -> Self {
        Self {
            value: 0,
            elapsed_time: 0,
            vblank: false,
        }
    }

    pub fn set_timer(&mut self, time: u8) {
        self.value = time;
    }

    pub fn get_timer(&self) -> u8 {
        self.value
    }

    pub fn clear_vblank(&mut self) {
        self.vblank = false;
    }

    pub fn get_vblank(&mut self) -> bool {
        self.vblank
    }

    pub fn advance(&mut self, elapsed: Duration) {
        self.elapsed_time += elapsed.as_nanos() * TIMER_HZ;

        let ticks = self.elapsed_time / NANOS_PER_SECOND;
        if ticks > 0 {
            self.vblank = true;
        }

        
        self.elapsed_time %= NANOS_PER_SECOND;

        self.value = self
            .value
            .saturating_sub(ticks.min(u128::from(self.value)) as u8);
    }
}
