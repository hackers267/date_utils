use chrono::{NaiveDate, NaiveDateTime, Utc};

pub trait CommonHelper {
    /// English: To check the current date or datetime is before the given date or datetime or not.
    ///
    /// 中文: 判断当前日期或时间是否在指定的日期或时间之前
    fn before(&self, other: &Self) -> bool;

    /// English: To check the current date or datetime is after the given date or datetime or not.
    ///
    /// 中文: 判断当前日期或时间是否在指定的日期或时间之后
    fn after(&self, other: &Self) -> bool;

    /// English: To check the current date is same  with the given date or datetime or not.
    ///
    /// 中文: 判断当前日期或时间是否和给定的日期或时间相等
    fn is_same(&self, other: &Self) -> bool;

    /// English: To check the date or datetime is future or not.
    ///
    /// 中文: 判断日期或时间是否是未来
    fn is_future(&self) -> bool;

    /// English: To check the date or datetime is past or not.
    ///
    /// 中文: 判断日期或时间是否在过去
    fn is_past(&self) -> bool;
}

impl CommonHelper for NaiveDate {
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    fn after(&self, other: &Self) -> bool {
        self > other
    }

    fn is_same(&self, other: &Self) -> bool {
        self == other
    }

    fn is_future(&self) -> bool {
        let now = Utc::now().date_naive();
        self > &now
    }

    fn is_past(&self) -> bool {
        let now = Utc::now().date_naive();
        self < &now
    }
}

impl CommonHelper for NaiveDateTime {
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    fn after(&self, other: &Self) -> bool {
        self > other
    }

    fn is_same(&self, other: &Self) -> bool {
        self == other
    }

    fn is_future(&self) -> bool {
        let now = Utc::now().naive_utc();
        self > &now
    }

    fn is_past(&self) -> bool {
        let now = Utc::now().naive_utc();
        self < &now
    }
}
