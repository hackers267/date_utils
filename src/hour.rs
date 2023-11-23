use chrono::{NaiveDateTime, Timelike};

/// English: The helper of hour
///
/// 中文: 时助手
pub trait HourHelper {
    /// English: Get the begin of one hour
    ///
    /// 中文: 获取小时的开始时间
    fn begin_of_hour(&self) -> Self;

    /// English: Get the end of one hour
    ///
    /// 中文: 获取小时的结束时间
    fn end_of_hour(&self) -> Self;

    /// English: Is the same hour
    ///
    /// 中文: 判断两个时间是否在同一个小时
    fn is_same_hour(&self, other: &Self) -> bool;
}

impl HourHelper for NaiveDateTime {
    fn begin_of_hour(&self) -> Self {
        let hour = self.hour();
        self.date().and_hms_opt(hour, 0, 0).unwrap()
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
}
