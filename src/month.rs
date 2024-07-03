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
    /// English: To check the current datetime is the same month with the given datetime or not.
    ///
    /// 中文：判断当前时间和指定的时间是否在同一个月
    fn is_same_month(&self, other: &Self) -> bool;
    /// English: Add the specified number of months
    ///
    /// 中文：加上指定月份
    fn add_months(&self, month: u32) -> Self;
    /// English:
    ///
    /// 中文：
    fn diff_month(&self, other: &Self) -> u32;
    /// English: Get the number of calendar months between the given dates.
    ///
    /// 中文: 获取日历上两个日期相差多少个月
    fn diff_calendar_month(&self, other: &Self) -> u32;
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

    fn diff_month(&self, other: &Self) -> u32 {
        let base = self.diff_calendar_month(other);
        if self > other {
            if other.add_months(base) > *self {
                base - 1
            } else {
                base
            }
        } else if self.add_months(base) > *other {
            return base - 1;
        } else {
            return base;
        }
    }

    fn diff_calendar_month(&self, other: &Self) -> u32 {
        if self > other {
            between_months(self, other)
        } else {
            between_months(other, self)
        }
    }
}

fn between_months(one: &NaiveDate, other: &NaiveDate) -> u32 {
    let year_diff = one.year() - other.year();
    let diff_month = one.month0() - other.month0();
    year_diff as u32 * 12 + diff_month
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

    fn diff_month(&self, other: &Self) -> u32 {
        let base = self.diff_calendar_month(other);
        if self > other {
            if other.add_months(base) > *self {
                base - 1
            } else {
                base
            }
        } else if self.add_months(base) > *other {
            return base - 1;
        } else {
            return base;
        }
    }

    fn diff_calendar_month(&self, other: &Self) -> u32 {
        let one = self.date();
        let other = other.date();
        one.diff_calendar_month(&other)
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
    fn test_date_add_month() {
        let one = get_date(2000, 1, 1);
        let result = one.map(|date| date.add_months(2));
        let actual = get_date(2000, 3, 1);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_datetime_add_month() {
        let one = get_time(2000, 1, 1, 0, 0, 0);
        let result = one.map(|date| date.add_months(2));
        let actual = get_time(2000, 3, 1, 0, 0, 0);
        assert_eq!(result, actual);
    }
}
