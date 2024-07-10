use super::DayHelper;
use crate::utils::utc_now;
use chrono::{NaiveDate, NaiveDateTime};

pub trait TodayHelper<T> {
    /// English: Return the end of today
    ///
    /// 中文：返回今天的结束时间
    fn end_of_today() -> NaiveDateTime;
    /// English: Is the given date today
    ///
    /// 中文：判断是否是今天
    fn is_today(&self) -> bool;
    /// English: Return the start of today
    ///
    /// 中文：返回今天的开始时间
    fn begin_of_today() -> NaiveDateTime;
}

impl TodayHelper<&Self> for NaiveDateTime {
    fn end_of_today() -> Self {
        end_of_today()
    }

    fn is_today(&self) -> bool {
        let today = today();
        let other = self.date();
        today.is_same_day(&other)
    }

    fn begin_of_today() -> NaiveDateTime {
        begin_of_today()
    }
}

impl TodayHelper<&Self> for NaiveDate {
    fn end_of_today() -> NaiveDateTime {
        end_of_today()
    }

    fn is_today(&self) -> bool {
        today().is_same_day(self)
    }

    fn begin_of_today() -> NaiveDateTime {
        begin_of_today()
    }
}

/// English: Is the given date today
///
/// 中文：判断是否是今天
pub fn today() -> NaiveDate {
    utc_now().naive_local().date()
}

/// English: Return the start of today
///
/// 中文：返回今天的开始时间
pub fn begin_of_today() -> NaiveDateTime {
    today().and_hms_opt(0, 0, 0).unwrap()
}

/// English: Return the end of today
///
/// 中文：返回今天的结束时间
pub fn end_of_today() -> NaiveDateTime {
    today().and_hms_opt(23, 59, 59).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_time;

    #[test]
    fn test_is_today_true() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        assert!(date.is_today())
    }

    #[test]
    fn test_is_today_false() {
        let date = NaiveDate::from_ymd_opt(1997, 7, 1).unwrap();
        assert!(!date.is_today());
    }

    #[test]
    fn test_begin_of_today() {
        let today = NaiveDate::begin_of_today();
        let actual = get_time(2000, 1, 1, 0, 0, 0);
        assert_eq!(today, actual)
    }

    #[test]
    fn test_datetime_begin_of_today() {
        let today = NaiveDateTime::begin_of_today();
        let actual = get_time(2000, 1, 1, 0, 0, 0);
        assert_eq!(today, actual)
    }

    #[test]
    fn test_end_of_today() {
        let today = NaiveDate::end_of_today();
        let actual = get_time(2000, 1, 1, 23, 59, 59);
        assert_eq!(today, actual)
    }

    #[test]
    fn test_datetime_end_of_today() {
        let today = NaiveDateTime::end_of_today();
        let actual = get_time(2000, 1, 1, 23, 59, 59);
        assert_eq!(today, actual)
    }
}
