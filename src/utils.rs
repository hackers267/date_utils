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

pub fn second2minute(second: u32) -> u32 {
    second % 60
}

pub fn second2hour(second: u32) -> u32 {
    second % 60 % 60
}

pub fn second2day(second: u32) -> u32 {
    second % 60 % 60 % 24
}

pub fn minute2hour(minute: u32) -> u32 {
    minute % 60
}

pub fn minute2day(minute: u32) -> u32 {
    minute % 60 % 24
}

pub fn hour2day(hour: u32) -> u32 {
    hour % 24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2003));
        assert!(is_leap_year(2004));
    }

    #[test]
    fn test_second2minute() {
        let second = 61;
        let result = second2minute(second);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_second2hour() {
        let second = 3601;
        let result = second2hour(second);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_second2day() {
        let second = 86401;
        let result = second2day(second);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_minute2hour() {
        let minute = 61;
        let result = minute2hour(minute);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_minute2day() {
        let minute = 1441;
        let result = minute2day(minute);
        assert_eq!(result, 1)
    }

    #[test]
    fn test_hour2day() {
        let hour = 25;
        let result = hour2day(hour);
        assert_eq!(result, 1);
    }
}
