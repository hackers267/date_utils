#[cfg(test)]
#[cfg(feature = "week")]
use super::*;
use chrono::Weekday;
use date_utils::{Range, WeekHelper};
use std::iter::zip;

#[test]
fn test_is_monday() {
    let date = calc_date(2023, 10, 2);
    assert!(date.is_monday());
}
#[test]
fn test_is_tuesday() {
    let date = calc_date(2023, 10, 3);
    assert!(date.is_tuesday());
}
#[test]
fn test_is_wednesday() {
    let date = calc_date(2023, 10, 4);
    assert!(date.is_wednesday());
}
#[test]
fn test_is_thursday() {
    let date = calc_date(2023, 10, 5);
    assert!(date.is_thursday());
}
#[test]
fn test_is_friday() {
    let date = calc_date(2023, 10, 6);
    assert!(date.is_friday());
}
#[test]
fn test_is_saturday() {
    let date = calc_date(2023, 10, 7);
    assert!(date.is_saturday());
}
#[test]
fn test_is_sunday() {
    let date = calc_date(2023, 10, 8);
    assert!(date.is_sunday());
}
#[test]
fn test_is_weekend() {
    let sat = calc_date(2023, 10, 7);
    let sun = calc_date(2023, 10, 8);
    let result = [sat, sun].iter().all(|date| date.is_weekend());
    assert!(result);
}
#[test]
fn test_is_workday() {
    let monday = calc_date(2023, 10, 2);
    let tuesday = calc_date(2023, 10, 3);
    let wednesday = calc_date(2023, 10, 4);
    let thursday = calc_date(2023, 10, 5);
    let friday = calc_date(2023, 10, 6);
    let result = [monday, tuesday, wednesday, thursday, friday]
        .iter()
        .all(|date| date.is_workday());
    assert!(result);
}
#[test]
fn test_add_week() {
    let date = calc_date(2023, 10, 2);
    let result = date.add_weeks(1);
    assert_eq!(result, calc_date(2023, 10, 9));
}

