use chrono::{NaiveDate, NaiveDateTime};
use date_utils::{DayHelper, HourHelper, MinuteHelper, YearHelper};

fn calc_datetime(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap()
}

#[test]
fn test_begin_of_day() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let begin = datetime.begin_of_day();
    let result = calc_datetime(2008, 8, 8, 0, 0, 0);
    assert_eq!(begin, result)
}
#[test]
fn test_begin_of_hour() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.begin_of_hour();
    let begin = calc_datetime(2008, 8, 8, 8, 0, 0);
    assert_eq!(begin, result);
}
#[test]
fn test_begin_of_minute() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.begin_of_minute();
    let begin = calc_datetime(2008, 8, 8, 8, 8, 0);
    assert_eq!(begin, result);
}

#[test]
fn test_end_of_day() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.end_of_day();
    let end = calc_datetime(2008, 8, 8, 23, 59, 59);
    assert_eq!(result, end)
}

#[test]
fn test_end_of_hour() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.end_of_hour();
    let end = calc_datetime(2008, 8, 8, 8, 59, 59);
    assert_eq!(result, end);
}

#[test]
fn test_end_of_minute() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.end_of_minute();
    let end = calc_datetime(2008, 8, 8, 8, 8, 59);
    assert_eq!(result, end);
}

#[test]
fn test_is_leap_year_true() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.is_leap_year();
    assert!(result)
}
#[test]
fn test_is_leap_year_false() {
    let datetime = calc_datetime(1990, 8, 8, 8, 8, 8);
    let result = datetime.is_leap_year();
    assert!(!result)
}
