pub(crate) fn is_leap_year(year: i32) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, _) => true,
        (_, 0, _) => false,
        (_, _, 0) => true,
        _ => false,
    }
}
const DAY_30: [u32; 4] = [4, 6, 9, 11];
const DAY_31: [u32; 7] = [1, 3, 5, 7, 8, 10, 12];

pub(crate) enum MonthType {
    Day30,
    Day31,
    Other(bool),
}

pub(crate) fn month_type(month: u32, year: i32) -> MonthType {
    if DAY_30.contains(&month) {
        MonthType::Day30
    } else if DAY_31.contains(&month) {
        MonthType::Day31
    } else {
        MonthType::Other(is_leap_year(year))
    }
}
