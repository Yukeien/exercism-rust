use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

fn handle_hours(mut hours: i32) -> i32 {
    if hours < 0 {
        hours = 24 - (-hours % 24);
    }

    if hours >= 24 {
        hours %= 24;
    }

    hours
}

fn handle_minutes(mut hours: i32, mut minutes: i32) -> (i32, i32) {
    if minutes >= 60 {
        hours += 1;
        minutes -= 60;
    } else if minutes < 0 {
        hours -= 1;
        minutes += 60;
    }

    (hours, minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours: 0, minutes: 0 }.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours + (minutes / 60) as i32;
        let mut spare_minutes = self.minutes + (minutes % 60);

        let handler = handle_minutes(hours, spare_minutes);

        hours = handler.0;
        spare_minutes = handler.1;

        hours = handle_hours(hours);

        Clock { hours, minutes: spare_minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
