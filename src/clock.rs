// Rank - Meduim
// Source - exercism
use std::fmt;
#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: u32,
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (_mins, _) = Self::get_mins(minutes);
        Self {
            hours: Self::get_hours(hours, minutes),
            minutes: _mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let _mins = self.minutes as i32 + minutes;
        if _mins > 60 || _mins < 0 {
            let mut _innerMins = _mins;
            let mut _additionalHours = self.hours as i32;
            while _innerMins > 60 {
                _additionalHours += 1;
                _innerMins -= 60;
            }
            while _innerMins < 0 {
                _innerMins += 60;
                _additionalHours -= 1;
            }
            if _additionalHours < 0 {
                while _additionalHours < 0 {
                    _additionalHours += 24
                }
            }
            if _additionalHours >= 24 {
                while _additionalHours >= 24 {
                    _additionalHours %= 24
                }
                return Self {
                    hours: _additionalHours as u32,
                    minutes: _innerMins as u32,
                };
            }
            return Self {
                hours: _additionalHours as u32,
                minutes: _innerMins as u32,
            };
        }
        if minutes > 0 {
            Self {
                hours: self.hours,
                minutes: self.minutes + minutes as u32,
            }
        } else {
            Self {
                hours: self.hours,
                minutes: (self.minutes as i32 + minutes) as u32,
            }
        }
    }

    pub fn get_mins(mins: i32) -> (u32, i32) {
        if mins < 0 {
            let mut _mins = mins;
            let mut _other = 0;
            while _mins < 0 {
                _other -= 1;
                _mins += 60_i32;
            }

            return (_mins as u32, _other);
        }
        let _mins = mins % 60;
        let _other = mins / 60;
        (_mins as u32, _other)
    }
    pub fn get_hours(hours: i32, mins: i32) -> u32 {
        if hours < 0 {
            let mut _hours = hours;
            let (_, other) = Self::get_mins(mins);

            while _hours < 0 {
                _hours += 24_i32
            }

            if other < 0 {
                let mut _innerHour = _hours + other;

                while _innerHour < 0 {
                    _innerHour += 24_i32;
                }

                return (_innerHour) as u32;
            }

            return _hours as u32;
        }
        let (_, other) = Self::get_mins(mins);
        let _hours = hours + other;
        let _hours = _hours % 24;
        if _hours < 0 {
            let mut _innerHour = _hours;

            while _innerHour < 0 {
                _innerHour += 24_i32;
            }

            return _innerHour as u32;
        }
        _hours as u32
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_hours = String::new();
        if self.hours < 10 {
            string_hours = format!("0{}", self.hours);
        } else {
            string_hours = self.hours.to_string();
        }

        let mut string_mins = String::new();
        if self.minutes < 10 {
            string_mins = format!("0{}", self.minutes);
        } else {
            string_mins = self.minutes.to_string();
        }
        write!(f, "{}:{}", string_hours, string_mins)
    }
}

