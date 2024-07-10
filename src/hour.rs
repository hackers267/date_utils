use chrono::{Duration, NaiveDateTime, Timelike};

/// English: The helper of hour
///
/// 中文: 时助手
pub trait HourHelper {
    /// Add the specified number of hours
    ///
    /// 中文：加上指定的小时数
    fn add_hours(&self, hour: u32) -> Self;

    /// English: Get the start of one hour
    ///
    /// 中文: 获取小时的开始时间
    fn begin_of_hour(&self) -> Self;

    /// English: Get the number of hours between the given dates;
    ///
    /// 中文：获取两个时间的小时差
    fn diff_hours(&self, other: &Self) -> i64;
    /// English: Get the end of one hour
    ///
    /// 中文: 获取小时的结束时间
    fn end_of_hour(&self) -> Self;
    /// English: Is the same hour
    ///
    /// 中文: 判断两个时间是否在同一个小时
    fn is_same_hour(&self, other: &Self) -> bool;
    /// English: Subtract the specified number
    ///
    /// 中文：减去给定的小时数
    fn sub_hours(&self, hour: u32) -> Self;
}

impl HourHelper for NaiveDateTime {
    fn add_hours(&self, hour: u32) -> Self {
        self.checked_add_signed(Duration::hours(hour as i64))
            .expect("Overflowed")
    }

    fn begin_of_hour(&self) -> Self {
        let hour = self.hour();
        self.date().and_hms_opt(hour, 0, 0).unwrap()
    }

    fn diff_hours(&self, other: &Self) -> i64 {
        self.signed_duration_since(other.to_owned()).num_hours()
    }

    fn end_of_hour(&self) -> Self {
        let hour = self.hour();
        self.date().and_hms_opt(hour, 59, 59).unwrap()
    }

    fn is_same_hour(&self, other: &Self) -> bool {
        let is_same_day = self.date().eq(&other.date());
        let is_same_hour = self.hour() == other.hour();
        is_same_day && is_same_hour
    }

    fn sub_hours(&self, hour: u32) -> Self {
        self.checked_sub_signed(Duration::hours(hour as i64))
            .expect("Overflowed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_time_opt;

    #[test]
    fn test_is_same_hour_true() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time_opt(2000, 1, 1, 0, 59, 59).unwrap();
        assert!(one.is_same_hour(&other))
    }

    #[test]
    fn test_is_same_hour_false() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let other = get_time_opt(2000, 1, 1, 1, 0, 0).unwrap();
        assert!(!one.is_same_hour(&other))
    }
    #[test]
    fn test_add_hours() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0);
        let actual = get_time_opt(2000, 1, 1, 6, 0, 0);
        let result = one.map(|date| date.add_hours(6));
        assert_eq!(result, actual);
    }

    #[test]
    fn test_diff_hours() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let actual = get_time_opt(2000, 1, 1, 6, 0, 0).unwrap();
        let diff = actual.diff_hours(&one);
        assert_eq!(diff, 6);
    }
    #[test]
    fn test_diff_hours_other() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0).unwrap();
        let actual = get_time_opt(2000, 1, 1, 6, 0, 0).unwrap();
        let diff = one.diff_hours(&actual);
        assert_eq!(diff, -6);
    }

    #[test]
    fn test_sub_hours() {
        let actual = get_time_opt(2000, 1, 1, 0, 0, 0);
        let one = get_time_opt(2000, 1, 1, 6, 0, 0);
        let result = one.map(|date| date.sub_hours(6));
        assert_eq!(result, actual);
    }
}
