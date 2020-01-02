use super::bindings::{osKernelSysTick};
use crate::glue::{TICKS_FREQ};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instant(u32);

impl Instant {
    pub fn now() -> Instant {
        // xTaskGetTickCount will enter critical section and disable interrupts
        Instant(unsafe{ osKernelSysTick() })
    }

    pub fn as_ms(self) -> u32 {
        self.0
    }
}

impl core::cmp::PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// It will work for instants close to each other (up to 1<<31)
// 1 > 0, 2 > 1, ... 0xFFFFFFFF >  0xFFFFFFFE, 0 > 0xFFFFFFFF
// At 1kHz tick rate and 32bit counter it takes 4294967sec = 49.7days to overflow
impl core::cmp::Ord for Instant {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.0.wrapping_sub(other.0) as i32).cmp(&0)
    }
}

pub struct Offset(i32);

impl Offset {
    fn from_ticks(ticks: i32) -> Offset {
        Offset(ticks)
    }

    pub fn as_duration(self) -> Duration {
        debug_assert!(self.0 >= 0);
        Duration::from_ticks(self.0 as u32)
    }
}

impl core::ops::Sub<Instant> for Instant {
    type Output = Offset;

    fn sub(self, rhs: Instant) -> Self::Output {
        // TODO proper convertation to i32
        Offset::from_ticks((self.0 - rhs.0) as i32)
    }
}

#[derive(Debug)]
pub struct Duration(u32);

impl Duration {
    fn from_ticks(ticks: u32) -> Duration {
        // TODO careful to not overflow
        // TODO if TICKS_FREQ more than 1000 we get fail
        const RATIO: u32 = 1000 / TICKS_FREQ;
        Duration(ticks * RATIO)
    }

    pub const fn from_ms(ms:u32) -> Duration {
        Duration(ms)
    }

    fn as_ticks(self) -> u32 {
        // TODO careful to not overflow
        const RATIO: u32 = TICKS_FREQ / 1000;
        self.0 * RATIO
    }

    pub fn as_ms(self) -> u32 {
        self.0
    }
}

impl core::ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, dur: Duration) {
        self.0 = self.0.wrapping_add(dur.as_ticks());
    }
}

impl core::ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(mut self, dur: Duration) -> Self {
        self += dur;
        self
    }
}