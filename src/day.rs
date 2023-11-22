use chrono::NaiveDateTime;

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
