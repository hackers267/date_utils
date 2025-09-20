#[cfg(feature = "second")]
use date_utils::SecondHelper;
use chrono::NaiveDate;
use chrono::NaiveTime;
use super::calc_datetime;

#[test]
#[cfg(feature = "second")]
fn test_second_add() {
    let date_time = calc_datetime(2000, 1, 1, 0, 0, 0);
    let result = date_time.add_seconds(32);
    let actual = calc_datetime(2000, 1, 1, 0, 0, 32);
    assert_eq!(result, actual);
}
#[test]
#[cfg(feature = "second")]
fn test_second_add_opt() {
    let date_time = calc_datetime(2000, 1, 1, 0, 0, 0);
    let result = date_time.add_seconds_opt(32);
    let actual = calc_datetime(2000, 1, 1, 0, 0, 32);
    assert_eq!(result, Some(actual));
}

#[test]
#[cfg(feature = "second")]
fn test_second_sub() {
    let date_time = calc_datetime(2000, 1, 1, 0, 0, 0);
    let result = date_time.sub_seconds(20);
    let actual = calc_datetime(1999, 12, 31, 23, 59, 40);
    assert_eq!(result, actual);
}

#[test]
#[cfg(feature = "second")]
fn test_second_sub_opt() {
    let date_time = calc_datetime(2000, 1, 1, 0, 0, 0);
    let result = date_time.sub_seconds_opt(20);
    let actual = calc_datetime(1999, 12, 31, 23, 59, 40);
    assert_eq!(result, Some(actual));
}

#[test]
#[cfg(feature = "second")]
fn test_difference_second() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(1999, 12, 31, 23, 59, 31);
    let result = one.diff_seconds(&other);
    let actual = 29;
    assert_eq!(result, actual)
}

#[test]
#[cfg(feature = "second")]
fn test_begin_of_second() {
    let time = NaiveTime::from_hms_micro_opt(0, 0, 0, 300);
    let date = NaiveDate::from_ymd_opt(2000, 1, 1);
    let date_time = date.and_then(|date| time.map(|time| date.and_time(time)));
    let result = date_time.map(|date_time| date_time.begin_of_second());
    let actual =
        NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_micro_opt(0, 0, 0, 0));
    assert_eq!(result, actual)
}

#[test]
#[cfg(feature = "second")]
fn test_is_same_second() {
    let date = NaiveDate::from_ymd_opt(2000, 1, 1);
    let time0 = NaiveTime::from_hms_micro_opt(0, 0, 0, 0);
    let time1 = NaiveTime::from_hms_micro_opt(0, 0, 0, 999);
    let old = date.and_then(|date| time0.map(|time| date.and_time(time)));
    let new = date.and_then(|date| time1.map(|time| date.and_time(time)));
    let result = old.and_then(|old| new.map(|new| old.is_same_second(&new)));
    assert!(result.is_some_and(|result| result));
}

#[test]
#[cfg(feature = "second")]
fn test_end_of_second() {
    let time = NaiveTime::from_hms_micro_opt(0, 0, 0, 123);
    let date = NaiveDate::from_ymd_opt(2000, 1, 1);
    let date_time = date.and_then(|date| time.map(|time| date.and_time(time)));
    let result = date_time.map(|date_time| date_time.end_of_second());
    let actual =
        NaiveDate::from_ymd_opt(2000, 1, 1).and_then(|date| date.and_hms_micro_opt(0, 0, 0, 999));
    assert_eq!(result, actual)
}
