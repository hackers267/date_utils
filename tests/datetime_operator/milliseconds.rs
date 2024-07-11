#[cfg(test)]
#[cfg(feature = "millisecond")]
use super::*;
use chrono::Duration;
use date_utils::MillisecondHelper;

#[test]
fn test_add_millisecond() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 0);
    let actual = date.add_millisecond(1000);
    assert_eq!(actual, calc_datetime(2023, 6, 11, 0, 0, 1));
}
#[test]
fn test_add_millisecond_opt() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 0);
    let actual = date.add_millisecond_opt(1000);
    assert_eq!(actual, Some(calc_datetime(2023, 6, 11, 0, 0, 1)));
}
#[test]
fn test_sub_millisecond_opt() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 1);
    let actual = date.sub_millisecond_opt(1000);
    assert_eq!(actual, Some(calc_datetime(2023, 6, 11, 0, 0, 0)));
}
#[test]
fn test_sub_millisecond() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 1);
    let actual = date.sub_millisecond(1000);
    assert_eq!(actual, calc_datetime(2023, 6, 11, 0, 0, 0));
}
#[test]
fn test_diff_millisecond() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 1);
    let other = calc_datetime(2023, 6, 11, 0, 0, 0);
    let actual = date.diff_milliseconds(&other);
    assert_eq!(actual, 1000);
}
#[test]
fn test_millisecond() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 1)
        .checked_add_signed(Duration::milliseconds(100))
        .unwrap();
    let actual = date.millisecond();
    assert_eq!(actual, 100);
}
#[test]
fn test_set_millisecond() {
    let date = calc_datetime(2023, 6, 11, 0, 0, 1);
    let result = date.set_millisecond(100);
    let actual = result.millisecond();
    assert_eq!(actual, 100);
}
