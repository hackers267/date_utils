use chrono::{DateTime, TimeZone, Timelike};

pub trait DateTimeOperator {
    fn begin_of_day(&self) -> Self;
    fn begin_of_hour(&self) -> Self;
    fn begin_of_minute(&self) -> Self;
}

impl<Tz: TimeZone> DateTimeOperator for DateTime<Tz> {
    fn begin_of_day(&self) -> Self {
        self.date().and_hms(0, 0, 0)
    }

    fn begin_of_hour(&self) -> Self {
        let hour = self.hour();
        self.date().and_hms(hour, 0, 0)
    }

    fn begin_of_minute(&self) -> Self {
        self.with_second(0).unwrap()
    }
}
