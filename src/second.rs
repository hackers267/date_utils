use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Timelike;

/// English: SecondHelper
///
/// 中文：秒助手
pub trait SecondHelper {
    /// English: Add the specified number of seconds
    ///
    /// 中文: 加上指定的秒数
    fn add_seconds(&self, second: i64) -> Self;
    /// English: Add the specified number of seconds
    ///
    /// 中文: 加上指定的秒数
    fn add_seconds_opt(&self, second: i64) -> Option<Self>
    where
        Self: Sized;

    /// English: Subtract the specified number of seconds
    ///
    /// 中文：减去指定的秒数
    fn sub_seconds(&self, second: i64) -> Self;
    /// English: Subtract the specified number of seconds
    ///
    /// 中文：减去指定的秒数
    fn sub_seconds_opt(&self, second: i64) -> Option<Self>
    where
        Self: Sized;
    fn diff_seconds(&self, other: &Self) -> i64;
    fn begin_of_second(&self) -> Self;

    /// English: Is the same second with the given one
    ///
    /// 中文：是否和指定的内容是同一秒
    fn is_same_second(&self, other: &Self) -> bool;

    /// English: Return the end of a second for the given date.
    ///
    /// 中文：返回本身的秒的结束时间
    fn end_of_second(&self) -> Self;
}

impl SecondHelper for NaiveDateTime {
    fn add_seconds(&self, second: i64) -> Self {
        self.add_seconds_opt(second).unwrap()
    }

    fn add_seconds_opt(&self, second: i64) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add_signed(Duration::seconds(second))
    }

    fn sub_seconds(&self, second: i64) -> Self {
        self.sub_seconds_opt(second).unwrap()
    }

    fn sub_seconds_opt(&self, second: i64) -> Option<Self> {
        self.checked_sub_signed(Duration::seconds(second))
    }

    fn diff_seconds(&self, other: &Self) -> i64 {
        self.and_utc().timestamp() - other.and_utc().timestamp()
    }

    fn begin_of_second(&self) -> Self
    where
        Self: Sized,
    {
        let date = self.date();
        let time = self.time();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();
        date.and_hms_micro_opt(hour, minute, second, 0).unwrap()
    }

    fn is_same_second(&self, other: &Self) -> bool {
        self.begin_of_second() == other.begin_of_second()
    }

    fn end_of_second(&self) -> Self {
        let date = self.date();
        let time = self.time();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();
        date.and_hms_micro_opt(hour, minute, second, 999).unwrap()
    }
}
