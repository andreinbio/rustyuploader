use std::time;

pub struct Present {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

pub struct Time {
    as_days: u64,
}

impl Time {
    pub fn new() -> Self {
        Time {
            as_days: 60 * 60 * 24,
        }
    }

    pub fn current(&self) -> Present {
        let system_time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();
        let today_seconds = system_time % (self.as_days);
        Present {
            hours: today_seconds / 60 / 60,
            minutes: today_seconds % (60 * 60) / 60,
            seconds: today_seconds % 60,
        }
    }
}

impl Present {
    pub fn get_hours(&self) -> u64 {
        self.hours
    }

    pub fn get_minutes(&self) -> u64 {
        self.minutes
    }

    pub fn get_seconds(&self) -> u64 {
        self.seconds
    }

    pub fn get_time(&self) -> String {
        format!("{:02}:{:02}:{:02} (UTC)", self.hours, self.minutes, self.seconds)
    }
}