#[cfg(test)]
#[cfg(feature = "month")]
use super::*;
use date_utils::MonthHelper;

#[test]
fn test_datetime_add_months() {
    let one = calc_datetime(2023, 4, 8, 11, 11, 11);
    let result = one.add_months(7);
    let actual = calc_datetime(2023, 11, 8, 11, 11, 11);
    assert_eq!(result, actual);
}
#[test]
fn test_datetime_add_months_opt() {
    let one = calc_datetime(2023, 4, 8, 11, 11, 11);
    let result = one.add_months_opt(7);
    let actual = calc_datetime(2023, 11, 8, 11, 11, 11);
    assert_eq!(result, Some(actual));
}
#[test]
fn test_datetime_diff_in_months() {
    let one = calc_datetime(2023, 4, 8, 11, 11, 11);
    let other = calc_datetime(2022, 1, 9, 11, 11, 11);
    let result = one.diff_months(&other);
    assert_eq!(result, 14);
}
#[test]
fn test_each_weekend() {
    let date = calc_datetime(2024, 6, 1, 0, 0, 0);
    let weekends = date.each_weekend();
    let actual = vec![
        (
            Some(calc_datetime(2024, 6, 1, 0, 0, 0)),
            Some(calc_datetime(2024, 6, 2, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2024, 6, 8, 0, 0, 0)),
            Some(calc_datetime(2024, 6, 9, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2024, 6, 15, 0, 0, 0)),
            Some(calc_datetime(2024, 6, 16, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2024, 6, 22, 0, 0, 0)),
            Some(calc_datetime(2024, 6, 23, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2024, 6, 29, 0, 0, 0)),
            Some(calc_datetime(2024, 6, 30, 0, 0, 0)),
        ),
    ];
    assert_eq!(weekends, actual);
}
#[test]
fn test_each_weekend_first_sat_is_none() {
    let date = calc_datetime(2023, 10, 1, 0, 0, 0);
    let weekends = date.each_weekend();
    let actual = vec![
        (None, Some(calc_datetime(2023, 10, 1, 0, 0, 0))),
        (
            Some(calc_datetime(2023, 10, 7, 0, 0, 0)),
            Some(calc_datetime(2023, 10, 8, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2023, 10, 14, 0, 0, 0)),
            Some(calc_datetime(2023, 10, 15, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2023, 10, 21, 0, 0, 0)),
            Some(calc_datetime(2023, 10, 22, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2023, 10, 28, 0, 0, 0)),
            Some(calc_datetime(2023, 10, 29, 0, 0, 0)),
        ),
    ];
    assert_eq!(weekends, actual);
}
#[test]
fn test_each_weekend_last_none_is_none() {
    let date = calc_datetime(2023, 9, 1, 0, 0, 0);
    let weekends = date.each_weekend();
    let actual = vec![
        (
            Some(calc_datetime(2023, 9, 2, 0, 0, 0)),
            Some(calc_datetime(2023, 9, 3, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2023, 9, 9, 0, 0, 0)),
            Some(calc_datetime(2023, 9, 10, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2023, 9, 16, 0, 0, 0)),
            Some(calc_datetime(2023, 9, 17, 0, 0, 0)),
        ),
        (
            Some(calc_datetime(2023, 9, 23, 0, 0, 0)),
            Some(calc_datetime(2023, 9, 24, 0, 0, 0)),
        ),
        (Some(calc_datetime(2023, 9, 30, 0, 0, 0)), None),
    ];
    assert_eq!(weekends, actual);
}
#[test]
fn test_days_of_month() {
    let list = [
        calc_datetime(2023, 1, 3, 0, 0, 0),
        calc_datetime(2023, 2, 3, 0, 0, 0),
        calc_datetime(2023, 3, 3, 0, 0, 0),
        calc_datetime(2023, 4, 3, 0, 0, 0),
        calc_datetime(2023, 5, 3, 0, 0, 0),
        calc_datetime(2023, 6, 3, 0, 0, 0),
        calc_datetime(2023, 7, 3, 0, 0, 0),
        calc_datetime(2023, 8, 3, 0, 0, 0),
        calc_datetime(2023, 9, 3, 0, 0, 0),
        calc_datetime(2023, 10, 3, 0, 0, 0),
        calc_datetime(2023, 11, 3, 0, 0, 0),
        calc_datetime(2023, 12, 3, 0, 0, 0),
    ];
    let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
    let actual = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    assert_eq!(result, actual);
}
#[test]
fn test_days_of_month_leap_year() {
    let list = [
        calc_datetime(2024, 1, 3, 0, 0, 0),
        calc_datetime(2024, 2, 3, 0, 0, 0),
        calc_datetime(2024, 3, 3, 0, 0, 0),
        calc_datetime(2024, 4, 3, 0, 0, 0),
        calc_datetime(2024, 5, 3, 0, 0, 0),
        calc_datetime(2024, 6, 3, 0, 0, 0),
        calc_datetime(2024, 7, 3, 0, 0, 0),
        calc_datetime(2024, 8, 3, 0, 0, 0),
        calc_datetime(2024, 9, 3, 0, 0, 0),
        calc_datetime(2024, 10, 3, 0, 0, 0),
        calc_datetime(2024, 11, 3, 0, 0, 0),
        calc_datetime(2024, 12, 3, 0, 0, 0),
    ];
    let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
    let actual = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    assert_eq!(result, actual);
}
#[test]
fn test_is_first_of_month_true() {
    let date = calc_datetime(2023, 1, 1, 0, 0, 0);
    assert!(date.is_first_day_of_month());
}
#[test]
fn test_is_first_of_month_false() {
    let date = calc_datetime(2023, 1, 2, 0, 0, 0);
    assert!(!date.is_first_day_of_month());
}
#[test]
fn test_is_last_of_month_true() {
    let list = [
        calc_datetime(2023, 1, 31, 0, 0, 0),
        calc_datetime(2023, 2, 28, 0, 0, 0),
        calc_datetime(2023, 3, 31, 0, 0, 0),
        calc_datetime(2023, 4, 30, 0, 0, 0),
        calc_datetime(2023, 5, 31, 0, 0, 0),
        calc_datetime(2023, 6, 30, 0, 0, 0),
        calc_datetime(2023, 7, 31, 0, 0, 0),
        calc_datetime(2023, 8, 31, 0, 0, 0),
        calc_datetime(2023, 9, 30, 0, 0, 0),
        calc_datetime(2023, 10, 31, 0, 0, 0),
        calc_datetime(2023, 11, 30, 0, 0, 0),
        calc_datetime(2023, 12, 31, 0, 0, 0),
    ];
    let result = list.iter().all(|v| v.is_last_day_of_month());
    assert!(result);
}
#[test]
fn test_is_last_of_month_false() {
    let date = calc_datetime(2023, 1, 30, 0, 0, 0);
    assert!(!date.is_last_day_of_month());
}
#[test]
fn test_sub_months() {
    let date = calc_datetime(2023, 1, 1, 0, 0, 0);
    let actual = date.sub_months(1);
    let expected = calc_datetime(2022, 12, 1, 0, 0, 0);
    assert_eq!(actual, expected);
}
#[test]
fn test_sub_months_opt() {
    let date = calc_datetime(2023, 1, 1, 0, 0, 0);
    let actual = date.sub_months_opt(1);
    let expected = Some(calc_datetime(2022, 12, 1, 0, 0, 0));
    assert_eq!(actual, expected);
}
#[test]
fn test_last_day_of_month() {
    let date = calc_datetime(2023, 1, 1, 12, 0, 0);
    let actual = date.last_day_of_month();
    let expected = calc_datetime(2023, 1, 31, 0, 0, 0);
    assert_eq!(actual, expected);
}
