use chrono::{NaiveDate, NaiveDateTime};

#[cfg(test)]
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

#[cfg(feature = "day")]
#[cfg(test)]
mod days {
    use super::*;
    use date_utils::DayHelper;
    #[test]
    fn test_begin_of_day() {
        let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
        let begin = datetime.begin_of_day();
        let result = calc_datetime(2008, 8, 8, 0, 0, 0);
        assert_eq!(begin, result)
    }

    #[test]
    fn test_end_of_day() {
        let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
        let result = datetime.end_of_day();
        let end = calc_datetime(2008, 8, 8, 23, 59, 59);
        assert_eq!(result, end)
    }
}
#[cfg(feature = "hour")]
#[cfg(test)]
mod hours {
    use super::*;
    use date_utils::HourHelper;
    #[test]
    fn test_begin_of_hour() {
        let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
        let result = datetime.begin_of_hour();
        let begin = calc_datetime(2008, 8, 8, 8, 0, 0);
        assert_eq!(begin, result);
    }
    #[test]
    fn test_end_of_hour() {
        let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
        let result = datetime.end_of_hour();
        let end = calc_datetime(2008, 8, 8, 8, 59, 59);
        assert_eq!(result, end);
    }
}
#[cfg(feature = "minute")]
#[cfg(test)]
mod minutes {
    use super::*;
    use date_utils::MinuteHelper;
    #[test]
    fn test_begin_of_minute() {
        let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
        let result = datetime.begin_of_minute();
        let begin = calc_datetime(2008, 8, 8, 8, 8, 0);
        assert_eq!(begin, result);
    }
    #[test]
    fn test_end_of_minute() {
        let datetime = calc_datetime(2008, 8, 8, 8, 8, 8);
        let result = datetime.end_of_minute();
        let end = calc_datetime(2008, 8, 8, 8, 8, 59);
        assert_eq!(result, end);
    }
}

#[cfg(feature = "year")]
#[cfg(test)]
mod years {
    use super::*;
    use date_utils::YearHelper;

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
}

#[cfg(feature = "month")]
#[cfg(test)]
mod months {
    use super::*;
    use date_utils::MonthHelper;

    #[test]
    fn test_datetime_add_months() {
        let one = calc_datetime(2023, 4, 8, 11, 11, 11);
        let result = one.add_months(7);
        let actual = calc_datetime(2023, 11, 8, 11, 11, 11);
        assert_eq!(result, actual);
    }
    #[test]
    fn test_datetime_diff_in_months() {
        let one = calc_datetime(2023, 4, 8, 11, 11, 11);
        let other = calc_datetime(2022, 1, 9, 11, 11, 11);
        let result = one.diff_month(&other);
        assert_eq!(result, 14);
    }
}
