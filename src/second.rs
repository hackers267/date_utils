use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::marker;

pub trait SecondHelper {
    fn add_second(&self, second: i64) -> Option<Self>
    where
        Self: marker::Sized;

    fn sub_second(&self, second: i64) -> Option<Self>
    where
        Self: marker::Sized;
    fn difference_is_second(&self, other: &Self) -> i64;
    fn begin_of_second(&self) -> Self
    where
        Self: marker::Sized;
}

impl SecondHelper for NaiveDateTime {
    fn add_second(&self, second: i64) -> Option<Self> {
        self.checked_add_signed(Duration::seconds(second))
    }

    fn sub_second(&self, second: i64) -> Option<Self> {
        self.checked_sub_signed(Duration::seconds(second))
    }

    fn difference_is_second(&self, other: &Self) -> i64 {
        self.timestamp() - other.timestamp()
    }

    fn begin_of_second(&self) -> Self
    where
        Self: marker::Sized,
    {
        let date = self.date();
        let time = self.time();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();
        date.and_hms_milli_opt(hour, minute, second, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::SecondHelper;

    fn get_time(y: i32, m: u32, d: u32, h: u32, minute: u32, s: u32) -> Option<NaiveDateTime> {
        NaiveDate::from_ymd_opt(y, m, d).and_then(|date| date.and_hms_opt(h, minute, s))
    }

    #[test]
    fn test_second_add() {
        let date_time = get_time(2000, 1, 1, 0, 0, 0);
        let result = date_time.and_then(|d| d.add_second(32));
        let actual = get_time(2000, 1, 1, 0, 0, 32);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_second_sub() {
        let date_time = get_time(2000, 1, 1, 0, 0, 0);
        let result = date_time.and_then(|date_time| date_time.sub_second(20));
        let actual = get_time(1999, 12, 31, 23, 59, 40);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_difference_second() {
        let one = get_time(2000, 1, 1, 0, 0, 0);
        let other = get_time(1999, 12, 31, 23, 59, 31);
        let result = one.and_then(|time| other.map(|time1| time.difference_is_second(&time1)));
        let actual = Some(29);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_begin_of_second() {
        let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 300);
        let date = NaiveDate::from_ymd_opt(2000, 1, 1);
        let date_time = date.and_then(|date| time.map(|time| date.and_time(time)));
        let result = date_time.map(|date_time| date_time.begin_of_second());
        let actual =
            NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_milli_opt(0, 0, 0, 0));
        assert_eq!(result, actual)
    }
}