#[test]
fn test_on_the_hour() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}
#[test]
//#[ignore]
fn test_past_the_hour() {
    assert_eq!(Clock::new(11, 9).to_string(), "11:09");
}
#[test]
//#[ignore]
fn test_midnight_is_zero_hours() {
    assert_eq!(Clock::new(24, 0).to_string(), "00:00");
}
#[test]
//#[ignore]
fn test_hour_rolls_over() {
    assert_eq!(Clock::new(25, 0).to_string(), "01:00");
}
#[test]
//#[ignore]
fn test_hour_rolls_over_continuously() {
    assert_eq!(Clock::new(100, 0).to_string(), "04:00");
}
#[test]
//#[ignore]
fn test_sixty_minutes_is_next_hour() {
    assert_eq!(Clock::new(1, 60).to_string(), "02:00");
}
#[test]
//#[ignore]
fn test_minutes_roll_over() {
    assert_eq!(Clock::new(0, 160).to_string(), "02:40");
}
#[test]
//#[ignore]
fn test_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
}
#[test]
//#[ignore]
fn test_hours_and_minutes_roll_over() {
    assert_eq!(Clock::new(25, 160).to_string(), "03:40");
}
// 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
#[test]
//#[ignore]
fn test_hours_and_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
}
#[test]
//#[ignore]
fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
    assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
}
#[test]
//#[ignore]
fn test_negative_hour() {
    assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
}
#[test]
//#[ignore]
fn test_negative_hour_roll_over() {
    assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
}
#[test]
//#[ignore]
fn test_negative_hour_roll_over_continuously() {
    assert_eq!(Clock::new(-91, 00).to_string(), "05:00");
}
#[test]
//#[ignore]
fn test_negative_minutes() {
    assert_eq!(Clock::new(1, -40).to_string(), "00:20");
}
#[test]
//#[ignore]
fn test_negative_minutes_roll_over() {
    assert_eq!(Clock::new(1, -160).to_string(), "22:20");
}
#[test]
//#[ignore]
fn test_negative_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
}
#[test]
//#[ignore]
fn test_negative_sixty_minutes_is_prev_hour() {
    assert_eq!(Clock::new(2, -60).to_string(), "01:00");
}
#[test]
//#[ignore]
fn test_negative_one_twenty_minutes_is_two_prev_hours() {
    assert_eq!(Clock::new(1, -120).to_string(), "23:00");
}
#[test]
//#[ignore]
fn test_negative_hour_and_minutes_both_roll_over() {
    assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
}
#[test]
//#[ignore]
fn test_negative_hour_and_minutes_both_roll_over_continuously() {
    assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
}
#[test]
//#[ignore]
fn test_zero_hour_and_negative_minutes() {
    assert_eq!(Clock::new(0, -22).to_string(), "23:38");
}
//
// Clock Math
//
#[test]
//#[ignore]
fn test_add_minutes() {
    let clock = Clock::new(10, 0).add_minutes(3);
    assert_eq!(clock.to_string(), "10:03");
}
#[test]
//#[ignore]
fn test_add_no_minutes() {
    let clock = Clock::new(6, 41).add_minutes(0);
    assert_eq!(clock.to_string(), "06:41");
}
#[test]
//#[ignore]
fn test_add_to_next_hour() {
    let clock = Clock::new(0, 45).add_minutes(40);
    assert_eq!(clock.to_string(), "01:25");
}
#[test]
//#[ignore]
fn test_add_more_than_one_hour() {
    let clock = Clock::new(10, 0).add_minutes(61);
    assert_eq!(clock.to_string(), "11:01");
}
#[test]
//#[ignore]
fn test_add_more_than_two_hours_with_carry() {
    let clock = Clock::new(0, 45).add_minutes(160);
    assert_eq!(clock.to_string(), "03:25");
}
#[test]
//#[ignore]
fn test_add_across_midnight() {
    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock.to_string(), "00:01");
}
#[test]
//#[ignore]
fn test_add_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!(clock.to_string(), "06:32");
}
#[test]
//#[ignore]
fn test_add_more_than_two_days() {
    let clock = Clock::new(1, 1).add_minutes(3500);
    assert_eq!(clock.to_string(), "11:21");
}
#[test]
//#[ignore]
fn test_subtract_minutes() {
    let clock = Clock::new(10, 3).add_minutes(-3);
    assert_eq!(clock.to_string(), "10:00");
}
#[test]
//#[ignore]
fn test_subtract_to_previous_hour() {
    let clock = Clock::new(10, 3).add_minutes(-30);
    assert_eq!(clock.to_string(), "09:33");
}
#[test]
//#[ignore]
fn test_subtract_more_than_an_hour() {
    let clock = Clock::new(10, 3).add_minutes(-70);
    assert_eq!(clock.to_string(), "08:53");
}
#[test]
//#[ignore]
fn test_subtract_across_midnight() {
    let clock = Clock::new(0, 3).add_minutes(-4);
    assert_eq!(clock.to_string(), "23:59");
}
#[test]
//#[ignore]
fn test_subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0).add_minutes(-160);
    assert_eq!(clock.to_string(), "21:20");
}
#[test]
//#[ignore]
fn test_subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new(6, 15).add_minutes(-160);
    assert_eq!(clock.to_string(), "03:35");
}
#[test]
//#[ignore]
fn test_subtract_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(-1500);
    assert_eq!(clock.to_string(), "04:32");
}
#[test]
//#[ignore]
fn test_subtract_more_than_two_days() {
    let clock = Clock::new(2, 20).add_minutes(-3000);
    assert_eq!(clock.to_string(), "00:20");
}
//
// Test Equality
//
#[test]
//#[ignore]
fn test_compare_clocks_for_equality() {
    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}
#[test]
//#[ignore]
fn test_compare_clocks_a_minute_apart() {
    assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
}
#[test]
//#[ignore]
fn test_compare_clocks_an_hour_apart() {
    assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_hour_overflow() {
    assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_hour_overflow_by_several_days() {
    assert_eq!(Clock::new(99, 11), Clock::new(3, 11));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hour() {
    assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hour_that_wraps() {
    assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hour_that_wraps_multiple_times() {
    assert_eq!(Clock::new(-83, 49), Clock::new(13, 49));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_minutes_overflow() {
    assert_eq!(Clock::new(0, 1441), Clock::new(0, 1));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_minutes_overflow_by_several_days() {
    assert_eq!(Clock::new(2, 4322), Clock::new(2, 2));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_minute() {
    assert_eq!(Clock::new(3, -20), Clock::new(2, 40));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_minute_that_wraps() {
    assert_eq!(Clock::new(5, -1490), Clock::new(4, 10));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
    assert_eq!(Clock::new(6, -4305), Clock::new(6, 15));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hours_and_minutes() {
    assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
}
#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hours_and_minutes_that_wrap() {
    assert_eq!(Clock::new(-54, -11_513), Clock::new(18, 7));
}
#[test]
//#[ignore]
fn test_compare_full_clock_and_zeroed_clock() {
    assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
}
