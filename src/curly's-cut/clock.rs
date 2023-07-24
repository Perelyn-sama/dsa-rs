use std::fmt::{Display, Formatter};

pub const HOURS_PER_DAY: i64 = 24;
pub const MINUTES_PER_HOUR: i64 = 60;
pub const MINUTES_PER_DAY: i64 = 1440;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Clock(i64);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self((i64::from(hours) * MINUTES_PER_HOUR + i64::from(minutes)) % MINUTES_PER_DAY)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self((self.0 + i64::from(minutes)) % MINUTES_PER_DAY)
    }

    fn time(&self) -> (u8, u8) {
        let minute = match self.0 % MINUTES_PER_HOUR {
            pos_minutes if pos_minutes >= 0 => pos_minutes,
            neg_minutes => MINUTES_PER_HOUR + neg_minutes,
        };
        let hour = match self.0 / MINUTES_PER_HOUR % HOURS_PER_DAY {
            pos_hours if self.0 >= 0 => pos_hours,
            neg_hours => HOURS_PER_DAY + neg_hours - (minute != 0) as i64,
        };

        debug_assert!(hour < HOURS_PER_DAY);
        debug_assert!(minute < MINUTES_PER_HOUR);
        (
            u8::try_from(hour).unwrap_or_else(|hour| {
                unreachable!("Internal error: time hour ({hour}) exceeded `u8::MAX`")
            }),
            u8::try_from(minute).unwrap_or_else(|minute| {
                unreachable!("Internal error: time minute ({minute}) exceeded `u8::MAX`")
            }),
        )
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (hour, minute) = self.time();
        write!(f, "{hour:0>2}:{minute:0>2}")
    }
}

impl PartialEq for Clock {
    fn eq(&self, rhs: &Self) -> bool {
        self.time() == rhs.time()
    }
}

fn main() {
    let clock = Clock::new(0, 0).add_minutes(-60);
    println!("Clock Time (hh:mm): {clock}");
}
