use std::ops::Add;

use chrono::{Duration, NaiveDateTime, Timelike};

/// English: The helper of minute
///
/// 中文: 分钟助手
pub trait MinuteHelper {
    /// English: Add the specified number of minutes
    ///
    /// 中文：加上指定的分钟数
    fn add_minutes(&self, minute: u32) -> Self;
    /// English: Get the start of one minute
    ///
    /// 中文: 获取分钟的开始时间
    fn begin_of_minute(&self) -> Self;
    /// English: Get the signed number of full minutes
    /// between the given dates.
    ///
    /// 中文：获取给定时间之间的分钟差
    fn diff_minutes(&self, other: &Self) -> i64;
    /// English: Get the end of one minute
    ///
    /// 中文: 获取分钟的结束时间
    fn end_of_minute(&self) -> Self;
    /// English: Is the same minute
    ///
    /// 中文: 判断两个时间是否在同一分钟
    fn is_same_minute(&self, other: &Self) -> bool;
    /// English: Subtract the specified number of minutes
    ///
    /// 中文：减去给定的分钟数
    fn sub_minutes(&self, minute: u32) -> Self;
}

impl MinuteHelper for NaiveDateTime {
    fn add_minutes(&self, minute: u32) -> Self {
        self.add(Duration::minutes(minute as i64))
    }

    fn begin_of_minute(&self) -> Self {
        self.with_second(0).unwrap()
    }

    fn diff_minutes(&self, other: &Self) -> i64 {
        self.signed_duration_since(other.to_owned()).num_minutes()
    }

    fn end_of_minute(&self) -> Self {
        self.with_second(59).unwrap()
    }

    fn is_same_minute(&self, other: &Self) -> bool {
        (self.timestamp() / 60) == (other.timestamp() / 60)
    }

    fn sub_minutes(&self, minute: u32) -> Self {
        self.checked_sub_signed(Duration::minutes(minute as i64))
            .expect("overflowed")
    }
}

#[cfg(test)]
mod tests {
    use crate::test::get_time_opt;

    use super::*;

    #[test]
    fn test_same_minute_true() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0);
        let other = get_time_opt(2000, 1, 1, 0, 0, 1);
        assert!(one.unwrap().is_same_minute(&other.unwrap()));
    }

    #[test]
    fn test_same_minute_false() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0);
        let other = get_time_opt(2000, 1, 1, 0, 1, 0);
        assert!(!one.unwrap().is_same_minute(&other.unwrap()));
    }

    #[test]
    fn test_add_minutes() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0);
        let actual = get_time_opt(2000, 1, 1, 0, 30, 0);
        let result = one.map(|date| date.add_minutes(30));
        assert_eq!(result, actual)
    }

    #[test]
    fn test_diff_minutes() {
        let one = get_time_opt(2000, 1, 1, 0, 0, 0);
        let other = get_time_opt(2000, 1, 1, 0, 30, 0);
        let diff = other.and_then(|first| one.map(|second| first.diff_minutes(&second)));
        assert_eq!(diff, Some(30));
    }

    #[test]
    fn test_sub_minutes() {
        let one = get_time_opt(2000, 1, 1, 12, 0, 0);
        let actual = get_time_opt(2000, 1, 1, 11, 30, 00);
        let result = one.map(|date| date.sub_minutes(30));
        assert_eq!(actual, result);
    }
}