#[test]
fn test_add_week_opt() {
    let date = calc_date(2023, 10, 2);
    let result = date.add_weeks_opt(1);
    assert_eq!(result, Some(calc_date(2023, 10, 9)));
}
#[test]
fn test_end_of_week() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week();
    assert_eq!(result, calc_date(2023, 10, 8));
}
#[test]
fn test_end_of_week0() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week0();
    assert_eq!(result, calc_date(2023, 10, 7));
}
#[test]
fn test_begin_of_week() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week();
    assert_eq!(result, calc_date(2023, 10, 2));
}
#[test]
fn test_begin_of_week0() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week0();
    assert_eq!(result, calc_date(2023, 10, 1));
}
#[test]
fn test_begin_of_week_with_sun() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Sun);
    assert_eq!(result, calc_date(2023, 10, 1));
}
#[test]
fn test_begin_of_week_with_sat() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Sat);
    assert_eq!(result, calc_date(2023, 9, 30));
}
#[test]
fn test_begin_of_week_with_mon() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Mon);
    assert_eq!(result, calc_date(2023, 10, 2));
}
#[test]
fn test_begin_of_week_with_tue() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Tue);
    assert_eq!(result, calc_date(2023, 9, 26));
}
#[test]
fn test_begin_of_week_with_wed() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Wed);
    assert_eq!(result, calc_date(2023, 9, 27));
}
#[test]
fn test_begin_of_week_with_thu() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Thu);
    assert_eq!(result, calc_date(2023, 9, 28));
}
#[test]
fn test_begin_of_week_with_fri() {
    let date = calc_date(2023, 10, 2);
    let result = date.begin_of_week_with(Weekday::Fri);
    assert_eq!(result, calc_date(2023, 9, 29));
}
#[test]
fn test_end_of_week_with_sun() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Sun);
    assert_eq!(result, calc_date(2023, 10, 7));
}
#[test]
fn test_end_of_week_with_sat() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Sat);
    assert_eq!(result, calc_date(2023, 10, 6));
}
#[test]
fn test_end_of_week_with_mon() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Mon);
    assert_eq!(result, calc_date(2023, 10, 8));
}
#[test]
fn test_end_of_week_with_tue() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Tue);
    assert_eq!(result, calc_date(2023, 10, 2));
}
#[test]
fn test_end_of_week_with_wed() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Wed);
    assert_eq!(result, calc_date(2023, 10, 3));
}
#[test]
fn test_end_of_week_with_thu() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Thu);
    assert_eq!(result, calc_date(2023, 10, 4));
}
#[test]
fn test_end_of_week_with_fri() {
    let date = calc_date(2023, 10, 2);
    let result = date.end_of_week_with(Weekday::Fri);
    assert_eq!(result, calc_date(2023, 10, 5));
}
#[test]
fn test_sub_weeks() {
    let date = calc_date(2023, 10, 2);
    let result = date.sub_weeks(1);
    assert_eq!(result, calc_date(2023, 9, 25));
}
#[test]
fn test_sub_weeks_opt() {
    let date = calc_date(2023, 10, 2);
    let result = date.sub_weeks_opt(1);
    assert_eq!(result, Some(calc_date(2023, 9, 25)));
}
#[test]
fn test_week_of_month() {
    let date = calc_date(2017, 11, 9);
    let result = date.week_of_month();
    assert_eq!(result, 2);
}
#[test]
fn test_week_of_month0() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(1).collect::<Vec<_>>();
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
fn test_week_of_month_with_sun() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(1).collect::<Vec<_>>();
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
fn test_week_of_month_with_mon() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(2).collect::<Vec<_>>();
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
fn test_week_of_month_with_tue() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(3).collect::<Vec<_>>();
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
fn test_week_of_month_with_wed() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(4).collect::<Vec<_>>();
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
fn test_week_of_month_with_thu() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(5).collect::<Vec<_>>();
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
fn test_week_of_month_with_fri() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(6).collect::<Vec<_>>();
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
fn test_week_of_month_with_sat() {
    let date = calc_date(2023, 7, 2);
    let mut list = date.range();
    let first = list.by_ref().take(7).collect::<Vec<_>>();
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
fn test_weeks_of_month() {
    let date = calc_date(2023, 7, 2);
    let weeks = date.weeks_of_month();
    assert_eq!(weeks, 6);
}
#[test]
fn test_weeks_of_month0() {
    let date = calc_date(2023, 7, 2);
    let weeks = date.weeks_of_month0();
    assert_eq!(weeks, 6);
}
#[test]
fn test_weeks_of_month_with() {
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let date = calc_date(2023, 7, 2);
    let weeks = weekdays
        .iter()
        .map(|d| date.weeks_of_month_with(*d))
        .collect::<Vec<_>>();
    let actual = vec![6, 5, 5, 5, 5, 5, 6];
    assert_eq!(weeks, actual);
}
#[test]
fn test_is_same_week_true() {
    let date = calc_date(2023, 7, 3);
    let other = calc_date(2023, 7, 9);
    let is_same = date.is_same_week(&other);
    assert!(is_same);
}
#[test]
fn test_is_same_week_false() {
    let date = calc_date(2023, 7, 3);
    let other = calc_date(2023, 7, 10);
    let is_same = date.is_same_week(&other);
    assert!(!is_same);
}
#[test]
fn test_is_same_week0_true() {
    let date = calc_date(2023, 7, 2);
    let other = calc_date(2023, 7, 8);
    let is_same = date.is_same_week0(&other);
    assert!(is_same);
}
#[test]
fn test_is_same_week0_false() {
    let date = calc_date(2023, 7, 8);
    let other = calc_date(2023, 7, 9);
    let is_same = date.is_same_week0(&other);
    assert!(!is_same);
}
#[test]
fn test_is_same_week_with() {
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let one_days = (5..=11)
        .map(|d: u32| calc_date(2023, 6, d))
        .collect::<Vec<_>>();
    let next_days = (6..=12)
        .map(|d: u32| calc_date(2023, 7, d))
        .collect::<Vec<_>>();
    let result = zip(zip(one_days, next_days), weekdays)
        .any(|((d1, d2), weekday)| d1.is_same_week_with(&d2, weekday));
    assert!(!result);
}
#[test]
fn test_last_day_of_week() {
    let date = calc_date(2023, 7, 2);
    let last_day = date.last_day_of_week();
    assert_eq!(last_day, calc_date(2023, 7, 2));
}
#[test]
fn test_last_day_of_week0() {
    let date = calc_date(2023, 7, 2);
    let last_day = date.last_day_of_week0();
    assert_eq!(last_day, calc_date(2023, 7, 8));
}
#[test]
fn test_diff_calendar_weeks_positive() {
    let date = calc_date(2014, 7, 21);
    let other = calc_date(2014, 7, 6);
    let diff = date.diff_calendar_weeks(&other);
    assert_eq!(diff, 3);
}
#[test]
fn test_diff_calendar_weeks_navigate() {
    let date = calc_date(2014, 7, 6);
    let other = calc_date(2014, 7, 21);
    let diff = date.diff_calendar_weeks(&other);
    assert_eq!(diff, -3);
}
#[test]
fn test_diff_calendar_weeks0_positive() {
    let date = calc_date(2014, 7, 21);
    let other = calc_date(2014, 7, 6);
    let diff = date.diff_calendar_weeks0(&other);
    assert_eq!(diff, 2);
}
#[test]
fn test_diff_calendar_weeks0_navigate() {
    let date = calc_date(2014, 7, 6);
    let other = calc_date(2014, 7, 21);
    let diff = date.diff_calendar_weeks0(&other);
    assert_eq!(diff, -2);
}
#[test]
fn test_diff_calendar_week_with() {
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let one_days = (2..=8)
        .map(|d: u32| calc_date(2023, 7, d))
        .collect::<Vec<_>>();
    let next_days = (17..=23)
        .map(|d: u32| calc_date(2023, 7, d))
        .collect::<Vec<_>>();
    let result = zip(zip(one_days, next_days), weekdays)
        .map(|((d1, d2), weekday)| d1.diff_calendar_weeks_with(&d2, weekday))
        .collect::<Vec<_>>();
    assert_eq!(result, vec![-3; 7]);
}
#[test]
fn test_diff_weeks() {
    let date = calc_date(2023, 7, 9);
    let other = calc_date(2023, 7, 2);
    let diff = date.diff_weeks(&other);
    assert_eq!(diff, 1);
}
#[test]
fn test_next_day() {
    let date = calc_date(2023, 7, 5);
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let result = weekdays.map(|d| date.next_day(d)).to_vec();
    let actual = date.range().skip(9).take(7).collect::<Vec<_>>();
    assert_eq!(result, actual);
}
#[test]
fn test_previous_day() {
    let date = calc_date(2023, 7, 14);
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let result = weekdays.map(|d| date.previous_day(d)).to_vec();
    let actual = date.range().skip(2).take(7).collect::<Vec<_>>();
    assert_eq!(result, actual);
}
#[test]
fn test_next_monday_and_so_on() {
    let date = calc_date(2023, 7, 14);
    let next_monday = date.next_monday();
    let next_tuesday = date.next_tuesday();
    let next_wednesday = date.next_wednesday();
    let next_thursday = date.next_thursday();
    let next_friday = date.next_friday();
    let next_saturday = date.next_saturday();
    let next_sunday = date.next_sunday();
    assert_eq!(next_monday, calc_date(2023, 7, 17));
    assert_eq!(next_tuesday, calc_date(2023, 7, 18));
    assert_eq!(next_wednesday, calc_date(2023, 7, 19));
    assert_eq!(next_thursday, calc_date(2023, 7, 20));
    assert_eq!(next_friday, calc_date(2023, 7, 21));
    assert_eq!(next_saturday, calc_date(2023, 7, 22));
    assert_eq!(next_sunday, calc_date(2023, 7, 23));
}
#[test]
fn test_previous_monday_and_so_on() {
    let date = calc_date(2023, 7, 14);
    let previous_monday = date.previous_monday();
    let previous_tuesday = date.previous_tuesday();
    let previous_wednesday = date.previous_wednesday();
    let previous_thursday = date.previous_thursday();
    let previous_friday = date.previous_friday();
    let previous_saturday = date.previous_saturday();
    let previous_sunday = date.previous_sunday();
    assert_eq!(previous_monday, calc_date(2023, 7, 3));
    assert_eq!(previous_tuesday, calc_date(2023, 7, 4));
    assert_eq!(previous_wednesday, calc_date(2023, 7, 5));
    assert_eq!(previous_thursday, calc_date(2023, 7, 6));
    assert_eq!(previous_friday, calc_date(2023, 7, 7));
    assert_eq!(previous_saturday, calc_date(2023, 7, 8));
    assert_eq!(previous_sunday, calc_date(2023, 7, 9));
}
