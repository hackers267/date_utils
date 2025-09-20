#[cfg(test)]
#[cfg(all(feature = "week",feature = "range"))]
use date_utils::{Range, WeekHelper};
use chrono::Weekday;
use std::iter::zip;
use crate::calc_datetime;

#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_monday() {
    let date = calc_datetime(2023, 10, 2, 0, 0, 0);
    assert!(date.is_monday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_tuesday() {
    let date = calc_datetime(2023, 10, 3, 0, 0, 0);
    assert!(date.is_tuesday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_wednesday() {
    let date = calc_datetime(2023, 10, 4, 0, 0, 0);
    assert!(date.is_wednesday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_thursday() {
    let date = calc_datetime(2023, 10, 5, 0, 0, 0);
    assert!(date.is_thursday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_friday() {
    let date = calc_datetime(2023, 10, 6, 0, 0, 0);
    assert!(date.is_friday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_saturday() {
    let date = calc_datetime(2023, 10, 7, 0, 0, 0);
    assert!(date.is_saturday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_sunday() {
    let date = calc_datetime(2023, 10, 8, 0, 0, 0);
    assert!(date.is_sunday());
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_weekend() {
    let sat = calc_datetime(2023, 10, 7, 0, 0, 0);
    let sun = calc_datetime(2023, 10, 8, 0, 0, 0);
    let result = [sat, sun].iter().all(|date| date.is_weekend());
    assert!(result);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_workday() {
    let mon = calc_datetime(2023, 10, 2, 0, 0, 0);
    let tue = calc_datetime(2023, 10, 3, 0, 0, 0);
    let wed = calc_datetime(2023, 10, 4, 0, 0, 0);
    let thu = calc_datetime(2023, 10, 5, 0, 0, 0);
    let fri = calc_datetime(2023, 10, 6, 0, 0, 0);
    let result = [mon, tue, wed, thu, fri]
        .iter()
        .all(|date| date.is_workday());
    assert!(result);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_add_week() {
    let date = calc_datetime(2023, 10, 2, 0, 0, 0);
    let actual = date.add_weeks(1);
    let expected = calc_datetime(2023, 10, 9, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_add_week_opt() {
    let date = calc_datetime(2023, 10, 2, 0, 0, 0);
    let actual = date.add_weeks_opt(1);
    let expected = calc_datetime(2023, 10, 9, 0, 0, 0);
    assert_eq!(actual, Some(expected));
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_end_of_week() {
    let date = calc_datetime(2023, 10, 2, 0, 0, 0);
    let actual = date.end_of_week();
    let expected = calc_datetime(2023, 10, 8, 23, 59, 59);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_end_of_week0() {
    let date = calc_datetime(2023, 10, 2, 0, 0, 0);
    let actual = date.end_of_week0();
    let expected = calc_datetime(2023, 10, 7, 23, 59, 59);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week();
    let expected = calc_datetime(2023, 10, 2, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week0() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week0();
    let expected = calc_datetime(2023, 10, 1, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_sat() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Sat);
    let expected = calc_datetime(2023, 9, 30, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_sun() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Sun);
    let expected = calc_datetime(2023, 10, 1, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_mon() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Mon);
    let expected = calc_datetime(2023, 10, 2, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_tue() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Tue);
    let expected = calc_datetime(2023, 9, 26, 0, 0, 0);
    assert_eq!(actual, expected);
}

#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_wed() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Wed);
    let expected = calc_datetime(2023, 9, 27, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_thu() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Thu);
    let expected = calc_datetime(2023, 9, 28, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_begin_of_week_fri() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.begin_of_week_with(Weekday::Fri);
    let expected = calc_datetime(2023, 9, 29, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_sub_week() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.sub_weeks(1);
    let expected = calc_datetime(2023, 9, 25, 12, 12, 12);
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_sub_week_opt() {
    let date = calc_datetime(2023, 10, 2, 12, 12, 12);
    let actual = date.sub_weeks_opt(1);
    let expected = Some(calc_datetime(2023, 9, 25, 12, 12, 12));
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month() {
    let date = calc_datetime(2017, 11, 9, 12, 12, 12);
    let actual = date.week_of_month();
    let expected = 2;
    assert_eq!(actual, expected);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month0() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(3).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first.iter().all(|d| d.week_of_month0() == 1);
    let rest_week = rest
        .iter()
        .enumerate()
        .all(|(i, list)| list.iter().all(|d| d.week_of_month0() == i as u8 + 2));
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_sun() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(3).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Sun) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Sun) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_mon() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(4).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Mon) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Mon) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_tue() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(5).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Tue) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Tue) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_wed() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(6).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Wed) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Wed) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_thu() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(7).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Thu) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Thu) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_fri() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(1).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Fri) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Fri) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_week_of_month_with_sat() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let mut list = date.range();
    let first = list.by_ref().take(2).collect::<Vec<_>>();
    let rest = list.collect::<Vec<_>>();
    let rest = rest.chunks(7).collect::<Vec<_>>();
    let is_first = first
        .iter()
        .all(|d| d.week_of_month_with(Weekday::Sat) == 1);
    let rest_week = rest.iter().enumerate().all(|(i, list)| {
        list.iter()
            .all(|d| d.week_of_month_with(Weekday::Sat) == i as u8 + 2)
    });
    assert!(is_first && rest_week);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_weeks_of_month() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let weeks = date.weeks_of_month();
    assert_eq!(weeks, 5);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_weeks_of_month0() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let weeks = date.weeks_of_month0();
    assert_eq!(weeks, 5);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_weeks_of_month_with() {
    let date = calc_datetime(2023, 6, 3, 12, 12, 12);
    let weekdays = [
        Weekday::Sun,
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
    ];
    let weeks = weekdays
        .iter()
        .map(|w| date.weeks_of_month_with(*w))
        .collect::<Vec<_>>();
    assert_eq!(weeks, [5, 5, 5, 5, 5, 6, 5]);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_same_week_true() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let other = calc_datetime(2023, 6, 10, 12, 12, 12);
    let is_same = date.is_same_week(&other);
    assert!(is_same);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_same_week_false() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let other = calc_datetime(2023, 6, 12, 12, 12, 12);
    let is_same = date.is_same_week(&other);
    assert!(!is_same);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_same_week0_true() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let other = calc_datetime(2023, 6, 4, 12, 12, 12);
    let is_same = date.is_same_week0(&other);
    assert!(is_same);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_same_week0_false() {
    let date = calc_datetime(2023, 6, 10, 12, 12, 12);
    let other = calc_datetime(2023, 6, 11, 12, 12, 12);
    let is_same = date.is_same_week0(&other);
    assert!(!is_same);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_is_same_week_with_true() {
    let weekdays = [
        Weekday::Sun,
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
    ];
    let one_days = (1..=7)
        .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
        .collect::<Vec<_>>();
    let next_days = (2..=8)
        .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
        .collect::<Vec<_>>();
    let result = zip(zip(one_days, next_days), weekdays)
        .any(|((d1, d2), weekday)| d1.is_same_week_with(&d2, weekday));
    assert!(result);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_last_day_of_week() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let last_day = date.last_day_of_week();
    let actual = calc_datetime(2023, 6, 11, 0, 0, 0);
    assert_eq!(last_day, actual);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_last_day_of_week0() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let last_day = date.last_day_of_week0();
    let actual = calc_datetime(2023, 6, 10, 0, 0, 0);
    assert_eq!(last_day, actual);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_diff_calendar_with() {
    let weekdays = [
        Weekday::Sun,
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
    ];
    let one_days = (18..=24)
        .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
        .collect::<Vec<_>>();
    let next_days = (4..=10)
        .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
        .collect::<Vec<_>>();
    let result = zip(zip(one_days, next_days), weekdays)
        .map(|((d1, d2), weekday)| d1.diff_calendar_weeks_with(&d2, weekday))
        .collect::<Vec<_>>();
    assert_eq!(result, vec![2; 7]);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_diff_weeks() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let other = calc_datetime(2023, 6, 12, 12, 12, 12);
    let diff = date.diff_weeks(&other);
    assert_eq!(diff, -1);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_diff_weeks1() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let other = calc_datetime(2023, 6, 12, 12, 12, 11);
    let diff = date.diff_weeks(&other);
    assert_eq!(diff, 0);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_next_day() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let next_days = weekdays.map(|w| date.next_day(w)).to_vec();
    let actual = date.range().skip(11).take(7).collect::<Vec<_>>();
    assert_eq!(next_days, actual);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_prev_day() {
    let date = calc_datetime(2023, 6, 15, 12, 12, 12);
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let prev_days = weekdays.map(|w| date.previous_day(w)).to_vec();
    let actual = date.range().skip(4).take(7).collect::<Vec<_>>();
    assert_eq!(prev_days, actual);
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_next_monday_and_so_on() {
    let date = calc_datetime(2023, 6, 5, 12, 12, 12);
    let next_monday = date.next_monday();
    let next_tuesday = date.next_tuesday();
    let next_wednesday = date.next_wednesday();
    let next_thursday = date.next_thursday();
    let next_friday = date.next_friday();
    let next_saturday = date.next_saturday();
    let next_sunday = date.next_sunday();
    assert_eq!(next_monday, calc_datetime(2023, 6, 12, 0, 0, 0));
    assert_eq!(next_tuesday, calc_datetime(2023, 6, 13, 0, 0, 0));
    assert_eq!(next_wednesday, calc_datetime(2023, 6, 14, 0, 0, 0));
    assert_eq!(next_thursday, calc_datetime(2023, 6, 15, 0, 0, 0));
    assert_eq!(next_friday, calc_datetime(2023, 6, 16, 0, 0, 0));
    assert_eq!(next_saturday, calc_datetime(2023, 6, 17, 0, 0, 0));
    assert_eq!(next_sunday, calc_datetime(2023, 6, 18, 0, 0, 0));
}
#[test]
#[cfg(all(feature = "week",feature = "range"))]
fn test_prev_monday_and_so_on() {
    let date = calc_datetime(2023, 6, 15, 12, 12, 12);
    let prev_monday = date.previous_monday();
    let prev_tuesday = date.previous_tuesday();
    let prev_wednesday = date.previous_wednesday();
    let prev_thursday = date.previous_thursday();
    let prev_friday = date.previous_friday();
    let prev_saturday = date.previous_saturday();
    let prev_sunday = date.previous_sunday();
    assert_eq!(prev_monday, calc_datetime(2023, 6, 5, 0, 0, 0));
    assert_eq!(prev_tuesday, calc_datetime(2023, 6, 6, 0, 0, 0));
    assert_eq!(prev_wednesday, calc_datetime(2023, 6, 7, 0, 0, 0));
    assert_eq!(prev_thursday, calc_datetime(2023, 6, 8, 0, 0, 0));
    assert_eq!(prev_friday, calc_datetime(2023, 6, 9, 0, 0, 0));
    assert_eq!(prev_saturday, calc_datetime(2023, 6, 10, 0, 0, 0));
    assert_eq!(prev_sunday, calc_datetime(2023, 6, 11, 0, 0, 0));
}
