use chrono::{Datelike, Days, NaiveDate, NaiveDateTime};

/// English: The helper of day
///
/// 中文: 日助手
pub trait DayHelper {
    /// English: Add the specified number of days
    ///
    /// 中文: 添加指定的天数
    fn add_days(&self, n: u64) -> Self
    where
        Self: Sized;

    /// English: Get the start of one day
    ///
    /// 中文: 获取一日的开始时间
    fn begin_of_day(&self) -> Self;

    /// English: Get the day of the year
    ///
    /// 中文: 返回一年中的天数
    fn day_of_year(&self) -> u32;

    /// English: Get the number of calendar days between two dates.
    /// This means that the times are removed from the dates and then
    /// the difference in days in calculated.
    ///
    /// 中文: 计算日历相差天数。这意味着，在去除时间部分后计算相差天数。
    fn diff_calendar_days(&self, other: &Self) -> i64;

    /// English: Get the number of full day periods between two dates.
    /// Fractional days are truncated towards zero.
    ///
    /// 中文: 计算日期之间的整天数。
    fn diff_days(&self, other: &Self) -> i64;

    /// English: Get the end of one day
    ///
    /// 中文: 获取一日的结束时间
    fn end_of_day(&self) -> Self;

    /// English: Is the same day
    ///
    /// 中文: 判断两个时间是否在同一天
    fn is_same_day(&self, other: &Self) -> bool;

    // TODO: 添加add_business_days
    // TODO: 添加diff_business_days
    // TODO: 添加sub_business_days

    /// English: Sub the specified number of days
    ///
    /// 中文: 减去指定的天数
    fn sub_days(&self, n: u64) -> Self
    where
        Self: Sized;
}

impl DayHelper for NaiveDate {
    fn add_days(&self, n: u64) -> Self {
        self.checked_add_days(Days::new(n)).unwrap()
    }

    fn begin_of_day(&self) -> Self {
        *self
    }

    fn day_of_year(&self) -> u32 {
        self.ordinal()
    }

    fn diff_calendar_days(&self, other: &Self) -> i64 {
        self.diff_days(other)
    }

    fn diff_days(&self, other: &Self) -> i64 {
        let duration = *self - *other;
        duration.num_days()
    }

    fn end_of_day(&self) -> Self {
        *self
    }

    fn is_same_day(&self, other: &Self) -> bool {
        self == other
    }

    fn sub_days(&self, n: u64) -> Self
    where
        Self: Sized,
    {
        self.checked_sub_days(Days::new(n)).unwrap()
    }
}

impl DayHelper for NaiveDateTime {
    fn add_days(&self, n: u64) -> Self {
        self.checked_add_days(Days::new(n)).unwrap()
    }

    fn begin_of_day(&self) -> Self {
        self.date().and_hms_opt(0, 0, 0).unwrap()
    }

    fn day_of_year(&self) -> u32 {
        self.ordinal()
    }

    fn diff_calendar_days(&self, other: &Self) -> i64 {
        self.date().diff_calendar_days(&other.date())
    }

    fn diff_days(&self, other: &Self) -> i64 {
        let duration = *self - *other;
        duration.num_days()
    }

    fn end_of_day(&self) -> Self {
        self.date().and_hms_opt(23, 59, 59).unwrap()
    }

    fn is_same_day(&self, other: &Self) -> bool {
        self.date().eq(&other.date())
    }

    fn sub_days(&self, n: u64) -> Self
    where
        Self: Sized,
    {
        self.checked_sub_days(Days::new(n)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_time_opt;

    fn get_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
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
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time_opt(2000, 1, 1, 23, 59, 59).unwrap();
        assert!(one.is_same_day(&other))
    }

    #[test]
    fn test_datetime_is_same_day_false() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time_opt(2000, 1, 2, 0, 0, 0).unwrap();
        assert!(!one.is_same_day(&other))
    }

    #[test]
    fn test_date_add_days() {
        let date = get_date(2000, 1, 1);
        let result = date.map(|date| date.add_days(8));
        let actual = get_date(2000, 1, 9);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_datetime_date_days() {
        let datetime = get_time_opt(2000, 1, 1, 0, 0, 0);
        let result = datetime.map(|datetime| datetime.add_days(8));
        let actual = get_time_opt(2000, 1, 9, 0, 0, 0);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_date_sub_days() {
        let date = get_date(2000, 1, 9);
        let result = date.map(|date| date.sub_days(8));
        let actual = get_date(2000, 1, 1);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_datetime_sub_days() {
        let time = get_time_opt(2000, 1, 9, 6, 0, 0);
        let result = time.map(|time| time.sub_days(8));
        let actual = get_time_opt(2000, 1, 1, 6, 0, 0);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_date_diff_days() {
        let one = get_date(2000, 1, 9).unwrap();
        let other = get_date(2000, 1, 1).unwrap();
        let result = one.diff_days(&other);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_datetime_diff_days() {
        let one = get_time_opt(2000, 1, 1, 12, 0, 0).unwrap();
        let other = get_time_opt(2000, 1, 9, 0, 0, 0).unwrap();
        let result = other.diff_days(&one);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_date_diff_calendar_days() {
        let one = get_date(2000, 1, 9);
        let other = get_date(2000, 1, 1);
        let result = one.and_then(|one| other.map(|other| one.diff_calendar_days(&other)));
        assert_eq!(result, Some(8))
    }

    #[test]
    fn test_datetime_diff_calendar_days() {
        let one = get_time_opt(2000, 1, 9, 0, 0, 0);
        let other = get_time_opt(2000, 1, 1, 12, 0, 0);
        let result = one.and_then(|one| other.map(|other| one.diff_calendar_days(&other)));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_date_day_of_year() {
        let date = get_date(2000, 3, 2);
        let result = date.map(|date| date.day_of_year());
        assert_eq!(result, Some(62))
    }

    #[test]
    fn test_datetime_day_of_year() {
        let time = get_time_opt(2000, 3, 2, 0, 0, 0);
        let result = time.map(|date| date.day_of_year());
        assert_eq!(result, Some(62));
    }
}
