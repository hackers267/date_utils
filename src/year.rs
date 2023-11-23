use chrono::{Datelike, NaiveDate, NaiveDateTime};

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
}
