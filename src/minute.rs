use chrono::{NaiveDateTime, Timelike};

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
}

impl MinuteHelper for NaiveDateTime {
    fn begin_of_minute(&self) -> Self {
        self.with_second(0).unwrap()
    }

    fn end_of_minute(&self) -> Self {
        self.with_second(59).unwrap()
    }

    fn is_same_minute(&self, other: &Self) -> bool {
        self.timestamp() / 1000 / 60 == other.timestamp() / 1000 / 60
    }
}
