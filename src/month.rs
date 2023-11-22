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
