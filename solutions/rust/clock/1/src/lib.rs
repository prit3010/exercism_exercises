use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,   // always 0..=23
    minutes: i32, // always 0..=59
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_total_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::from_total_minutes(self.hours * 60 + self.minutes + minutes)
    }

    fn from_total_minutes(total: i32) -> Self {
        // normalize into [0, 1439]
        let mut m = total % 1440;
        if m < 0 { m += 1440; }

        Clock {
            hours: m / 60,
            minutes: m % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}