use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {

        // Add 1 hour for every full 60min
        hours += minutes / 60;

        // Remove excess minutes
        minutes = minutes % 60;

        // Roll 1 hour back if minutes are negative
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        // Make hours always under 24
        hours %= 24;

        // ...and finally turn negative hours into positive
        if hours < 0 {
            hours += 24;
        }

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", zero_padding(self.hours), zero_padding(self.minutes))
    }
}


pub fn zero_padding(value: i32) -> String {
    format!("{:0>2}", value)
}