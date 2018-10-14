use std::time::{Instant, Duration};

// 60 hz
const TIMER_RATE: u64 = 1000/60;

pub struct TimerSubsystem {
    pub delay: u8,
    pub sound: u8,
    
    now: Instant,
    accumulator: u64,
}

impl TimerSubsystem {
    pub fn new() -> Self {
        let mut timers = TimerSubsystem {
            delay: 0,
            sound: 0,
            now: Instant::now(),
            accumulator: 0
        };
        return timers;
    }
    
    /// Decrements the timers accordingly
    pub fn cycle(&mut self) {
        self.accumulator += self.elapsed_millis();
        self.now = Instant::now();

        while self.accumulator >= TIMER_RATE {
            if self.delay > 0 {
                self.delay -= 1;
            }
            if self.sound > 0 {
                self.sound -= 1;
            }

            self.accumulator -= TIMER_RATE;
        }
    }
    
    fn elapsed_millis(&mut self) -> (u64) {
        let dur = self.now.elapsed();

        dur.as_secs() * 1000 +
        dur.subsec_nanos() as u64 / 1_000_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    
    #[test]
    fn test_timer_subsystem() {
        let mut timers = TimerSubsystem::new();
        timers.delay = 200;
        timers.sound = 200;
        
        // dunno if sleeping in a test is a really good idea but i don't
        // know what else i could do here.
        sleep(Duration::from_millis(100));
        timers.cycle();
        
        assert!(timers.delay < 200);
        assert!(timers.sound < 200);
    }
}
