#[cfg(feature = "month")]
use date_utils::{MonthHelper, Range};

#[test]
#[cfg(feature = "month")]
fn test_is_same_month_date_true() {
    let date = calc_date(2008, 8, 8);
    let other = calc_date(2008, 8, 12);
    assert!(date.is_same_month(&other))
}
#[test]
#[cfg(feature = "month")]
fn test_is_same_month_date_false() {
    let date = calc_date(2008, 8, 8);
    let other = calc_date(2008, 1, 1);
    assert!(!date.is_same_month(&other));
}

#[test]
#[cfg(feature = "month")]
fn test_end_of_month_may_true() {
    let date = calc_date(2008, 5, 5);
    let end = calc_date(2008, 5, 31);
    let result = date.end_of_month();
    assert_eq!(result, end);
}
#[test]
#[cfg(feature = "month")]
fn test_end_of_month_february_2006_true() {
    let date = calc_date(2006, 2, 2);
    let end = calc_date(2006, 2, 28);
    let result = date.end_of_month();
    assert_eq!(result, end);
}
#[test]
#[cfg(feature = "month")]
fn test_end_of_month_april_true() {
    let date = calc_date(2008, 4, 5);
    let end = calc_date(2008, 4, 30);
    let result = date.end_of_month();
    assert_eq!(result, end);
}

#[test]
#[cfg(feature = "month")]
fn test_end_of_month_february_2008_true() {
    let date = calc_date(2008, 2, 5);
    let end = calc_date(2008, 2, 29);
    let result = date.end_of_month();
    assert_eq!(result, end);
}

