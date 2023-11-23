use chrono::{NaiveDate, NaiveDateTime};

pub trait CommonHelper {
    /// English: To check the current date or datetime is before the given date or datetime or not.
    ///
    /// 中文: 判断当前日期或时间是否在指定的日期或时间之前
    fn before(&self, other: &Self) -> bool;

    /// English: To check the current date or datetime is after the given date or datetime or not.
    ///
    /// 中文: 判断当前日期或时间是否在指定的日期或时间之后
    fn after(&self, other: &Self) -> bool;

    /// English: To check the current date is same  with the given date or datetime or not.
    ///
    /// 中文: 判断当前日期或时间是否和给定的日期或时间相等
    fn is_same(&self, other: &Self) -> bool;
}

impl CommonHelper for NaiveDate {
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    fn after(&self, other: &Self) -> bool {
        self > other
    }

    fn is_same(&self, other: &Self) -> bool {
        self == other
    }
}

impl CommonHelper for NaiveDateTime {
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    fn after(&self, other: &Self) -> bool {
        self > other
    }

    fn is_same(&self, other: &Self) -> bool {
        self == other
    }
}
