use chrono::{NaiveDate, NaiveDateTime};

/// English: The helper of day
///
/// 中文: 日助手
pub trait DayHelper {
    /// English: Get the begin of one day
    ///
    /// 中文: 获取一日的开始时间
    fn begin_of_day(&self) -> Self;

    /// English: Get the end of one day
    ///
    /// 中文: 获取一日的结束时间
    fn end_of_day(&self) -> Self;

    /// English: Is the same day
    ///
    /// 中文: 判断两个时间是否在同一天
    fn is_same_day(&self, other: &Self) -> bool;
}

impl DayHelper for NaiveDate {
    fn begin_of_day(&self) -> Self {
        *self
    }

    fn end_of_day(&self) -> Self {
        *self
    }

    fn is_same_day(&self, other: &Self) -> bool {
        self == other
    }
}

impl DayHelper for NaiveDateTime {
    fn begin_of_day(&self) -> Self {
        self.date().and_hms_opt(0, 0, 0).unwrap()
    }

    fn end_of_day(&self) -> Self {
        self.date().and_hms_opt(23, 59, 59).unwrap()
    }

    fn is_same_day(&self, other: &Self) -> bool {
        self.date().eq(&other.date())
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
    fn test_date_begin_of_day() {
        let date = get_date(2000, 1, 1).unwrap();
        assert_eq!(date, date.begin_of_day());
    }

    #[test]
    fn test_date_end_of_day() {
        let date = get_date(2000, 1, 1).unwrap();
        assert_eq!(date, date.end_of_day());
    }

    #[test]
    fn test_datetime_is_same_day_true() {
        let one = get_time(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time(2000, 1, 1, 23, 59, 59).unwrap();
        assert!(one.is_same_day(&other))
    }

    #[test]
    fn test_datetime_is_same_day_false() {
        let one = get_time(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time(2000, 1, 2, 0, 0, 0).unwrap();
        assert!(!one.is_same_day(&other))
    }
}