#[test]
#[cfg(feature = "month")]
fn test_add_months() {
    let one = calc_date(2023, 4, 8);
    let result = one.add_months(8);
    let actual = calc_date(2023, 12, 8);
    assert_eq!(result, actual);
}
#[test]
#[cfg(feature = "month")]
fn test_add_months_opt() {
    let one = calc_date(2023, 4, 8);
    let result = one.add_months_opt(8);
    assert_eq!(result, Some(calc_date(2023, 12, 8)));
}
#[test]
#[cfg(feature = "month")]
fn test_sub_months_opt() {
    let one = calc_date(2023, 4, 8);
    let result = one.sub_months_opt(8);
    assert_eq!(result, Some(calc_date(2022, 8, 8)));
}
#[test]
#[cfg(feature = "month")]
fn test_diff_in_calendar_months() {
    let one = calc_date(2023, 4, 8);
    let other = calc_date(2022, 1, 7);
    let result = one.diff_calendar_months(&other);
    assert_eq!(result, 15);
}
#[test]
#[cfg(feature = "month")]
fn test_diff_in_months() {
    let one = calc_date(2023, 4, 8);
    let other = calc_date(2022, 1, 9);
    let result = one.diff_months(&other);
    assert_eq!(result, 14);
}
#[test]
#[cfg(feature = "month")]
fn test_each_weekend() {
    let date = calc_date(2024, 6, 1);
    let weekends = date.each_weekend();
    let actual = vec![
        (Some(calc_date(2024, 6, 1)), Some(calc_date(2024, 6, 2))),
        (Some(calc_date(2024, 6, 8)), Some(calc_date(2024, 6, 9))),
        (Some(calc_date(2024, 6, 15)), Some(calc_date(2024, 6, 16))),
        (Some(calc_date(2024, 6, 22)), Some(calc_date(2024, 6, 23))),
        (Some(calc_date(2024, 6, 29)), Some(calc_date(2024, 6, 30))),
    ];
    assert_eq!(weekends, actual);
}
#[test]
#[cfg(feature = "month")]
fn test_each_weekend_first_sat_is_none() {
    let date = calc_date(2023, 10, 1);
    let weekends = date.each_weekend();
    let actual = vec![
        (None, Some(calc_date(2023, 10, 1))),
        (Some(calc_date(2023, 10, 7)), Some(calc_date(2023, 10, 8))),
        (Some(calc_date(2023, 10, 14)), Some(calc_date(2023, 10, 15))),
        (Some(calc_date(2023, 10, 21)), Some(calc_date(2023, 10, 22))),
        (Some(calc_date(2023, 10, 28)), Some(calc_date(2023, 10, 29))),
    ];
    assert_eq!(weekends, actual);
}
#[test]
#[cfg(feature = "month")]
fn test_each_weekend_last_none_is_none() {
    let date = calc_date(2023, 9, 1);
    let weekends = date.each_weekend();
    let actual = vec![
        (Some(calc_date(2023, 9, 2)), Some(calc_date(2023, 9, 3))),
        (Some(calc_date(2023, 9, 9)), Some(calc_date(2023, 9, 10))),
        (Some(calc_date(2023, 9, 16)), Some(calc_date(2023, 9, 17))),
        (Some(calc_date(2023, 9, 23)), Some(calc_date(2023, 9, 24))),
        (Some(calc_date(2023, 9, 30)), None),
    ];
    assert_eq!(weekends, actual);
}
#[test]
#[cfg(feature = "month")]
fn test_weekend_list() {
    let date = calc_date(2023, 9, 1);
    let weekends = date.weekend_list();
    let actual = vec![
        calc_date(2023, 9, 2),
        calc_date(2023, 9, 3),
        calc_date(2023, 9, 9),
        calc_date(2023, 9, 10),
        calc_date(2023, 9, 16),
        calc_date(2023, 9, 17),
        calc_date(2023, 9, 23),
        calc_date(2023, 9, 24),
        calc_date(2023, 9, 30),
    ];
    assert_eq!(actual, weekends);
}
#[test]
#[cfg(feature = "month")]
fn test_range() {
    let date = calc_date(2023, 9, 1);
    let list = date.range().collect::<Vec<_>>();
    assert_eq!(list.len(), 30);
    assert_eq!(list[0], calc_date(2023, 9, 1));
    assert_eq!(list.last(), Some(&calc_date(2023, 9, 30)));
}
#[test]
#[cfg(feature = "month")]
fn test_days_of_month() {
    let list = [
        calc_date(2023, 1, 3),
        calc_date(2023, 2, 3),
        calc_date(2023, 3, 3),
        calc_date(2023, 4, 3),
        calc_date(2023, 5, 3),
        calc_date(2023, 6, 3),
        calc_date(2023, 7, 3),
        calc_date(2023, 8, 3),
        calc_date(2023, 9, 3),
        calc_date(2023, 10, 3),
        calc_date(2023, 11, 3),
        calc_date(2023, 12, 3),
    ];
    let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
    let actual = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    assert_eq!(result, actual);
}
#[test]
#[cfg(feature = "month")]
fn test_days_of_month_leap_year() {
    let list = [
        calc_date(2024, 1, 3),
        calc_date(2024, 2, 3),
        calc_date(2024, 3, 3),
        calc_date(2024, 4, 3),
        calc_date(2024, 5, 3),
        calc_date(2024, 6, 3),
        calc_date(2024, 7, 3),
        calc_date(2024, 8, 3),
        calc_date(2024, 9, 3),
        calc_date(2024, 10, 3),
        calc_date(2024, 11, 3),
        calc_date(2024, 12, 3),
    ];
    let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
    let actual = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    assert_eq!(result, actual);
}
#[test]
#[cfg(feature = "month")]
fn test_is_first_day_of_month_true() {
    let date = calc_date(2023, 1, 1);
    assert!(date.is_first_day_of_month());
}
#[test]
#[cfg(feature = "month")]
fn test_is_last_day_of_month_false() {
    let date = calc_date(2023, 1, 31);
    assert!(!date.is_first_day_of_month());
}
#[test]
#[cfg(feature = "month")]
fn test_is_last_day_of_month_true() {
    let date = calc_date(2023, 1, 31);
    assert!(date.is_last_day_of_month());
}
#[test]
#[cfg(feature = "month")]
fn test_is_first_day_of_month_false() {
    let date = calc_date(2023, 1, 1);
    assert!(!date.is_last_day_of_month());
}
#[test]
#[cfg(feature = "month")]
fn test_sub_months() {
    let date = calc_date(2023, 1, 1);
    let result = date.sub_months(1);
    assert_eq!(result, calc_date(2022, 12, 1));
}
#[test]
#[cfg(feature = "month")]
fn test_last_day_of_month() {
    let date = calc_date(2023, 1, 1);
    let result = date.last_day_of_month();
    assert_eq!(result, calc_date(2023, 1, 31));
}
