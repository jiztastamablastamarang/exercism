use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = (hours + minutes.div_euclid(60)).rem_euclid(24);
        let minutes = minutes.rem_euclid(60);

        return Clock { hours, minutes };
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        return Clock::new(self.hours, self.minutes + minutes);
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:02}:{:02}", self.hours, self.minutes);
    }
}