use chrono::{DateTime, TimeZone};

pub trait DateTimeOperator {
    fn begin_of_day(&self) -> Self;
}

impl<Tz: TimeZone> DateTimeOperator for DateTime<Tz> {
    fn begin_of_day(&self) -> Self {
        self.date().and_hms(0, 0, 0)
    }
}
