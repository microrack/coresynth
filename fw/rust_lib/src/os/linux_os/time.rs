use std::time::{
    Duration as StdDuration,
    Instant as StdInstant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instant(StdInstant);

impl Instant {
    pub fn now() -> Instant {
        Instant(StdInstant::now())
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
        self.0.cmp(&other.0)
    }
}

// TODO StdDuration is unsigned
pub struct Offset(StdDuration);

impl Offset {
    pub fn as_duration(self) -> Duration {
        Duration(self.0)
    }
}

impl core::ops::Sub<Instant> for Instant {
    type Output = Offset;

    fn sub(mut self, rhs: Instant) -> Self::Output {
        Offset(self.0 - rhs.0)
    }
}

#[derive(Debug)]
pub struct Duration(StdDuration);

impl Duration {
    pub const fn from_ms(ms:u32) -> Duration {
        Duration(StdDuration::from_millis(ms as u64))
    }

    pub fn as_ms(self) -> u32 {
        // TODO safer cast
        self.0.as_millis() as u32
    }
}

impl core::ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, dur: Duration) {
        self.0 += dur.0;
    }
}

impl core::ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(mut self, dur: Duration) -> Self {
        Instant(self.0 + dur.0)
    }
}

impl Into<StdDuration> for Duration {
    fn into(self) -> StdDuration {
        self.0
    }
}