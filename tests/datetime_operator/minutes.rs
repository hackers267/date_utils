#[cfg(feature = "minute")]
use date_utils::MinuteHelper;
use super::*;
#[test]
#[cfg(feature = "minute")]
fn test_begin_of_minute() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.begin_of_minute();
    let begin = calc_datetime(2008, 8, 8, 8, 8, 0);
    assert_eq!(begin, result);
}
#[test]
#[cfg(feature = "minute")]
fn test_end_of_minute() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.end_of_minute();
    let end = calc_datetime(2008, 8, 8, 8, 8, 59);
    assert_eq!(result, end);
}
#[test]
#[cfg(feature = "minute")]
fn test_same_minute_true() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(2000, 1, 1, 0, 0, 1);
    assert!(one.is_same_minute(&other));
}

#[test]
#[cfg(feature = "minute")]
fn test_same_minute_false() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(2000, 1, 1, 0, 1, 0);
    assert!(!one.is_same_minute(&other));
}

#[test]
#[cfg(feature = "minute")]
fn test_add_minutes() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let actual = calc_datetime(2000, 1, 1, 0, 30, 0);
    let result = one.add_minutes(30);
    assert_eq!(result, actual)
}

#[test]
#[cfg(feature = "minute")]
fn test_add_minutes_opt() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(2000, 1, 1, 0, 30, 0);
    let result = one.add_minutes_opt(30);
    assert_eq!(result, Some(other));
}

#[test]
#[cfg(feature = "minute")]
fn test_diff_minutes() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(2000, 1, 1, 0, 30, 0);
    let diff = other.diff_minutes(&one);
    assert_eq!(diff, 30);
}

#[test]
#[cfg(feature = "minute")]
fn test_sub_minutes() {
    let one = calc_datetime(2000, 1, 1, 12, 0, 0);
    let actual = calc_datetime(2000, 1, 1, 11, 30, 00);
    let result = one.sub_minutes(30);
    assert_eq!(actual, result);
}

#[test]
#[cfg(feature = "minute")]
fn test_sub_minutes_opt() {
    let one = calc_datetime(2000, 1, 1, 12, 0, 0);
    let other = calc_datetime(2000, 1, 1, 11, 30, 00);
    let result = one.sub_minutes_opt(30);
    assert_eq!(result, Some(other));
}
