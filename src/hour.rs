use chrono::{Duration, NaiveDateTime, Timelike};

/// English: The helper of hour
///
/// 中文: 时助手
pub trait HourHelper {
    /// Add the specified number of hours
    ///
    /// 中文：加上指定的小时数
    fn add_hours(&self, hour: u32) -> Self;
    /// Add the specified number of hours
    ///
    /// 中文：加上指定的小时数
    fn add_hours_opt(&self, hour: u32) -> Option<Self>
    where
        Self: Sized;

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
    /// English: Subtract the specified number
    ///
    /// 中文：减去给定的小时数
    fn sub_hours_opt(&self, hour: u32) -> Option<Self>
    where
        Self: Sized;
}

impl HourHelper for NaiveDateTime {
    fn add_hours(&self, hour: u32) -> Self {
        self.add_hours_opt(hour).expect("Overflowed")
    }

    fn add_hours_opt(&self, hour: u32) -> Option<Self> {
        self.checked_add_signed(Duration::hours(hour as i64))
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
        self.sub_hours_opt(hour).expect("Overflowed")
    }

    fn sub_hours_opt(&self, hour: u32) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add_signed(-Duration::hours(hour as i64))
    }
}
