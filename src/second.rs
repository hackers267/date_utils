use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Timelike;

/// English: SecondHelper
///
/// 中文：秒助手
pub trait SecondHelper {
    /// English: Add the specified number of seconds
    ///
    /// 中文: 加上指定的秒数
    fn add_seconds(&self, second: i64) -> Option<Self>
    where
        Self: Sized;

    /// English: Subtract the specified number of seconds
    ///
    /// 中文：减去指定的秒数
    fn sub_seconds(&self, second: i64) -> Option<Self>
    where
        Self: Sized;
    fn diff_seconds(&self, other: &Self) -> i64;
    fn begin_of_second(&self) -> Self;

    /// English: Is the same second with the given one
    ///
    /// 中文：是否和指定的内容是同一秒
    fn is_same_second(&self, other: &Self) -> bool;

    /// English: Return the end of a second for the given date.
    ///
    /// 中文：返回本身的秒的结束时间
    fn end_of_second(&self) -> Self;
}

impl SecondHelper for NaiveDateTime {
    fn add_seconds(&self, second: i64) -> Option<Self> {
        self.checked_add_signed(Duration::seconds(second))
    }

    fn sub_seconds(&self, second: i64) -> Option<Self> {
        self.checked_sub_signed(Duration::seconds(second))
    }

    fn diff_seconds(&self, other: &Self) -> i64 {
        self.and_utc().timestamp() - other.and_utc().timestamp()
    }

    fn begin_of_second(&self) -> Self
    where
        Self: Sized,
    {
        let date = self.date();
        let time = self.time();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();
        date.and_hms_micro_opt(hour, minute, second, 0).unwrap()
    }

    fn is_same_second(&self, other: &Self) -> bool {
        self.begin_of_second() == other.begin_of_second()
    }

    fn end_of_second(&self) -> Self {
        let date = self.date();
        let time = self.time();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();
        date.and_hms_micro_opt(hour, minute, second, 999).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::SecondHelper;
    use crate::test::get_time_opt;
    use chrono::prelude::*;

    #[test]
    fn test_second_add() {
        let date_time = get_time_opt(2000, 1, 1, 0, 0, 0);
        let result = date_time.and_then(|d| d.add_seconds(32));
        let actual = get_time_opt(2000, 1, 1, 0, 0, 32);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_second_sub() {
        let date_time = get_time_opt(2000, 1, 1, 0, 0, 0);
        let result = date_time.and_then(|date_time| date_time.sub_seconds(20));
        let actual = get_time_opt(1999, 12, 31, 23, 59, 40);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_difference_second() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0);
        let other = get_time_opt(1999, 12, 31, 23, 59, 31);
        let result = one.and_then(|time| other.map(|time1| time.diff_seconds(&time1)));
        let actual = Some(29);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_begin_of_second() {
        let time = NaiveTime::from_hms_micro_opt(0, 0, 0, 300);
        let date = NaiveDate::from_ymd_opt(2000, 1, 1);
        let date_time = date.and_then(|date| time.map(|time| date.and_time(time)));
        let result = date_time.map(|date_time| date_time.begin_of_second());
        let actual =
            NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_micro_opt(0, 0, 0, 0));
        assert_eq!(result, actual)
    }

    #[test]
    fn test_is_same_second() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 1);
        let time0 = NaiveTime::from_hms_micro_opt(0, 0, 0, 0);
        let time1 = NaiveTime::from_hms_micro_opt(0, 0, 0, 999);
        let old = date.and_then(|date| time0.map(|time| date.and_time(time)));
        let new = date.and_then(|date| time1.map(|time| date.and_time(time)));
        let result = old.and_then(|old| new.map(|new| old.is_same_second(&new)));
        assert!(result.is_some_and(|result| result));
    }

    #[test]
    fn test_end_of_second() {
        let time = NaiveTime::from_hms_micro_opt(0, 0, 0, 123);
        let date = NaiveDate::from_ymd_opt(2000, 1, 1);
        let date_time = date.and_then(|date| time.map(|time| date.and_time(time)));
        let result = date_time.map(|date_time| date_time.end_of_second());
        let actual = NaiveDate::from_ymd_opt(2000, 1, 1)
            .and_then(|date| date.and_hms_micro_opt(0, 0, 0, 999));
        assert_eq!(result, actual)
    }
}
