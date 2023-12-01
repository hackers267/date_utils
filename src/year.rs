use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

/// English: The helper of year
///
/// 中文: 年份助手
pub trait YearHelper {
    /// English: Get the begin of year.
    ///
    /// 中文: 获取一年的开始
    fn begin_of_year(&self) -> Self;
    /// English: Get the end of year.
    ///
    /// 中文: 获取一个年的结束。
    fn end_of_year(&self) -> Self;
    /// English: To check the other date or datetime is same year or not with the date.
    ///
    /// 中文: 判断另一个日期或时间是否和本日期或时间在同一年
    fn is_same_year(&self, other: &Self) -> bool;
    /// English: Get the date or datetime is a leap year.
    ///
    /// 中文: 获取日期或时间所在的年是不是闰年。
    fn is_leap_year(&self) -> bool;

    /// English: Add the n years
    ///
    /// 中文: 加上n年
    fn add_years(&self, n: i32) -> Option<Self>
    where
        Self: std::marker::Sized;
    /// English: Get the number of calendar years between the given dates;
    ///
    /// 中文: 计算日历年差
    fn diff_calendar_years(&self, other: &Self) -> i32;

    /// English: Get the number of years between the given dates;
    ///
    /// 中文: 计算日期年差
    fn diff_years(&self, other: &Self) -> i32;
}

impl YearHelper for NaiveDateTime {
    fn begin_of_year(&self) -> Self {
        let year = self.year();
        NaiveDate::from_ymd_opt(year, 1, 1)
            .and_then(|date| date.and_hms_opt(0, 0, 0))
            .unwrap()
    }

    fn end_of_year(&self) -> Self {
        let year = self.year();
        NaiveDate::from_ymd_opt(year, 12, 31)
            .and_then(|date| date.and_hms_opt(23, 59, 59))
            .unwrap()
    }

    fn is_same_year(&self, other: &Self) -> bool {
        self.date().is_same_year(&other.date())
    }

    fn is_leap_year(&self) -> bool {
        self.date().leap_year()
    }

    fn add_years(&self, n: i32) -> Option<Self> {
        self.with_year(self.year() + n)
    }

    fn diff_calendar_years(&self, other: &Self) -> i32 {
        self.year() - other.year()
    }

    fn diff_years(&self, other: &Self) -> i32 {
        let adder = match (
            self.month() >= other.month(),
            self.day() >= other.day(),
            self.hour() >= other.hour(),
            self.minute() >= other.minute(),
            self.second() >= other.second(),
        ) {
            (true, true, true, true, true) => 0,
            _ => -1,
        };
        self.date().diff_years(&other.date()) + adder
    }
}

impl YearHelper for NaiveDate {
    fn begin_of_year(&self) -> Self {
        let year = self.year();
        Self::from_ymd_opt(year, 1, 1).unwrap()
    }

    fn end_of_year(&self) -> Self {
        let year = self.year();
        Self::from_ymd_opt(year, 12, 31).unwrap()
    }

    fn is_same_year(&self, other: &Self) -> bool {
        self.year() == other.year()
    }

    fn is_leap_year(&self) -> bool {
        self.leap_year()
    }

    fn add_years(&self, n: i32) -> Option<Self> {
        let year = self.year() + n;
        self.with_year(year)
    }

    fn diff_calendar_years(&self, other: &Self) -> i32 {
        self.year() - other.year()
    }

    fn diff_years(&self, other: &Self) -> i32 {
        let diff = self.years_since(*other);
        match diff {
            Some(n) => n as i32,
            None => -(other.years_since(*self).unwrap() as i32),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime};
    use proptest::prelude::*;

    use crate::year::YearHelper;

    proptest! {}

    #[test]
    fn test_add_year() {
        let cur = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let after = cur.add_years(3).unwrap();
        let expect = NaiveDate::from_ymd_opt(2003, 1, 1).unwrap();
        assert_eq!(after, expect);
    }

    #[test]
    fn test_since_year() {
        let before = NaiveDate::from_ymd_opt(2020, 3, 4).unwrap();
        let cur = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let y = cur.years_since(before).unwrap();
        assert_eq!(y, 1);
    }

    #[test]
    fn test_since_year1() {
        let before = NaiveDate::from_ymd_opt(2020, 3, 1).unwrap();
        let cur = NaiveDate::from_ymd_opt(2022, 3, 1).unwrap();
        let y = cur.years_since(before).unwrap();
        assert_eq!(y, 2);
    }

    #[test]
    fn test_diff_calendar_years() {
        let before = NaiveDate::from_ymd_opt(2020, 3, 4).unwrap();
        let cur = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let y = cur.diff_calendar_years(&before);
        assert_eq!(y, 2);
    }

    #[test]
    fn test_diff_calendar_years_time() {
        let before = NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let cur = NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let diff = cur.diff_years(&before);
        assert_eq!(diff, 2);
    }

    #[test]
    fn test_diff_calendar_years_time1() {
        let before = NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 1)
            .unwrap();

        let cur = NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let diff = cur.diff_years(&before);
        assert_eq!(diff, 1);
    }
    #[test]
    fn test_diff_calendar_years_time2() {
        let before = NaiveDate::from_ymd_opt(2020, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let cur = NaiveDate::from_ymd_opt(2022, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let diff = cur.diff_years(&before);
        assert_eq!(diff, 2);
    }

    fn gen_time(
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
    fn test_datetime_begin_of_year() {
        let date = gen_time(2000, 6, 6, 6, 6, 6).unwrap();
        let result = date.begin_of_year();
        let actual = gen_time(2000, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn test_datetime_end_of_year() {
        let date = gen_time(2000, 6, 6, 6, 6, 6).unwrap();
        let result = date.end_of_year();
        let actual = gen_time(2000, 12, 31, 23, 59, 59).unwrap();
        assert_eq!(result, actual);
    }

    fn gen_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    #[test]
    fn test_date_end_of_year() {
        let date = gen_date(2000, 6, 6).unwrap();
        let result = date.end_of_year();
        let actual = gen_date(2000, 12, 31).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn test_datetime_is_same_year_true() {
        let one = gen_time(2000, 1, 1, 0, 0, 0).unwrap();
        let other = gen_time(2000, 12, 31, 23, 59, 59).unwrap();
        assert!(one.is_same_year(&other))
    }

    #[test]
    fn test_datetime_is_same_year_false() {
        let one = gen_time(2000, 1, 1, 0, 0, 0).unwrap();
        let other = gen_time(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(!one.is_same_year(&other))
    }
}
