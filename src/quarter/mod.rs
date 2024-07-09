use std::iter::from_fn;
use chrono::{Datelike, NaiveDate};
use crate::MonthHelper;

pub trait QuarterHelper {
    /// English: Get the first day of the quarter
    ///
    /// 中文: 获取季度的第一天
    fn begin_of_quarter(&self) -> Self;
    /// English: Get the last day of the quarter
    ///
    /// 中文: 获取季度的最后一天
    fn end_of_quarter(&self) -> Self;
    /// English: Whether the two dates are in the same quarter
    ///
    /// 中文: 两个日期是否在同一个季度
    fn is_same_quarter(&self, other: &Self) -> bool;
    /// English: Get the quarter of the year
    ///
    /// 中文: 获取年份的季度
    fn quarter(&self) -> Quarter;
    /// English: Get the quarter of the year
    ///
    /// 中文: 获取年份的季度
    fn quarters(&self) -> impl Iterator<Item=Self>;
    /// English: Add the specified number of year quarters to the given date.
    ///
    /// 中文: 给定日期增加指定的年份季度数。
    fn add_quarters(&self, quarters: i32) -> Self;
    /// English: Subtract the specified number of year quarters from the given date.
    ///
    /// 中文: 给定日期减去指定的年份季度数。
    fn sub_quarters(&self, quarters: i32) -> Self;
    /// English: Get the number of calendar quarters between the given dates.
    ///
    /// 中文: 获取两个日期之间的日历季度数。
    fn diff_calendar_quarters(&self, other: &Self) -> i64;
    /// English: Get the number of quarters between the given dates.
    ///
    /// 中文: 获取两个日期之间的季度数。
    fn diff_quarters(&self, other: &Self) -> i64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl QuarterHelper for NaiveDate {
    fn begin_of_quarter(&self) -> Self {
        let quarter = self.quarter();
        let month = match quarter {
            Quarter::Q1 => 1,
            Quarter::Q2 => 4,
            Quarter::Q3 => 7,
            Quarter::Q4 => 10,
        };
        self.with_month(month).unwrap().begin_of_month()
    }

    fn end_of_quarter(&self) -> Self {
        let quarter = self.quarter();
        let month = match quarter {
            Quarter::Q1 => 3,
            Quarter::Q2 => 6,
            Quarter::Q3 => 9,
            Quarter::Q4 => 12,
        };
        self.with_month(month).unwrap().end_of_month()
    }

    fn is_same_quarter(&self, other: &Self) -> bool {
        self.quarter() == other.quarter()
    }

    fn quarter(&self) -> Quarter {
        let month = self.month();
        match month {
            1 | 2 | 3 => Quarter::Q1,
            4 | 5 | 6 => Quarter::Q2,
            7 | 8 | 9 => Quarter::Q3,
            10 | 11 | 12 => Quarter::Q4,
            _ => unreachable!(),
        }
    }

    fn quarters(&self) -> impl Iterator<Item=Self> {
        let mut start = self.begin_of_quarter();
        from_fn(move || {
            let result = start;
            start = start.add_quarters(1);
            Some(result)
        })
    }

    fn add_quarters(&self, quarters: i32) -> Self {
        self.add_months((quarters as i64 * 3))
    }

    fn sub_quarters(&self, quarters: i32) -> Self {
        self.sub_months((quarters as i64 * 3))
    }

    fn diff_calendar_quarters(&self, other: &Self) -> i64 {
        let start = self.begin_of_quarter();
        let other = other.begin_of_quarter();
        start.diff_months(&other) as i64 / 3
    }

    fn diff_quarters(&self, other: &Self) -> i64 {
        self.diff_months(other) as i64 / 3
    }
}
