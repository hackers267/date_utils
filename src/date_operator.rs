pub use chrono::{DateTime, TimeZone};
use chrono::{Datelike, NaiveDate, NaiveDateTime};

pub trait DateOperator {
    /// To check the current date or datetime is before the given date or datetime or not.
    /// 判断当前日期或时间是否在指定的日期或时间之前
    fn before(&self, other: &Self) -> bool;
    /// To check the current date or datetime is after the given date or datetime or not.
    /// 判断当前日期或时间是否在指定的日期或时间之后
    fn after(&self, other: &Self) -> bool;
    /// To check the other date or datetime is the same day with the given date or datetime or not.
    /// 判断当前日期或时间是否与指定的日期或时间在同一天
    fn is_same_day(&self, other: &Self) -> bool;
    /// To check the other date or datetime is the same month with the given date or datetime or not.
    /// 判断当前日期或时间是否与指定的日期或时间在同一个月
    fn is_same_month(&self, other: &Self) -> bool;
    /// To check the other date or datetime is same year or not with the date. 判断另一个日期或时间是否和本日期或时间在同一年
    fn is_same_year(&self, other: &Self) -> bool;
    /// Get the begin of year.
    /// 获取一年的开始
    fn begin_of_year(&self) -> Self;
    /// Get the end of year.
    /// 获取一个年的结束。
    fn end_of_year(&self) -> Self;
    /// Get the begin of month.
    /// 获取某个月的开始
    fn begin_of_month(&self) -> Self;
    ///  Get the end of month.
    /// 获取某个月的结束
    fn end_of_month(&self) -> Self;
    /// Get the date or datetime is a leap year.
    /// 获取日期或时间所在的年是不是闰年。
    fn is_leap_year(&self) -> bool;
}

impl DateOperator for NaiveDateTime {
    /// To check the current datetime is before the given  datetime or not.
    /// 判断当前时间是否在指定的时间前。
    ///
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    /// To check the current datetime is after the given datetime or not.
    /// 判断当前时间是否在指定的时间后。
    ///
    fn after(&self, other: &Self) -> bool {
        self > other
    }

    /// To check the current datetime is the same datetime with the given datetime or not.
    /// 判断当前时间和指定的时间是否在同一天
    ///
    fn is_same_day(&self, other: &Self) -> bool {
        self.date() == other.date()
    }

    /// To check the current datetime is the same month with the given datetime or not.
    /// 判断当前时间和指定的时间是否在同一个月
    ///
    fn is_same_month(&self, other: &Self) -> bool {
        self.year() == other.year() && self.month() == other.month()
    }

    /// To check whether the another datetime is the same year with the datatime.
    /// 判断另一个时间是否和当前时间在同一年
    ///
    fn is_same_year(&self, other: &Self) -> bool {
        self.year() == other.year()
    }
    /// Get the start time of a year.
    /// 获取一年的开始时间
    ///
    fn begin_of_year(&self) -> Self {
        self.date()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    /// Get the end time of the year.
    /// 获取一年的结束时间
    ///
    fn end_of_year(&self) -> Self {
        self.date()
            .with_month(12)
            .unwrap()
            .with_day(31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap()
    }

    /// Get the start time of the month.
    /// 获取某个月的开始时间
    ///
    fn begin_of_month(&self) -> Self {
        self.date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    /// Get the end time of the month.
    /// 获取某个月的结束时间
    ///
    fn end_of_month(&self) -> Self {
        let date = self.date();
        let end_date = date.end_of_month();
        end_date.and_hms_opt(23, 59, 59).unwrap()
    }

    /// To check whether the datetime is leap year or not.
    /// 判断时间所有的年是否是闰年
    ///
    fn is_leap_year(&self) -> bool {
        self.date().is_leap_year()
    }
}
impl DateOperator for NaiveDate {
    /// To check the current date  is before the given date or not.
    /// 判断当前日期是否在指定的日期前。
    ///
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    /// To check the current date is after the given date or datetime or not.
    /// 判断当前刵是是否在指定的日期后。
    ///
    fn after(&self, other: &Self) -> bool {
        self > other
    }

    /// To check the current date is the same date with the given date or not.
    /// 判断当前日期和指定的日期是否在同一天
    ///
    fn is_same_day(&self, other: &Self) -> bool {
        self == other
    }

    /// To check the current date is the same date with the given date or not.
    /// 判断当前日期和指定的日期是否在同一个月
    ///
    fn is_same_month(&self, other: &Self) -> bool {
        self.year() == other.year() && self.month() == other.month()
    }

    /// To check whether the other date is the same year with the date.
    /// 判断另一个日期是否和当前日期在同一年
    ///
    fn is_same_year(&self, other: &Self) -> bool {
        self.year() == other.year()
    }

    /// Get the start date of a year.
    /// 获取一年的开始日期
    fn begin_of_year(&self) -> Self {
        self.with_month(1).unwrap().with_day(1).unwrap()
    }

    /// Get the end of year.
    /// 获取一年的结束日期。
    fn end_of_year(&self) -> Self {
        self.with_month(12).unwrap().with_day(31).unwrap()
    }

    /// Get the begin date of a month.
    /// 获取某个月的开始日
    ///
    fn begin_of_month(&self) -> Self {
        self.with_day(1).unwrap()
    }

    /// Get the end of month.
    /// 获取某个月的结束日期
    ///
    fn end_of_month(&self) -> Self {
        let day31 = [1, 3, 5, 7, 8, 10, 12];
        let day30 = [4, 6, 9, 11];
        let month = self.month();
        if day31.binary_search(&month).is_ok() {
            return self.with_day(31).unwrap();
        }
        if day30.binary_search(&month).is_ok() {
            return self.with_day(30).unwrap();
        }
        if self.is_leap_year() {
            return self.with_day(29).unwrap();
        }
        self.with_day(28).unwrap()
    }

    /// To check the date's year is leap year or not.
    /// 判断日期所在的年份是可是闰年
    ///
    fn is_leap_year(&self) -> bool {
        let year = self.year();
        if year % 400 == 0 {
            return true;
        }
        if year % 4 == 0 {
            return true;
        }
        false
    }
}
