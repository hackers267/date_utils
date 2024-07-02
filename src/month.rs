use chrono::{Datelike, Months, NaiveDate, NaiveDateTime};

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
    /// English: Add the specified number of months to the given date
    ///
    /// 中文: 为指定的日期添加指定的月份
    fn add_months(&self, month: u32) -> Self;
    /// English: Get the number of calendar months between the given dates
    ///
    /// 中文: 计算两个日期对象在日历月份上的差异。
    fn diff_in_calendar_months(&self, other: &Self) -> i32;
    /// English: Get the number of full months between the given dates using trunc as a default rounding method.
    ///
    /// 中文: 计算两个日期对象之间有多少个整月
    fn diff_in_months(&self, other: &Self) -> i32;
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

    fn add_months(&self, month: u32) -> Self {
        self.checked_add_months(Months::new(month)).unwrap()
    }

    fn diff_in_calendar_months(&self, other: &Self) -> i32 {
        let month = self.month() as i32;
        let year = self.year();
        let other_month = other.month() as i32;
        let other_year = other.year();
        (year - other_year) * 12 + (month - other_month)
    }

    fn diff_in_months(&self, other: &Self) -> i32 {
        let calender_month = self.diff_in_calendar_months(other);
        if calender_month > 0 {
            let later = other.add_months(calender_month as u32);
            match self.cmp(&later) {
                std::cmp::Ordering::Less => calender_month - 1,
                std::cmp::Ordering::Equal => calender_month,
                std::cmp::Ordering::Greater => calender_month,
            }
        } else {
            let later = self.add_months(calender_month.unsigned_abs());
            match other.cmp(&later) {
                std::cmp::Ordering::Less => calender_month + 1,
                std::cmp::Ordering::Equal => calender_month,
                std::cmp::Ordering::Greater => calender_month,
            }
        }
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

    fn add_months(&self, month: u32) -> Self {
        self.checked_add_months(Months::new(month)).unwrap()
    }

    fn diff_in_calendar_months(&self, other: &Self) -> i32 {
        self.date().diff_in_calendar_months(&other.date())
    }

    fn diff_in_months(&self, other: &Self) -> i32 {
        todo!()
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
    #[test]
    fn test_date_add_months() {
        let one = get_date(2023, 4, 8).unwrap();
        let result = one.add_months(8);
        let actual = get_date(2023, 12, 8).unwrap();
        assert_eq!(result, actual);
    }
}
