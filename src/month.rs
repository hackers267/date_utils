use chrono::{Datelike, NaiveDate, NaiveDateTime};

use crate::utils::{month_type, MonthType};

/// English: The helper of month
///
/// 中文: 月份助手
pub trait MonthHelper {
    /// English: Get the start time of the month.
    ///
    /// 中文: 获取某个月的开始时间
    fn begin_of_month(&self) -> Self;

    /// English: Get the end time of the month.
    ///
    /// 中文: 获取某个月的结束时间
    fn end_of_month(&self) -> Self;
    /// To check the current datetime is the same month with the given datetime or not.
    ///
    /// 判断当前时间和指定的时间是否在同一个月
    fn is_same_month(&self, other: &Self) -> bool;
}

impl MonthHelper for NaiveDate {
    fn begin_of_month(&self) -> Self {
        self.with_day(1).unwrap()
    }

    fn end_of_month(&self) -> Self {
        let month_type = month_type(self.month(), self.year());
        let last_day = match month_type {
            MonthType::Day30 => 30,
            MonthType::Day31 => 31,
            MonthType::Other(false) => 28,
            MonthType::Other(true) => 29,
        };
        self.with_day(last_day).unwrap()
    }

    fn is_same_month(&self, other: &Self) -> bool {
        self.year() == other.year() && self.month0() == other.month0()
    }
}

impl MonthHelper for NaiveDateTime {
    fn begin_of_month(&self) -> Self {
        self.date().begin_of_month().and_hms_opt(0, 0, 0).unwrap()
    }

    fn end_of_month(&self) -> Self {
        self.date().end_of_month().and_hms_opt(23, 59, 59).unwrap()
    }

    fn is_same_month(&self, other: &Self) -> bool {
        self.date().is_same_month(&other.date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    fn get_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Option<NaiveDateTime> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
    }

    #[test]
    fn test_date_begin_of_month() {
        let date = get_date(2000, 6, 6).unwrap();
        let begin = get_date(2000, 6, 1).unwrap();
        assert_eq!(date.begin_of_month(), begin)
    }

    #[test]
    fn test_datetime_begin_of_month() {
        let date = get_time(2000, 6, 6, 6, 6, 6).unwrap();
        let begin = get_time(2000, 6, 1, 0, 0, 0).unwrap();
        assert_eq!(date.begin_of_month(), begin);
    }

    #[test]
    fn test_datetime_end_of_month() {
        let date = get_time(2000, 6, 6, 6, 6, 6).unwrap();
        let end = get_time(2000, 6, 30, 23, 59, 59).unwrap();
        assert_eq!(date.end_of_month(), end);
    }

    #[test]
    fn test_datetime_is_same_month() {
        let one = get_time(2000, 6, 6, 6, 6, 6).unwrap();
        let other = get_time(2000, 6, 1, 0, 0, 0).unwrap();
        assert!(one.is_same_month(&other));
    }
}
