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

#[cfg(test)]
mod date {
    use chrono::Months;

    use super::*;

    fn get_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    #[test]
    fn test_date_is_same_true() {
        let one = get_date(2000, 6, 6).unwrap();
        let other = get_date(2000, 6, 6).unwrap();
        assert!(one.is_same(&other))
    }

    #[test]
    fn test_date_is_same_false() {
        let one = get_date(2000, 6, 6).unwrap();
        let other = get_date(2000, 6, 7).unwrap();
        assert!(!one.is_same(&other))
    }

    #[test]
    fn test_date_is_future_true() {
        let now = Utc::now().date_naive();
        let one = now.checked_add_months(Months::new(1)).unwrap();
        assert!(one.is_future());
    }

    #[test]
    fn test_date_is_fature_false() {
        let now = Utc::now().date_naive();
        let one = now.checked_sub_months(Months::new(1)).unwrap();
        assert!(!one.is_future());
    }

    #[test]
    fn test_date_is_past_true() {
        let now = Utc::now().date_naive();
        let one = now.checked_sub_months(Months::new(1)).unwrap();
        assert!(one.is_past());
    }

    #[test]
    fn test_date_is_past_false() {
        let now = Utc::now().date_naive();
        let one = now.checked_add_months(Months::new(1)).unwrap();
        assert!(!one.is_past());
    }
}

#[cfg(test)]
mod datetimes {
    use super::*;

    fn get_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Option<NaiveDateTime> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
    }

    #[test]
    fn test_before_true() {
        let one = get_time(2000, 6, 6, 0, 0, 0).unwrap();
        let other = get_time(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(one.before(&other));
    }

    #[test]
    fn test_before_false() {
        let other = get_time(2000, 6, 6, 0, 0, 0).unwrap();
        let one = get_time(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(!one.before(&other));
    }
}
