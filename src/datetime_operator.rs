use chrono::{NaiveDateTime, Timelike};

pub trait DateTimeOperator {
    fn begin_of_day(&self) -> Self;
    fn begin_of_hour(&self) -> Self;
    fn begin_of_minute(&self) -> Self;
    fn end_of_day(&self) -> Self;
    fn end_of_hour(&self) -> Self;
    fn end_of_minute(&self) -> Self;
}

impl DateTimeOperator for NaiveDateTime {
    fn begin_of_day(&self) -> Self {
        self.date().and_hms_opt(0, 0, 0).unwrap()
    }

    fn begin_of_hour(&self) -> Self {
        let hour = self.hour();
        self.date().and_hms_opt(hour, 0, 0).unwrap()
    }

    fn begin_of_minute(&self) -> Self {
        self.with_second(0).unwrap()
    }

    fn end_of_day(&self) -> Self {
        self.date().and_hms_opt(23, 59, 59).unwrap()
    }

    fn end_of_hour(&self) -> Self {
        self.with_minute(59).unwrap().with_second(59).unwrap()
    }

    fn end_of_minute(&self) -> Self {
        self.with_second(59).unwrap()
    }
}
