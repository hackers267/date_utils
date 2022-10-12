use chrono::{TimeZone, Utc};
use date_utils::DateTimeOperator;

#[test]
fn test_begin_of_day() {
    let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let begin = datetime.begin_of_day();
    let result = Utc.ymd(2008, 8, 8).and_hms(0, 0, 0);
    assert_eq!(begin, result)
}
#[test]
fn test_begin_of_hour() {
    let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
    let result = datetime.begin_of_hour();
    let begin = Utc.ymd(2008, 8, 8).and_hms(8, 0, 0);
    assert_eq!(begin, result);
}
