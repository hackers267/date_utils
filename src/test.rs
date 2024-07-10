use chrono::{NaiveDate, NaiveDateTime};
#[cfg(test)]
pub(crate) fn get_time_opt(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_opt(hour, minute, second))
}
#[cfg(test)]
pub(crate) fn get_time(y: i32, m: u32, d: u32, h: u32, minute: u32, second: u32) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(y, m, d)
        .and_then(|date| date.and_hms_opt(h, minute, second))
        .unwrap()
}
