use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        let add_hours = new_minutes / 60;

        let new_hours = self.hours + add_hours;
        Clock::new(new_hours, new_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
