use chrono::{TimeZone, Utc};
use date_utils::DateOperator;
#[test]
fn test_before_date() {
    let now = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2018, 8, 8);
    assert!(now.before(&other))
}
#[test]
fn test_before_datetime() {
    let now = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let other = Utc.ymd(2018, 8, 8).and_hms(8, 8, 8);
    assert!(now.before(&other))
}
#[test]
fn test_after_date() {
    let now = Utc.ymd(2018, 8, 8);
    let other = Utc.ymd(2008, 8, 8);
    assert!(now.after(&other))
}
#[test]
fn test_after_datetime() {
    let now = Utc.ymd(2018, 8, 8).and_hms(8, 8, 8);
    let other = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    assert!(now.after(&other))
}
#[test]
fn test_years_since_date() {
    let date = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2018, 8, 8);
    let diff = other.years_since(date);
    assert_eq!(diff, Some(10))
}
#[test]
fn test_is_same_day_date_true() {
    let date = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2008, 8, 8);
    assert!(date.is_same_day(&other));
}
#[test]
fn test_is_same_day_date_false() {
    let date = Utc.ymd(2008, 8, 9);
    let other = Utc.ymd(2008, 8, 8);
    assert!(!date.is_same_day(&other));
}
#[test]
fn test_is_same_day_datetime_true() {
    let date = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let other = Utc.ymd(2008, 8, 8).and_hms(23, 59, 59);
    assert!(date.is_same_day(&other));
}
#[test]
fn test_is_same_day_datetime_false() {
    let date = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let other = Utc.ymd(2008, 8, 7).and_hms(8, 8, 8);
    assert!(!date.is_same_day(&other));
}
#[test]
fn test_is_same_month_date_true() {
    let date = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2008, 8, 12);
    assert!(date.is_same_month(&other))
}
#[test]
fn test_is_same_month_date_false() {
    let date = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2008, 1, 1);
    assert!(!date.is_same_month(&other));
}
#[test]
fn test_is_same_month_datetime_true() {
    let date = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let other = Utc.ymd(2008, 8, 31).and_hms(8, 8, 8);
    assert!(date.is_same_month(&other));
}
#[test]
fn test_is_same_month_datetime_false() {
    let date = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let other = Utc.ymd(2008, 1, 1).and_hms(8, 8, 8);
    assert!(!date.is_same_month(&other));
}
#[test]
fn test_is_same_year_date_true() {
    let date = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2008, 1, 1);
    assert!(date.is_same_year(&other));
}
#[test]
fn test_is_same_year_date_false() {
    let date = Utc.ymd(2008, 8, 8);
    let other = Utc.ymd(2007, 12, 31);
    assert!(!date.is_same_year(&other))
}
#[test]
fn test_is_same_year_datetime_true() {
    let date = Utc.ymd(2008, 1, 1).and_hms(0, 0, 0);
    let other = Utc.ymd(2008, 12, 31).and_hms(23, 59, 59);
    assert!(date.is_same_year(&other));
}
#[test]
fn test_is_same_year_datetime_false() {
    let date = Utc.ymd(2008, 1, 1).and_hms(0, 0, 0);
    let other = Utc.ymd(2007, 12, 31).and_hms(23, 59, 59);
    assert!(!date.is_same_year(&other));
}
#[test]
fn test_begin_of_year_date_true() {
    let date = Utc.ymd(2008, 8, 8);
    let result = date.begin_of_year();
    let begin = Utc.ymd(2008, 1, 1);
    assert_eq!(begin, result);
}
#[test]
fn test_begin_of_year_datetime_true() {
    let datetime = Utc.ymd(2008, 12, 31).and_hms(8, 8, 8);
    let result = datetime.begin_of_year();
    let begin = Utc.ymd(2008, 1, 1).and_hms(0, 0, 0);
    assert_eq!(begin, result)
}
