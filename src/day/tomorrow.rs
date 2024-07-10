use chrono::{Days, NaiveDate, NaiveDateTime};

use crate::utils::utc_now;

use super::DayHelper;

pub trait TomorrowHelper {
    /// English: Is the given date tomorrow
    ///
    /// 中文：是否是明天
    fn is_tomorrow(&self) -> bool;
    /// English: Return the start of tomorrow
    ///
    /// 中文：返回明天的开始时间
    fn begin_of_tomorrow() -> NaiveDateTime;
    /// English: Return the end of tomorrow
    ///
    /// 中文：返回明天的结束时间
    fn end_of_tomorrow() -> NaiveDateTime;
}

impl TomorrowHelper for NaiveDate {
    fn is_tomorrow(&self) -> bool {
        tomorrow().is_same_day(self)
    }

    fn begin_of_tomorrow() -> NaiveDateTime {
        begin_of_tomorrow()
    }

    fn end_of_tomorrow() -> NaiveDateTime {
        end_of_tomorrow()
    }
}

impl TomorrowHelper for NaiveDateTime {
    fn is_tomorrow(&self) -> bool {
        tomorrow().is_same_day(&self.date())
    }

    fn begin_of_tomorrow() -> NaiveDateTime {
        begin_of_tomorrow()
    }

    fn end_of_tomorrow() -> NaiveDateTime {
        end_of_tomorrow()
    }
}

/// English: return the tomorrow date
///
/// 中文: 返回明天的日期
pub fn tomorrow() -> NaiveDate {
    utc_now()
        .checked_add_days(Days::new(1))
        .map(|date| date.date_naive())
        .unwrap()
}

/// English: Return the start of tomorrow
///
/// 中文：返回明天的开始时间
pub fn begin_of_tomorrow() -> NaiveDateTime {
    tomorrow().and_hms_opt(0, 0, 0).unwrap()
}

/// English: Return the end of tomorrow
///
/// 中文：返回明天的结束时间
pub fn end_of_tomorrow() -> NaiveDateTime {
    tomorrow().and_hms_opt(23, 59, 59).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_time;

    #[test]
    fn test_is_tomorrow_true() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 2).unwrap();
        assert!(date.is_tomorrow())
    }

    #[test]
    fn test_is_tomorrow_false() {
        let date = NaiveDate::from_ymd_opt(1997, 7, 1).unwrap();
        assert!(!date.is_tomorrow());
    }

    #[test]
    fn test_begin_of_tomorrow() {
        let tomorrow = NaiveDate::begin_of_tomorrow();
        let actual = get_time(2000, 1, 2, 0, 0, 0);
        assert_eq!(tomorrow, actual)
    }

    #[test]
    fn test_datetime_begin_of_tomorrow() {
        let tomorrow = NaiveDateTime::begin_of_tomorrow();
        let actual = get_time(2000, 1, 2, 0, 0, 0);
        assert_eq!(tomorrow, actual)
    }

    #[test]
    fn test_end_of_tomorrow() {
        let tomorrow = NaiveDate::end_of_tomorrow();
        let actual = get_time(2000, 1, 2, 23, 59, 59);
        assert_eq!(tomorrow, actual)
    }

    #[test]
    fn test_datetime_end_of_tomorrow() {
        let tomorrow = NaiveDateTime::end_of_tomorrow();
        let actual = get_time(2000, 1, 2, 23, 59, 59);
        assert_eq!(tomorrow, actual)
    }
}
