use chrono::NaiveDateTime;

use crate::utils::utc_now;

/// English: The helper of daytime
///
/// 中文: 日时间助手
pub trait DayTimeHelper {
    /// English: Get the end of today.
    ///
    /// 中文: 获取今日的结束时间
    fn end_of_today() -> Self;

    /// English: Get the start of today.
    ///
    /// 中文: 获取今日的开始时间
    fn start_of_today() -> Self;
}
impl DayTimeHelper for NaiveDateTime {
    fn start_of_today() -> Self {
        utc_now().date_naive().and_hms_opt(0, 0, 0).unwrap()
    }

    fn end_of_today() -> Self {
        utc_now().date_naive().and_hms_opt(23, 59, 59).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};

    use super::*;

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
    fn test_datetime_start_of_today() {
        let result = NaiveDateTime::start_of_today();
        let actual = get_time(2000, 1, 1, 0, 0, 0);
        assert_eq!(Some(result), actual);
    }

    #[test]
    fn test_end_of_today() {
        let result = NaiveDateTime::end_of_today();
        let actual = get_time(2000, 1, 1, 23, 59, 59);
        assert_eq!(Some(result), actual);
    }
}
