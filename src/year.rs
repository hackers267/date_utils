use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

/// English: The helper of year
///
/// 中文: 年份助手
pub trait YearHelper {
    /// English: Get the start of year.
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
    fn add_years(&self, n: i32) -> Self;
    /// English: Add the n years
    ///
    /// 中文: 加上n年
    fn add_years_opt(&self, n: i32) -> Option<Self>
    where
        Self: Sized;
    /// English: Get the number of calendar years between the given dates;
    ///
    /// 中文: 计算日历年差
    fn diff_calendar_years(&self, other: &Self) -> i32;

    /// English: Get the number of years between the given dates;
    ///
    /// 中文: 计算日期年差
    fn diff_years(&self, other: &Self) -> i32;
    /// English: Return the last day of a year for the given date.
    ///
    /// 中文: 获取一年的最后一天
    fn last_day_of_year(&self) -> Self;
    /// English: Subtract the specified number of years from the given date.
    ///
    /// 中文: 减去n年
    fn sub_years(&self, n: i32) -> Self;
    /// English: Subtract the specified number of years from the given date.
    ///
    /// 中文: 减去n年
    fn sub_years_opt(&self, n: i32) -> Option<Self>
    where
        Self: Sized;
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

    fn add_years(&self, n: i32) -> Self {
        self.add_years_opt(n).unwrap()
    }

    fn add_years_opt(&self, n: i32) -> Option<Self>
    where
        Self: Sized,
    {
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

    fn last_day_of_year(&self) -> Self {
        self.end_of_year().date().and_hms_opt(23, 59, 59).unwrap()
    }

    fn sub_years(&self, n: i32) -> Self {
        self.add_years(-n)
    }

    fn sub_years_opt(&self, n: i32) -> Option<Self>
    where
        Self: Sized,
    {
        self.add_years_opt(-n)
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

    fn add_years(&self, n: i32) -> Self {
        self.add_years_opt(n).unwrap()
    }

    fn add_years_opt(&self, n: i32) -> Option<Self>
    where
        Self: Sized,
    {
        self.with_year(self.year() + n)
    }

    fn diff_calendar_years(&self, other: &Self) -> i32 {
        self.year() - other.year()
    }

    fn diff_years(&self, other: &Self) -> i32 {
        let diff = self.years_since(*other);
        match diff {
            Some(n) => n as i32,
            None => -other.diff_years(self),
        }
    }

    fn last_day_of_year(&self) -> Self {
        self.end_of_year()
    }

    fn sub_years(&self, n: i32) -> Self {
        self.add_years(-n)
    }

    fn sub_years_opt(&self, n: i32) -> Option<Self>
    where
        Self: Sized,
    {
        self.add_years_opt(-n)
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::test::get_time_opt;
    use crate::year::YearHelper;

    #[test]
    fn test_add_year() {
        let cur = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let after = cur.add_years(3);
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
    fn test_diff_years_1() {
        let before = NaiveDate::from_ymd_opt(2020, 3, 4).unwrap();
        let cur = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let y = before.diff_years(&cur);
        assert_eq!(y, -1);
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

    #[test]
    fn test_datetime_begin_of_year() {
        let date = get_time_opt(2000, 6, 6, 6, 6, 6).unwrap();
        let result = date.begin_of_year();
        let actual = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn test_datetime_sub_years() {
        let date = get_time_opt(2000, 6, 6, 6, 6, 6).unwrap();
        let result = date.sub_years(2);
        let actual = get_time_opt(1998, 6, 6, 6, 6, 6).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn test_date_sub_years() {
        let date = gen_date(2000, 6, 6).unwrap();
        let result = date.sub_years(2);
        let actual = gen_date(1998, 6, 6).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn test_datetime_last_day_of_year() {
        let date = get_time_opt(2000, 6, 6, 6, 6, 6).unwrap();
        let result = date.last_day_of_year();
        let actual = get_time_opt(2000, 12, 31, 23, 59, 59).unwrap();
        assert_eq!(result, actual);
    }
    #[test]
    fn test_date_last_day_of_year() {
        let date = gen_date(2000, 6, 6).unwrap();
        let result = date.last_day_of_year();
        let actual = gen_date(2000, 12, 31).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn test_datetime_end_of_year() {
        let date = get_time_opt(2000, 6, 6, 6, 6, 6).unwrap();
        let result = date.end_of_year();
        let actual = get_time_opt(2000, 12, 31, 23, 59, 59).unwrap();
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
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time_opt(2000, 12, 31, 23, 59, 59).unwrap();
        assert!(one.is_same_year(&other))
    }

    #[test]
    fn test_datetime_is_same_year_false() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time_opt(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(!one.is_same_year(&other))
    }

    #[test]
    fn test_datetime_add_years() {
        let one = get_time_opt(2000, 6, 6, 6, 6, 6).unwrap();
        let other = one.add_years(8);
        let actual = get_time_opt(2008, 6, 6, 6, 6, 6).unwrap();
        assert_eq!(other, actual);
    }

    #[test]
    fn test_datetime_calendar_diff_years() {
        let one = get_time_opt(2000, 6, 6, 6, 6, 6).unwrap();
        let actual = get_time_opt(2008, 6, 6, 0, 0, 0).unwrap();
        assert_eq!(actual.diff_calendar_years(&one), 8);
    }
}
