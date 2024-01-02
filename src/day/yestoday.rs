use chrono::{Days, NaiveDate, NaiveDateTime};

use crate::utils::utc_now;

use super::DayHelper;

pub trait YestodayHelper {
    /// English: Is the given date yestoday
    ///
    /// 中文：判断是否是昨天
    fn is_yestoday(&self) -> bool;
    /// English: Return the start of yestoday
    ///
    /// 中文：返回昨天的开始时间
    fn begin_of_yestoday() -> NaiveDateTime;
    /// English: Reurn the end of yestoday
    ///
    /// 中文：返回昨天的结束时间
    fn end_of_yestoday() -> NaiveDateTime;
}

impl YestodayHelper for NaiveDate {
    fn is_yestoday(&self) -> bool {
        yestoday().is_same_day(self)
    }

    fn begin_of_yestoday() -> NaiveDateTime {
        begin_of_yestoday()
    }

    fn end_of_yestoday() -> NaiveDateTime {
        end_of_yestoday()
    }
}

impl YestodayHelper for NaiveDateTime {
    fn is_yestoday(&self) -> bool {
        yestoday().is_same_day(&self.date())
    }

    fn begin_of_yestoday() -> NaiveDateTime {
        begin_of_yestoday()
    }

    fn end_of_yestoday() -> NaiveDateTime {
        end_of_yestoday()
    }
}

pub fn yestoday() -> NaiveDate {
    utc_now()
        .checked_sub_days(Days::new(1))
        .map(|date| date.date_naive())
        .unwrap()
}
/// English: Return the start of yestoday
///
/// 中文：返回昨天的开始时间
fn begin_of_yestoday() -> NaiveDateTime {
    yestoday().and_hms_opt(0, 0, 0).unwrap()
}
/// English: Reurn the end of YestodayHelper
///
/// 中文：返回昨天的结束时间
fn end_of_yestoday() -> NaiveDateTime {
    yestoday().and_hms_opt(23, 59, 59).unwrap()
}
#[cfg(test)]
mod tess {
    use super::*;

    fn get_time(y: i32, m: u32, d: u32, h: u32, minute: u32, second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .and_then(|date| date.and_hms_opt(h, minute, second))
            .unwrap()
    }

    #[test]
    fn test_is_yestoday_true() {
        let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
        assert!(date.is_yestoday())
    }

    #[test]
    fn test_is_yestoday_false() {
        let date = NaiveDate::from_ymd_opt(1997, 7, 1).unwrap();
        assert!(!date.is_yestoday());
    }

    #[test]
    fn test_begin_of_yestoday() {
        let yestoday = NaiveDate::begin_of_yestoday();
        let actual = get_time(1999, 12, 31, 0, 0, 0);
        assert_eq!(yestoday, actual)
    }

    #[test]
    fn test_datetime_begin_of_yestoday() {
        let yestoday = NaiveDateTime::begin_of_yestoday();
        let actual = get_time(1999, 12, 31, 0, 0, 0);
        assert_eq!(yestoday, actual)
    }

    #[test]
    fn test_end_of_yestoday() {
        let yestoday = NaiveDate::end_of_yestoday();
        let actual = get_time(1999, 12, 31, 23, 59, 59);
        assert_eq!(yestoday, actual)
    }

    #[test]
    fn test_datetime_end_of_yestoday() {
        let yestoday = NaiveDateTime::end_of_yestoday();
        let actual = get_time(1999, 12, 31, 23, 59, 59);
        assert_eq!(yestoday, actual)
    }
}
