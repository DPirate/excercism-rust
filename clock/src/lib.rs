use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours:i32,
    minutes:i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock { 
            hours:hours, 
            minutes:minutes, }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock { 
            hours:(self.hours + (self.minutes + minutes) / 60) % 24, 
            minutes:(self.minutes + minutes) % 60, }
    }

    pub fn sanitize_clock(&self) -> Self {
        let totalEffectiveMinutes = self.minutes % 60;
        let hoursOffset = if totalEffectiveMinutes>=0 {0} else {-1};
        let totalEffectiveHours = (self.hours + self.minutes/60) % 24 + hoursOffset;

        return Clock {
            hours: if totalEffectiveHours >=0 {totalEffectiveHours} else {24 + totalEffectiveHours},
            minutes: if totalEffectiveMinutes >=0 {totalEffectiveMinutes} else {60 + totalEffectiveMinutes}
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
        println!("{} - {} \n {} - {}", self.sanitize_clock().hours, other.sanitize_clock().hours, self.sanitize_clock().minutes, other.sanitize_clock().minutes);
        return self.sanitize_clock().hours == other.sanitize_clock().hours &&
        self.sanitize_clock().minutes == other.sanitize_clock().minutes;
    }
}



