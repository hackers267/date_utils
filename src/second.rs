use chrono::Duration;
use chrono::NaiveDateTime;
use std::marker;

pub trait SecondHelper {
    fn add_second(&self, second: i64) -> Option<Self>
    where
        Self: marker::Sized;

    fn sub_second(&self, second: i64) -> Option<Self>
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
}
