use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (minutes + hours * 60).rem_euclid(1440),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.minutes / 60) % 24;
        let leftover_minutes = self.minutes % 60;

        write!(
            f,
            "{}:{}",
            zero_padding(hours),
            zero_padding(leftover_minutes)
        )
    }
}

pub fn zero_padding(value: i32) -> String {
    format!("{:0>2}", value)
}
