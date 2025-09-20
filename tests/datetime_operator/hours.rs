#[cfg(feature = "hour")]
use date_utils::HourHelper;

#[test]
#[cfg(feature = "hour")]
fn test_begin_of_hour() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.begin_of_year();
    let begin = calc_datetime(2008, 8, 8, 8, 0, 0);
    assert_eq!(begin, result);
}
#[test]
#[cfg(feature = "hour")]
fn test_end_of_hour() {
    let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
    let result = datetime.end_of_hour();
    let end = calc_datetime(2008, 8, 8, 8, 59, 59);
    assert_eq!(result, end);
}
#[test]
#[cfg(feature = "hour")]
fn test_is_same_hour_true() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(2000, 1, 1, 0, 59, 59);
    assert!(one.is_same_hour(&other))
}

#[test]
#[cfg(feature = "hour")]
fn test_is_same_hour_false() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let other = calc_datetime(2000, 1, 1, 1, 0, 0);
    assert!(!one.is_same_hour(&other))
}
#[test]
#[cfg(feature = "hour")]
fn test_add_hours() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let actual = calc_datetime(2000, 1, 1, 6, 0, 0);
    let result = one.add_hours(6);
    assert_eq!(result, actual);
}

#[test]
#[cfg(feature = "hour")]
fn test_add_hours_opt() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let actual = calc_datetime(2000, 1, 1, 6, 0, 0);
    let result = one.add_hours_opt(6);
    assert_eq!(result, Some(actual));
}

#[test]
#[cfg(feature = "hour")]
fn test_diff_hours() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let actual = calc_datetime(2000, 1, 1, 6, 0, 0);
    let diff = actual.diff_hours(&one);
    assert_eq!(diff, 6);
}
#[test]
#[cfg(feature = "hour")]
fn test_diff_hours_other() {
    let one = calc_datetime(2000, 1, 1, 0, 0, 0);
    let actual = calc_datetime(2000, 1, 1, 6, 0, 0);
    let diff = one.diff_hours(&actual);
    assert_eq!(diff, -6);
}

#[test]
#[cfg(feature = "hour")]
fn test_sub_hours() {
    let actual = calc_datetime(2000, 1, 1, 0, 0, 0);
    let one = calc_datetime(2000, 1, 1, 6, 0, 0);
    let result = one.sub_hours(6);
    assert_eq!(result, actual);
}
#[test]
#[cfg(feature = "hour")]
fn test_sub_hours_opt() {
    let actual = calc_datetime(2000, 1, 1, 0, 0, 0);
    let one = calc_datetime(2000, 1, 1, 6, 0, 0);
    let result = one.sub_hours_opt(6);
    assert_eq!(result, Some(actual));
}
