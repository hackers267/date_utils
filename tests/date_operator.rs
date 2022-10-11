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
