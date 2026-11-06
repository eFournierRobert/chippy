use std::time::{Duration, Instant};

const TIMER_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / 60);

/// CHIP-8 delay and sound timers, updated at 60 Hz.
pub struct Timers {
    delay_timer: u8,
    sound_timer: u8,
    last_timer_tick: Instant,
}

impl Default for Timers {
    fn default() -> Self {
        Self::new()
    }
}

impl Timers {
    /// Creates stopped timers and records the current time.
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0,
            last_timer_tick: Instant::now(),
        }
    }

    /// Returns the delay timer value.
    pub fn get_delay(&self) -> u8 {
        self.delay_timer
    }

    /// Sets the delay timer value.
    pub fn set_delay(&mut self, value: u8) {
        self.delay_timer = value;
    }

    /// Sets the sound timer value.
    pub fn set_sound(&mut self, value: u8) {
        self.sound_timer = value;
    }

    /// Applies elapsed timer ticks and emits a terminal bell while sound is active.
    pub fn tick(&mut self) {
        let now = Instant::now();

        while now.duration_since(self.last_timer_tick) >= TIMER_INTERVAL {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                print!("\x07");
                self.sound_timer -= 1;
            }

            self.last_timer_tick += TIMER_INTERVAL;
        }
    }
}
