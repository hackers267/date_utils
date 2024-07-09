use std::cmp::Ordering;
use crate::{
    day::DayHelper,
    utils::{month_type, MonthType},
    WeekHelper,
};
use chrono::{Datelike, Months, NaiveDate, NaiveDateTime};

/// English: The helper of month
///
/// 中文: 月份助手
pub trait MonthHelper {
    /// English: Get the start time of the month.
    ///
    /// 中文: 获取某个月的开始时间
    fn begin_of_month(&self) -> Self;

    /// English: Get the end time of the month.
    ///
    /// 中文: 获取某个月的结束时间
    fn end_of_month(&self) -> Self;
    /// English: To check the current datetime is the same month with the given datetime or not.
    ///
    /// 中文：判断当前时间和指定的时间是否在同一个月
    fn is_same_month(&self, other: &Self) -> bool;
    /// English: Add the specified number of months
    ///
    /// 中文：加上指定月份
    fn add_months(&self, month: i64) -> Self;
    /// English: Get the number of full months between the given dates using trunc as a default rounding method.
    ///
    /// 中文：获取两个日期之间的整月
    fn diff_month(&self, other: &Self) -> i64;
    /// English: Get the number of calendar months between the given dates.
    ///
    /// 中文: 获取日历上两个日期相差多少个月
    fn diff_calendar_month(&self, other: &Self) -> i64;
    /// English: Get all Saturdays and Sundays in the given month.
    ///
    /// 中文: 获取指定月份中所有的周六和周日
    fn each_weekend(&self) -> Vec<(Option<Self>, Option<Self>)>
    where
        Self: Sized;
    /// English: Get all Saturdays and Sundays in the given month with list.
    ///
    /// 中文: 以列表形式获取指定月份中所有的周六和周日
    fn weekend_list(&self) -> Vec<Self>
    where
        Self: Sized;
    /// English: Get the number of days in a month of the given date.
    ///
    /// 中文: 获取指定日期所在月份的天数
    fn days_in_month(&self) -> u32;
    /// English: Is the given date the first day of a month?
    ///
    /// 中文: 判断指定日期是否是一个月的第一天
    fn is_first_day_of_month(&self) -> bool;
    /// English: Is the given date the last day of a month?
    ///
    /// 中文: 判断指定日期是否是一个月的最后一天
    fn is_last_day_of_month(&self) -> bool;
    /// English:Subtract the specified number of months from the given date.
    ///
    /// 中文: 减去指定月份
    fn sub_months(&self, month: i64) -> Self;
    /// English: Return the last day of a month for the given date. The result will be in the local timezone.
    ///
    /// 中文: 返回指定日期所在月份的最后一天
    fn last_day_of_month(&self) -> Self;
}

pub trait Range<T> {
    fn range(&self) -> impl Iterator<Item=T>;
}

impl Range<NaiveDate> for NaiveDate {
    fn range(&self) -> impl Iterator<Item=NaiveDate> {
        let start = self.begin_of_month();
        let end = self.end_of_month();
        let mut count = 0;
        std::iter::from_fn(move || date_from_fn(start, end, &mut count))
    }
}

fn date_from_fn<T>(start: T, end: T, count: &mut u64) -> Option<T>
where
    T: DayHelper + MonthHelper + Ord,
{
    let current = start.add_days(*count);
    match current.cmp(&end) {
        Ordering::Less | Ordering::Equal => {
            *count += 1;
            Some(current)
        }
        _ => None,
    }
}

impl Range<NaiveDateTime> for NaiveDateTime {
    fn range(&self) -> impl Iterator<Item=NaiveDateTime> {
        let start = self.begin_of_month();
        let end = self.end_of_month();
        let mut count = 0;
        std::iter::from_fn(move || date_from_fn(start, end, &mut count))
    }
}

impl MonthHelper for NaiveDate {
    fn begin_of_month(&self) -> Self {
        self.with_day(1).unwrap()
    }

