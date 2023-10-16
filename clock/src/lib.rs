use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock {
            hours: hours,
            minutes: minutes,
        }
        .sanitize_clock();
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::new(self.hours, self.minutes + minutes);
    }

    /**
     * Sanitize the clock to ensure that the hours and minutes are within the range of 0-23 and 0-59 respectively.
     * Clock constructor uses this method to sanitize the clock.
     */
    pub fn sanitize_clock(&self) -> Self {
        let total_effective_minutes = self.minutes % 60;
        let hours_offset = if total_effective_minutes >= 0 { 0 } else { -1 };
        let total_effective_hours = (self.hours + self.minutes / 60) % 24 + hours_offset;

        return Clock {
            hours: (24 + total_effective_hours) % 24, // 24 + total_effective_hours to handle negative hours
            minutes: (60 + total_effective_minutes) % 60, // 60 + total_effective_minutes to handle negative minutes
        };
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>#2}:{:0>#2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        return self.hours == other.hours && self.minutes == other.minutes;
    }
}
