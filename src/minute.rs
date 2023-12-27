use std::ops::Add;

use chrono::{Duration, NaiveDateTime, Timelike};

/// English: The helper of minite
///
/// 中文: 分钟助手
pub trait MinuteHelper {
    /// English: Get the begin of one minute
    ///
    /// 中文: 获取分钟的开始时间
    fn begin_of_minute(&self) -> Self;
    /// English: Get the end of one minute
    ///
    /// 中文: 获取分钟的结束时间
    fn end_of_minute(&self) -> Self;
    /// English: Is the same minute
    ///
    /// 中文: 判断两个时间是否在同一分钟
    fn is_same_minute(&self, other: &Self) -> bool;
    /// English: Add the specified number of minutes
    ///
    /// 中文：加上指定的分钟数
    fn add_minutes(&self, minute: u32) -> Self;
}

impl MinuteHelper for NaiveDateTime {
    fn begin_of_minute(&self) -> Self {
        self.with_second(0).unwrap()
    }

    fn end_of_minute(&self) -> Self {
        self.with_second(59).unwrap()
    }

    fn is_same_minute(&self, other: &Self) -> bool {
        (self.timestamp() / 60) == (other.timestamp() / 60)
    }

    fn add_minutes(&self, minute: u32) -> Self {
        self.add(Duration::minutes(minute as i64))
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_same_minute_true() {
        let one = NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_opt(0, 0, 0));
        let other = NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_opt(0, 0, 1));
        assert!(one.unwrap().is_same_minute(&other.unwrap()));
    }
    #[test]
    fn test_same_minute_false() {
        let one = NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_opt(0, 0, 0));
        let other = NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_opt(0, 1, 0));
        assert!(!one.unwrap().is_same_minute(&other.unwrap()));
    }

    #[test]
    fn test_add_minutes() {
        let one = NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_opt(0, 0, 0));
        let result = one.map(|date| date.add_minutes(30));
        let actual =
            NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_opt(0, 30, 0));
        assert_eq!(result, actual)
    }
}
