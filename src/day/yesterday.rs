use chrono::{Days, NaiveDate, NaiveDateTime};

use crate::utils::utc_now;

use super::DayHelper;

pub trait YesterdayHelper {
    /// English: Is the given date yesterday
    ///
    /// 中文：判断是否是昨天
    fn is_yesterday(&self) -> bool;
    /// English: Return the start of yesterday
    ///
    /// 中文：返回昨天的开始时间
    fn begin_of_yesterday() -> NaiveDateTime;
    /// English: Return the end of yesterday
    ///
    /// 中文：返回昨天的结束时间
    fn end_of_yesterday() -> NaiveDateTime;
}

impl YesterdayHelper for NaiveDate {
    fn is_yesterday(&self) -> bool {
        yesterday().is_same_day(self)
    }

    fn begin_of_yesterday() -> NaiveDateTime {
        begin_of_yesterday()
    }

    fn end_of_yesterday() -> NaiveDateTime {
        end_of_yesterday()
    }
}

impl YesterdayHelper for NaiveDateTime {
    fn is_yesterday(&self) -> bool {
        yesterday().is_same_day(&self.date())
    }

    fn begin_of_yesterday() -> NaiveDateTime {
        begin_of_yesterday()
    }

    fn end_of_yesterday() -> NaiveDateTime {
        end_of_yesterday()
    }
}

pub fn yesterday() -> NaiveDate {
    utc_now()
        .checked_sub_days(Days::new(1))
        .map(|date| date.date_naive())
        .unwrap()
}
/// English: Return the start of yesterday
///
/// 中文：返回昨天的开始时间
fn begin_of_yesterday() -> NaiveDateTime {
    yesterday().and_hms_opt(0, 0, 0).unwrap()
}
/// English: Return the end of YesterdayHelper
///
/// 中文：返回昨天的结束时间
fn end_of_yesterday() -> NaiveDateTime {
    yesterday().and_hms_opt(23, 59, 59).unwrap()
}
#[cfg(test)]
mod tess {
    use super::*;
    use crate::test::get_time;

    #[test]
    fn test_is_yesterday_true() {
        let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
        assert!(date.is_yesterday())
    }

    #[test]
    fn test_is_yesterday_false() {
        let date = NaiveDate::from_ymd_opt(1997, 7, 1).unwrap();
        assert!(!date.is_yesterday());
    }

    #[test]
    fn test_begin_of_yesterday() {
        let yesterday = NaiveDate::begin_of_yesterday();
        let actual = get_time(1999, 12, 31, 0, 0, 0);
        assert_eq!(yesterday, actual)
    }

    #[test]
    fn test_datetime_begin_of_yesterday() {
        let yesterday = NaiveDateTime::begin_of_yesterday();
        let actual = get_time(1999, 12, 31, 0, 0, 0);
        assert_eq!(yesterday, actual)
    }

    #[test]
    fn test_end_of_yesterday() {
        let yesterday = NaiveDate::end_of_yesterday();
        let actual = get_time(1999, 12, 31, 23, 59, 59);
        assert_eq!(yesterday, actual)
    }

    #[test]
    fn test_datetime_end_of_yesterday() {
        let yesterday = NaiveDateTime::end_of_yesterday();
        let actual = get_time(1999, 12, 31, 23, 59, 59);
        assert_eq!(yesterday, actual)
    }
}
