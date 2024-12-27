#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized_minutes = ((total_minutes % (24 * 60)) + (24 * 60)) % (24 * 60);
        
        Clock {
            hours: (normalized_minutes / 60) % 24,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn assert_equal(&self, other: &Clock) {
        assert_eq!(self.hours, other.hours);
        assert_eq!(self.minutes, other.minutes);
    }
}
