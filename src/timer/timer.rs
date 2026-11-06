use std::time::Duration;

const NANOS_PER_SECOND: u128 = 1_000_000_000;
const TIMER_HZ: u128 = 60;

pub struct Timer {
    value: u8,
    elapsed_time: u128,
}

impl Timer {
    pub fn init_timer() -> Self {
        Self {
            value: 0,
            elapsed_time: 0,
        }
    }

    pub fn set_timer(&mut self, time: u8) {
        self.value = time;
    }

    pub fn get_timer(&self) -> u8 {
        self.value
    }

    pub fn advance(&mut self, elapsed: Duration) {
        self.elapsed_time += elapsed.as_nanos() * TIMER_HZ;
    
        let ticks = self.elapsed_time / NANOS_PER_SECOND;
        self.elapsed_time %= NANOS_PER_SECOND;

        self.value = self.value.saturating_sub(ticks.min(u128::from(self.value)) as u8);
    }
}