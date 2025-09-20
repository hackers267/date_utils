#[cfg(feature = "year")]
use date_utils::YearHelper;
use super::calc_date;

#[test]
#[cfg(feature = "year")]
fn test_years_since_date() {
    let date = calc_date(2008, 8, 8);
    let other = calc_date(2018, 8, 8);
    let diff = other.years_since(date);
    assert_eq!(diff, Some(10))
}
#[test]
#[cfg(feature = "year")]
fn test_is_same_year_date_true() {
    let date = calc_date(2008, 8, 8);
    let other = calc_date(2008, 1, 1);
    assert!(date.is_same_year(&other));
}
#[test]
#[cfg(feature = "year")]
fn test_is_same_year_date_false() {
    let date = calc_date(2008, 8, 8);
    let other = calc_date(2007, 12, 31);
    assert!(!date.is_same_year(&other))
}
#[test]
#[cfg(feature = "year")]
fn test_begin_of_year_date_true() {
    let date = calc_date(2008, 8, 8);
    let result = date.begin_of_year();
    let begin = calc_date(2008, 1, 1);
    assert_eq!(begin, result);
}
#[test]
#[cfg(feature = "year")]
fn test_is_leap_year() {
    let date = calc_date(1900, 1, 1);
    assert!(!date.is_leap_year());
}