    fn end_of_month(&self) -> Self {
        let month_type = month_type(self.month(), self.year());
        let last_day = match month_type {
            MonthType::Day30 => 30,
            MonthType::Day31 => 31,
            MonthType::Other(false) => 28,
            MonthType::Other(true) => 29,
        };
        self.with_day(last_day).unwrap()
    }

    fn is_same_month(&self, other: &Self) -> bool {
        self.year() == other.year() && self.month0() == other.month0()
    }
    fn add_months(&self, month: i64) -> Self {
        match month.cmp(&0) {
            Ordering::Less => {
                self.checked_sub_months(Months::new(-month as u32)).unwrap()
            }
            _ => self.checked_add_months(Months::new(month as u32)).unwrap()
        }
    }

    fn diff_month(&self, other: &Self) -> i64 {
        let base = self.diff_calendar_month(other);
        if self > other {
            if other.add_months(base) > *self {
                base - 1
            } else {
                base
            }
        } else if self.add_months(base) > *other {
            return base - 1;
        } else {
            return base;
        }
    }

    fn diff_calendar_month(&self, other: &Self) -> i64 {
        if self > other {
            between_months(self, other)
        } else {
            between_months(other, self)
        }
    }

    fn each_weekend(&self) -> Vec<(Option<Self>, Option<Self>)>
    where
        Self: Sized,
    {
        let range = self.weekend_list();
        each_weekend(&range)
    }

    fn weekend_list(&self) -> Vec<Self> {
        self.range().filter(|date| date.is_weekend()).collect()
    }

    fn days_in_month(&self) -> u32 {
        self.end_of_month().day()
    }

    fn is_first_day_of_month(&self) -> bool {
        self.day() == 1
    }

    fn is_last_day_of_month(&self) -> bool {
        self.day() == self.days_in_month()
    }

    fn sub_months(&self, month: i64) -> Self {
        if month.is_negative() {
            self.checked_add_months(Months::new(-month as u32)).unwrap()
        } else {
            self.checked_sub_months(Months::new(month as u32)).unwrap()
        }

    }

    fn last_day_of_month(&self) -> Self {
        self.with_day(self.days_in_month()).unwrap()
    }
}

/// 从给定的日期范围中收集每个周六和周日的组合
///
/// 该函数接受一个日期范围的切片，并返回一个由每个周六和周日的组合构成的向量。
/// 如果切片中的元素数量是奇数，那么最后一个周六将没有与之对应的周日，
/// 因此返回的向量中的最后一个元素将是一个 `(Some(周六), None)` 的元组。
///
/// 参数：
/// - `range`：日期范围的切片，其中 `T` 实现了 `Copy` trait
///
/// 返回值：
/// - 包含每个周六和周日的组合的向量，其中每个元素都是一个 `(Option<T>, Option<T>)` 的元组
///   表示周六和周日（如果存在）。
fn collect_sat_sun<T>(range: &[T]) -> Vec<(Option<T>, Option<T>)>
where
    T: Copy,
{
    range
        .chunks(2)
        .map(|date| (date.first().copied(), date.get(1).copied()))
        .collect::<Vec<_>>()
}

/// 提取给定日期范围内每个周末的日期对
///
/// 该函数会遍历给定的日期范围`range`，并返回一个包含每个周末的日期对的向量。
/// 如果周末的第一天（周六）不在`range`的开始，则第一个元素将是`(None, Some(first_day_of_range))`，
/// 表示范围开始的日期并非周末的开始。
///
/// 参数：
/// - `range`: 日期范围的切片，其中`T`类型实现了`Datelike`和`Copy` trait
///
/// 返回值：
/// - 包含每个周末的日期对的向量，每个周末都由一个`(Option<T>, Option<T>)`元组表示，
///   分别对应周六和周日（如果存在）。如果某个周末不完整（即范围内没有周日），则周日将为`None`。
fn each_weekend<T>(range: &[T]) -> Vec<(Option<T>, Option<T>)>
where
    T: Datelike + Copy,
{
    match range[0].weekday() {
        chrono::Weekday::Sat => collect_sat_sun(&range),
        _ => {
            if let Some((sun, rest)) = range.split_first() {
                let mut result = Vec::with_capacity(5);
                result.push((None, Some(*sun)));
                let rest = collect_sat_sun(rest);
                result.extend(rest);
                result
            } else {
                vec![]
            }
        }
    }
}

