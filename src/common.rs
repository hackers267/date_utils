use chrono::{NaiveDate, NaiveDateTime, Utc};

/// English: Checks if the given arguments convert to an existing date.
///
/// 中文: 检查给定的参数是否可以转换为一个存在的日期。
pub fn is_exist(year: i32, month: u32, day: u32) -> bool {
    let date = NaiveDate::from_ymd_opt(year, month, day);
    date.is_some()
}

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
    /// English: Return a date from the array closest to the given date.
    ///
    /// 中文: 返回数组中与给定日期最近的日期
    fn closest_to(&self, dates: &[Self]) -> Option<Self>
    where
        Self: Sized;
    /// English: Return an index of the closest date from the array comparing to the given date.
    ///
    /// 中文: 返回数组中与给定日期最近的日期的索引
    fn closest_to_index(&self, dates: &[Self]) -> Option<usize>
    where
        Self: Sized;
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

    fn closest_to(&self, dates: &[Self]) -> Option<Self> {
        dates
            .iter()
            .map(|date| {
                let one = *self;
                let other = *date;
                let diff = one.signed_duration_since(other);
                (diff.num_seconds().abs(), other)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, date)| date)
    }

    fn closest_to_index(&self, dates: &[Self]) -> Option<usize> {
        dates
            .iter()
            .enumerate()
            .map(|(i, date)| {
                let one = *self;
                let other = *date;
                let diff = one.signed_duration_since(other);
                (diff.num_seconds().abs(), i)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, index)| index)
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

    fn closest_to(&self, dates: &[Self]) -> Option<Self> {
        dates
            .iter()
            .map(|date| {
                let one = *self;
                let other = *date;
                let diff = one.signed_duration_since(other);
                (diff.num_seconds().abs(), other)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, date)| date)
    }

    fn closest_to_index(&self, dates: &[Self]) -> Option<usize> {
        dates
            .iter()
            .enumerate()
            .map(|(i, date)| {
                let one = *self;
                let other = *date;
                let diff = one.signed_duration_since(other);
                (diff.num_seconds().abs(), i)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, index)| index)
    }
}

#[cfg(test)]
mod date {
    use super::*;
    use chrono::Months;

    fn get_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    #[test]
    fn test_is_exist() {
        assert!(is_exist(2000, 6, 6));
        assert!(!is_exist(2000, 13, 6));
        assert!(!is_exist(2000, 6, 32));
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
    fn test_date_is_future_false() {
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
    #[test]
    fn test_close_to() {
        let dates = vec![
            get_date(2000, 6, 6).unwrap(),
            get_date(2000, 6, 7).unwrap(),
            get_date(2000, 6, 8).unwrap(),
        ];
        let date = get_date(2000, 6, 5).unwrap();
        assert_eq!(
            date.closest_to(&dates).unwrap(),
            get_date(2000, 6, 6).unwrap()
        );
    }
    #[test]
    fn test_close_to_index() {
        let dates = vec![
            get_date(2000, 6, 6).unwrap(),
            get_date(2000, 6, 7).unwrap(),
            get_date(2000, 6, 8).unwrap(),
        ];
        let date = get_date(2000, 6, 5).unwrap();
        assert_eq!(date.closest_to_index(&dates).unwrap(), 0);
    }
}

#[cfg(test)]
mod date_times {
    use super::*;
    use crate::test::get_time_opt;
    use chrono::Duration;

    #[test]
    fn test_before_true() {
        let one = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        let other = get_time_opt(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(one.before(&other));
    }

    #[test]
    fn test_before_false() {
        let other = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        let one = get_time_opt(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(!one.before(&other));
    }

    #[test]
    fn test_after_true() {
        let other = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        let one = get_time_opt(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(one.after(&other));
    }

    #[test]
    fn test_after_false() {
        let one = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        let other = get_time_opt(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(!one.after(&other));
    }

    #[test]
    fn test_is_same_true() {
        let other = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        let one = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        assert!(one.is_same(&other));
    }

    #[test]
    fn test_is_same_false() {
        let one = get_time_opt(2000, 6, 6, 0, 0, 0).unwrap();
        let other = get_time_opt(2001, 1, 1, 0, 0, 0).unwrap();
        assert!(!one.is_same(&other));
    }

    #[test]
    fn test_is_future_true() {
        let now = Utc::now().naive_utc();
        let one = now.checked_add_signed(Duration::hours(3)).unwrap();
        assert!(one.is_future());
    }

    #[test]
    fn test_is_future_false() {
        let now = Utc::now().naive_utc();
        let one = now.checked_sub_signed(Duration::hours(3)).unwrap();
        assert!(!one.is_future());
    }
    #[test]
    fn test_is_past_true() {
        let now = Utc::now().naive_utc();
        let one = now.checked_sub_signed(Duration::hours(3)).unwrap();
        assert!(one.is_past());
    }

    #[test]
    fn test_is_past_false() {
        let now = Utc::now().naive_utc();
        let one = now.checked_add_signed(Duration::hours(3)).unwrap();
        assert!(!one.is_past());
    }
    #[test]
    fn test_closet_to() {
        let now = Utc::now().naive_utc();
        let one = now.checked_add_signed(Duration::hours(3)).unwrap();
        let two = now.checked_add_signed(Duration::hours(2)).unwrap();
        let three = now.checked_add_signed(Duration::hours(1)).unwrap();
        assert_eq!(one.closest_to(&[two, three]), Some(two));
    }
    #[test]
    fn test_closet_to_index() {
        let now = Utc::now().naive_utc();
        let one = now.checked_add_signed(Duration::hours(3)).unwrap();
        let two = now.checked_add_signed(Duration::hours(2)).unwrap();
        let three = now.checked_add_signed(Duration::hours(1)).unwrap();
        assert_eq!(one.closest_to_index(&[two, three]), Some(0));
    }
}
