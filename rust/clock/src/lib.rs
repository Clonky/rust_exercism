use core::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Clock { hours: i32, minutes: i32 }

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minute_overflow = minutes / 60;
        let hours = hours + minute_overflow;
        Self { hours: hours.rem_euclid(24), minutes: minutes.rem_euclid(60) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let actual_minutes = minutes + self.minutes;
        Self::new(self.hours, actual_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = match self.hours.to_string().len() {
            2 => self.hours.to_string(),
            1 => format!("0{}", self.hours.to_string()).to_string(),
            _ => panic!("Error in hour pattern")
        };
        let minutes = match self.minutes.to_string().len() {
            2 => self.minutes.to_string(),
            1 => format!("0{}", self.minutes.to_string()).to_string(),
            _ => panic!("Error in hour pattern")
        };
        write!(f, "{}:{}", hours, minutes)
    }
}