fn between_months(one: &NaiveDate, other: &NaiveDate) -> i64 {
    let year_diff = one.year() - other.year();
    let diff_month = one.month0() - other.month0();
    year_diff as i64 * 12 + diff_month as i64
}

impl MonthHelper for NaiveDateTime {
    fn begin_of_month(&self) -> Self {
        self.date().begin_of_month().and_hms_opt(0, 0, 0).unwrap()
    }

    fn end_of_month(&self) -> Self {
        self.date().end_of_month().and_hms_opt(23, 59, 59).unwrap()
    }

    fn is_same_month(&self, other: &Self) -> bool {
        self.date().is_same_month(&other.date())
    }
    fn add_months(&self, month: i64) -> Self {
        if month.is_negative() {
            self.checked_sub_months(Months::new(-month as u32)).unwrap()
        }else {
            self.checked_add_months(Months::new(month as u32)).unwrap()
        }
    }

    fn diff_month(&self, other: &Self) -> i64 {
        let base = self.diff_calendar_month(other);
        if self > other {
            if other.add_months(base) > *self {
                base - 1
            } else {
                base
            }
        } else if self.add_months(base) > *other {
            return base - 1;
        } else {
            return base;
        }
    }

    fn diff_calendar_month(&self, other: &Self) -> i64 {
        let one = self.date();
        let other = other.date();
        one.diff_calendar_month(&other)
    }

    fn each_weekend(&self) -> Vec<(Option<Self>, Option<Self>)>
    where
        Self: Sized,
    {
        let range = self.weekend_list();
        each_weekend(&range)
    }

    fn weekend_list(&self) -> Vec<Self> {
        self.range().filter(|date| date.is_weekend()).collect()
    }

    fn days_in_month(&self) -> u32 {
        self.end_of_month().day()
    }

    fn is_first_day_of_month(&self) -> bool {
        self.day() == 1
    }

    fn is_last_day_of_month(&self) -> bool {
        self.day() == self.days_in_month()
    }

    fn sub_months(&self, month: i64) -> Self {
        if month.is_negative() {
            self.checked_add_months(Months::new(-month as u32)).unwrap()
        } else {
            self.checked_sub_months(Months::new(month as u32)).unwrap()
        }
    }

    fn last_day_of_month(&self) -> Self {
        self.date().end_of_month().and_hms_opt(0, 0, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

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
    fn test_date_begin_of_month() {
        let date = get_date(2000, 6, 6).unwrap();
        let begin = get_date(2000, 6, 1).unwrap();
        assert_eq!(date.begin_of_month(), begin)
    }

    #[test]
    fn test_datetime_begin_of_month() {
        let date = get_time(2000, 6, 6, 6, 6, 6).unwrap();
        let begin = get_time(2000, 6, 1, 0, 0, 0).unwrap();
        assert_eq!(date.begin_of_month(), begin);
    }

    #[test]
    fn test_datetime_end_of_month() {
        let date = get_time(2000, 6, 6, 6, 6, 6).unwrap();
        let end = get_time(2000, 6, 30, 23, 59, 59).unwrap();
        assert_eq!(date.end_of_month(), end);
    }

    #[test]
    fn test_datetime_is_same_month() {
        let one = get_time(2000, 6, 6, 6, 6, 6).unwrap();
        let other = get_time(2000, 6, 1, 0, 0, 0).unwrap();
        assert!(one.is_same_month(&other));
    }
    #[test]
    fn test_date_add_month() {
        let one = get_date(2000, 1, 1);
        let result = one.map(|date| date.add_months(2));
        let actual = get_date(2000, 3, 1);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_datetime_add_month() {
        let one = get_time(2000, 1, 1, 0, 0, 0);
        let result = one.map(|date| date.add_months(2));
        let actual = get_time(2000, 3, 1, 0, 0, 0);
        assert_eq!(result, actual);
    }
}
