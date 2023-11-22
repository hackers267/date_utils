pub use chrono::{DateTime, TimeZone};
use chrono::{NaiveDate, NaiveDateTime};

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
}
