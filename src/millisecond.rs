use chrono::{Duration, NaiveDateTime, Timelike};

pub trait MillisecondHelper {
    /// English: Add the specified number of milliseconds to the given date.
    ///
    /// 中文: 给指定的日期添加指定的毫秒数。
    fn add_millisecond(&self, millisecond: i32) -> Self;
    /// English: Add the specified number of milliseconds to the given date.
    ///
    /// 中文: 给指定的日期添加指定的毫秒数。
    fn add_millisecond_opt(&self, millisecond: i32) -> Option<Self>
    where
        Self: Sized;
    /// English: Subtract the specified number of milliseconds from the given date.
    ///
    /// 中文: 给指定的日期减去指定的毫秒数。
    fn sub_millisecond(&self, millisecond: i32) -> Self;
    /// English: Subtract the specified number of milliseconds from the given date.
    ///
    /// 中文: 给指定的日期减去指定的毫秒数。
    fn sub_millisecond_opt(&self, millisecond: i32) -> Option<Self>
    where
        Self: Sized;
    /// English: Get the number of milliseconds between the given dates.
    ///
    /// 中文: 获取两个日期之间的毫秒数。
    fn diff_milliseconds(&self, other: &Self) -> i64;
    /// English: Get the milliseconds of the given date.
    ///
    /// 中文: 获取指定日期的毫秒数。
    fn millisecond(&self) -> i64;
    /// English: Set the milliseconds to the given date.
    ///
    /// 中文: 设置指定日期的毫秒数。
    fn set_millisecond(&self, millisecond: i64) -> Self;
}

impl MillisecondHelper for NaiveDateTime {
    fn add_millisecond(&self, millisecond: i32) -> Self {
        self.add_millisecond_opt(millisecond).unwrap()
    }

    fn add_millisecond_opt(&self, millisecond: i32) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add_signed(Duration::milliseconds(millisecond as i64))
    }

    fn sub_millisecond(&self, millisecond: i32) -> Self {
        self.sub_millisecond_opt(millisecond).unwrap()
    }

    fn sub_millisecond_opt(&self, millisecond: i32) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_sub_signed(Duration::milliseconds(millisecond as i64))
    }

    fn diff_milliseconds(&self, other: &Self) -> i64 {
        self.signed_duration_since(*other).num_milliseconds()
    }

    fn millisecond(&self) -> i64 {
        (self.nanosecond() / 1_000_000) as i64
    }

    fn set_millisecond(&self, millisecond: i64) -> Self {
        self.with_nanosecond(millisecond as u32 * 1_000_000)
            .unwrap()
    }
}
