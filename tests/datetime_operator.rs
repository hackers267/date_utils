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
        let result = one.diff_months(&other);
        assert_eq!(result, 14);
    }
    #[test]
    fn test_each_weekend() {
        let date = calc_datetime(2024, 6, 1, 0, 0, 0);
        let weekends = date.each_weekend();
        let actual = vec![
            (
                Some(calc_datetime(2024, 6, 1, 0, 0, 0)),
                Some(calc_datetime(2024, 6, 2, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2024, 6, 8, 0, 0, 0)),
                Some(calc_datetime(2024, 6, 9, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2024, 6, 15, 0, 0, 0)),
                Some(calc_datetime(2024, 6, 16, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2024, 6, 22, 0, 0, 0)),
                Some(calc_datetime(2024, 6, 23, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2024, 6, 29, 0, 0, 0)),
                Some(calc_datetime(2024, 6, 30, 0, 0, 0)),
            ),
        ];
        assert_eq!(weekends, actual);
    }
    #[test]
    fn test_each_weekend_first_sat_is_none() {
        let date = calc_datetime(2023, 10, 1, 0, 0, 0);
        let weekends = date.each_weekend();
        let actual = vec![
            (None, Some(calc_datetime(2023, 10, 1, 0, 0, 0))),
            (
                Some(calc_datetime(2023, 10, 7, 0, 0, 0)),
                Some(calc_datetime(2023, 10, 8, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2023, 10, 14, 0, 0, 0)),
                Some(calc_datetime(2023, 10, 15, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2023, 10, 21, 0, 0, 0)),
                Some(calc_datetime(2023, 10, 22, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2023, 10, 28, 0, 0, 0)),
                Some(calc_datetime(2023, 10, 29, 0, 0, 0)),
            ),
        ];
        assert_eq!(weekends, actual);
    }
    #[test]
    fn test_each_weekend_last_none_is_none() {
        let date = calc_datetime(2023, 9, 1, 0, 0, 0);
        let weekends = date.each_weekend();
        let actual = vec![
            (
                Some(calc_datetime(2023, 9, 2, 0, 0, 0)),
                Some(calc_datetime(2023, 9, 3, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2023, 9, 9, 0, 0, 0)),
                Some(calc_datetime(2023, 9, 10, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2023, 9, 16, 0, 0, 0)),
                Some(calc_datetime(2023, 9, 17, 0, 0, 0)),
            ),
            (
                Some(calc_datetime(2023, 9, 23, 0, 0, 0)),
                Some(calc_datetime(2023, 9, 24, 0, 0, 0)),
            ),
            (Some(calc_datetime(2023, 9, 30, 0, 0, 0)), None),
        ];
        assert_eq!(weekends, actual);
    }
    #[test]
    fn test_days_of_month() {
        let list = [
            calc_datetime(2023, 1, 3, 0, 0, 0),
            calc_datetime(2023, 2, 3, 0, 0, 0),
            calc_datetime(2023, 3, 3, 0, 0, 0),
            calc_datetime(2023, 4, 3, 0, 0, 0),
            calc_datetime(2023, 5, 3, 0, 0, 0),
            calc_datetime(2023, 6, 3, 0, 0, 0),
            calc_datetime(2023, 7, 3, 0, 0, 0),
            calc_datetime(2023, 8, 3, 0, 0, 0),
            calc_datetime(2023, 9, 3, 0, 0, 0),
            calc_datetime(2023, 10, 3, 0, 0, 0),
            calc_datetime(2023, 11, 3, 0, 0, 0),
            calc_datetime(2023, 12, 3, 0, 0, 0),
        ];
        let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
        let actual = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        assert_eq!(result, actual);
    }
    #[test]
    fn test_days_of_month_leap_year() {
        let list = [
            calc_datetime(2024, 1, 3, 0, 0, 0),
            calc_datetime(2024, 2, 3, 0, 0, 0),
            calc_datetime(2024, 3, 3, 0, 0, 0),
            calc_datetime(2024, 4, 3, 0, 0, 0),
            calc_datetime(2024, 5, 3, 0, 0, 0),
            calc_datetime(2024, 6, 3, 0, 0, 0),
            calc_datetime(2024, 7, 3, 0, 0, 0),
            calc_datetime(2024, 8, 3, 0, 0, 0),
            calc_datetime(2024, 9, 3, 0, 0, 0),
            calc_datetime(2024, 10, 3, 0, 0, 0),
            calc_datetime(2024, 11, 3, 0, 0, 0),
            calc_datetime(2024, 12, 3, 0, 0, 0),
        ];
        let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
        let actual = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        assert_eq!(result, actual);
    }
    #[test]
    fn test_is_first_of_month_true() {
        let date = calc_datetime(2023, 1, 1, 0, 0, 0);
        assert!(date.is_first_day_of_month());
    }
    #[test]
    fn test_is_first_of_month_false() {
        let date = calc_datetime(2023, 1, 2, 0, 0, 0);
        assert!(!date.is_first_day_of_month());
    }
    #[test]
    fn test_is_last_of_month_true() {
        let list = [
            calc_datetime(2023, 1, 31, 0, 0, 0),
            calc_datetime(2023, 2, 28, 0, 0, 0),
            calc_datetime(2023, 3, 31, 0, 0, 0),
            calc_datetime(2023, 4, 30, 0, 0, 0),
            calc_datetime(2023, 5, 31, 0, 0, 0),
            calc_datetime(2023, 6, 30, 0, 0, 0),
            calc_datetime(2023, 7, 31, 0, 0, 0),
            calc_datetime(2023, 8, 31, 0, 0, 0),
            calc_datetime(2023, 9, 30, 0, 0, 0),
            calc_datetime(2023, 10, 31, 0, 0, 0),
            calc_datetime(2023, 11, 30, 0, 0, 0),
            calc_datetime(2023, 12, 31, 0, 0, 0),
        ];
        let result = list.iter().all(|v| v.is_last_day_of_month());
        assert!(result);
    }
    #[test]
    fn test_is_last_of_month_false() {
        let date = calc_datetime(2023, 1, 30, 0, 0, 0);
        assert!(!date.is_last_day_of_month());
    }
    #[test]
    fn test_sub_months() {
        let date = calc_datetime(2023, 1, 1, 0, 0, 0);
        let actual = date.sub_months(1);
        let expected = calc_datetime(2022, 12, 1, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_last_day_of_month() {
        let date = calc_datetime(2023, 1, 1, 12, 0, 0);
        let actual = date.last_day_of_month();
        let expected = calc_datetime(2023, 1, 31, 0, 0, 0);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
#[cfg(feature = "week")]
mod weeks {
    use super::*;
    use chrono::Weekday;
    use date_utils::{Range, WeekHelper};
    use std::iter::zip;

    #[test]
    fn test_is_monday() {
        let date = calc_datetime(2023, 10, 2, 0, 0, 0);
        assert!(date.is_monday());
    }
    #[test]
    fn test_is_tuesday() {
        let date = calc_datetime(2023, 10, 3, 0, 0, 0);
        assert!(date.is_tuesday());
    }
    #[test]
    fn test_is_wednesday() {
        let date = calc_datetime(2023, 10, 4, 0, 0, 0);
        assert!(date.is_wednesday());
    }
    #[test]
    fn test_is_thursday() {
        let date = calc_datetime(2023, 10, 5, 0, 0, 0);
        assert!(date.is_thursday());
    }
    #[test]
    fn test_is_friday() {
        let date = calc_datetime(2023, 10, 6, 0, 0, 0);
        assert!(date.is_friday());
    }
    #[test]
    fn test_is_saturday() {
        let date = calc_datetime(2023, 10, 7, 0, 0, 0);
        assert!(date.is_saturday());
    }
    #[test]
    fn test_is_sunday() {
        let date = calc_datetime(2023, 10, 8, 0, 0, 0);
        assert!(date.is_sunday());
    }
    #[test]
    fn test_is_weekend() {
        let sat = calc_datetime(2023, 10, 7, 0, 0, 0);
        let sun = calc_datetime(2023, 10, 8, 0, 0, 0);
        let result = [sat, sun].iter().all(|date| date.is_weekend());
        assert!(result);
    }
    #[test]
    fn test_is_workday() {
        let mon = calc_datetime(2023, 10, 2, 0, 0, 0);
        let tue = calc_datetime(2023, 10, 3, 0, 0, 0);
        let wed = calc_datetime(2023, 10, 4, 0, 0, 0);
        let thu = calc_datetime(2023, 10, 5, 0, 0, 0);
        let fri = calc_datetime(2023, 10, 6, 0, 0, 0);
        let result = [mon, tue, wed, thu, fri]
            .iter()
            .all(|date| date.is_workday());
        assert!(result);
    }
    #[test]
    fn test_add_week() {
        let date = calc_datetime(2023, 10, 2, 0, 0, 0);
        let actual = date.add_week(1);
        let expected = calc_datetime(2023, 10, 9, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_end_of_week() {
        let date = calc_datetime(2023, 10, 2, 0, 0, 0);
        let actual = date.end_of_week();
        let expected = calc_datetime(2023, 10, 8, 23, 59, 59);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_end_of_week0() {
        let date = calc_datetime(2023, 10, 2, 0, 0, 0);
        let actual = date.end_of_week0();
        let expected = calc_datetime(2023, 10, 7, 23, 59, 59);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week();
        let expected = calc_datetime(2023, 10, 2, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week0() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week0();
        let expected = calc_datetime(2023, 10, 1, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_sat() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Sat);
        let expected = calc_datetime(2023, 9, 30, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_sun() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Sun);
        let expected = calc_datetime(2023, 10, 1, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_mon() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Mon);
        let expected = calc_datetime(2023, 10, 2, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_tue() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Tue);
        let expected = calc_datetime(2023, 9, 26, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_wed() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Wed);
        let expected = calc_datetime(2023, 9, 27, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_thu() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Thu);
        let expected = calc_datetime(2023, 9, 28, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_begin_of_week_fri() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.begin_of_week_with(Weekday::Fri);
        let expected = calc_datetime(2023, 9, 29, 0, 0, 0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_sub_week() {
        let date = calc_datetime(2023, 10, 2, 12, 12, 12);
        let actual = date.sub_week(1);
        let expected = calc_datetime(2023, 9, 25, 12, 12, 12);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_week_of_month() {
        let date = calc_datetime(2017, 11, 9, 12, 12, 12);
        let actual = date.week_of_month();
        let expected = 2;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_week_of_month0() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(3).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first.iter().all(|d| d.week_of_month0() == 1);
        let rest_week = rest
            .iter()
            .enumerate()
            .all(|(i, list)| list.iter().all(|d| d.week_of_month0() == i as u8 + 2));
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_sun() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(3).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Sun) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Sun) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_mon() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(4).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Mon) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Mon) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_tue() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(5).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Tue) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Tue) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_wed() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(6).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Wed) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Wed) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_thu() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(7).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Thu) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Thu) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_fri() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(1).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Fri) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Fri) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_week_of_month_with_sat() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let mut list = date.range();
        let first = list.by_ref().take(2).collect::<Vec<_>>();
        let rest = list.collect::<Vec<_>>();
        let rest = rest.chunks(7).collect::<Vec<_>>();
        let is_first = first
            .iter()
            .all(|d| d.week_of_month_with(Weekday::Sat) == 1);
        let rest_week = rest.iter().enumerate().all(|(i, list)| {
            list.iter()
                .all(|d| d.week_of_month_with(Weekday::Sat) == i as u8 + 2)
        });
        assert!(is_first && rest_week);
    }
    #[test]
    fn test_weeks_of_month() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let weeks = date.weeks_of_month();
        assert_eq!(weeks, 5);
    }
    #[test]
    fn test_weeks_of_month0() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let weeks = date.weeks_of_month0();
        assert_eq!(weeks, 5);
    }
    #[test]
    fn test_weeks_of_month_with() {
        let date = calc_datetime(2023, 6, 3, 12, 12, 12);
        let weekdays = [
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];
        let weeks = weekdays
            .iter()
            .map(|w| date.weeks_of_month_with(*w))
            .collect::<Vec<_>>();
        assert_eq!(weeks, [5, 5, 5, 5, 5, 6, 5]);
    }
    #[test]
    fn test_is_same_week_true() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let other = calc_datetime(2023, 6, 10, 12, 12, 12);
        let is_same = date.is_same_week(&other);
        assert!(is_same);
    }
    #[test]
    fn test_is_same_week_false() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let other = calc_datetime(2023, 6, 12, 12, 12, 12);
        let is_same = date.is_same_week(&other);
        assert!(!is_same);
    }
    #[test]
    fn test_is_same_week0_true() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let other = calc_datetime(2023, 6, 4, 12, 12, 12);
        let is_same = date.is_same_week0(&other);
        assert!(is_same);
    }
    #[test]
    fn test_is_same_week0_false() {
        let date = calc_datetime(2023, 6, 10, 12, 12, 12);
        let other = calc_datetime(2023, 6, 11, 12, 12, 12);
        let is_same = date.is_same_week0(&other);
        assert!(!is_same);
    }
    #[test]
    fn test_is_same_week_with_true() {
        let weekdays = [
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];
        let one_days = (1..=7)
            .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
            .collect::<Vec<_>>();
        let next_days = (2..=8)
            .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
            .collect::<Vec<_>>();
        let result = zip(zip(one_days, next_days), weekdays)
            .any(|((d1, d2), weekday)| d1.is_same_week_with(&d2, weekday));
        assert!(result);
    }
    #[test]
    fn test_last_day_of_week() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let last_day = date.last_day_of_week();
        let actual = calc_datetime(2023, 6, 11, 0, 0, 0);
        assert_eq!(last_day, actual);
    }
    #[test]
    fn test_last_day_of_week0() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let last_day = date.last_day_of_week0();
        let actual = calc_datetime(2023, 6, 10, 0, 0, 0);
        assert_eq!(last_day, actual);
    }
    #[test]
    fn test_diff_calendar_with() {
        let weekdays = [
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];
        let one_days = (18..=24)
            .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
            .collect::<Vec<_>>();
        let next_days = (4..=10)
            .map(|d| calc_datetime(2023, 6, d, 12, 12, 12))
            .collect::<Vec<_>>();
        let result = zip(zip(one_days, next_days), weekdays)
            .map(|((d1, d2), weekday)| d1.diff_calendar_weeks_with(&d2, weekday))
            .collect::<Vec<_>>();
        assert_eq!(result, vec![2; 7]);
    }
    #[test]
    fn test_diff_weeks() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let other = calc_datetime(2023, 6, 12, 12, 12, 12);
        let diff = date.diff_weeks(&other);
        assert_eq!(diff, -1);
    }
    #[test]
    fn test_diff_weeks1() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let other = calc_datetime(2023, 6, 12, 12, 12, 11);
        let diff = date.diff_weeks(&other);
        assert_eq!(diff, 0);
    }
    #[test]
    fn test_next_day() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        let next_days = weekdays.map(|w| date.next_day(w)).to_vec();
        let actual = date.range().skip(11).take(7).collect::<Vec<_>>();
        assert_eq!(next_days, actual);
    }
    #[test]
    fn test_prev_day() {
        let date = calc_datetime(2023, 6, 15, 12, 12, 12);
        let weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        let prev_days = weekdays.map(|w| date.previous_day(w)).to_vec();
        let actual = date.range().skip(4).take(7).collect::<Vec<_>>();
        assert_eq!(prev_days, actual);
    }
    #[test]
    fn test_next_monday_and_so_on() {
        let date = calc_datetime(2023, 6, 5, 12, 12, 12);
        let next_monday = date.next_monday();
        let next_tuesday = date.next_tuesday();
        let next_wednesday = date.next_wednesday();
        let next_thursday = date.next_thursday();
        let next_friday = date.next_friday();
        let next_saturday = date.next_saturday();
        let next_sunday = date.next_sunday();
        assert_eq!(next_monday, calc_datetime(2023, 6, 12, 0, 0, 0));
        assert_eq!(next_tuesday, calc_datetime(2023, 6, 13, 0, 0, 0));
        assert_eq!(next_wednesday, calc_datetime(2023, 6, 14, 0, 0, 0));
        assert_eq!(next_thursday, calc_datetime(2023, 6, 15, 0, 0, 0));
        assert_eq!(next_friday, calc_datetime(2023, 6, 16, 0, 0, 0));
        assert_eq!(next_saturday, calc_datetime(2023, 6, 17, 0, 0, 0));
        assert_eq!(next_sunday, calc_datetime(2023, 6, 18, 0, 0, 0));
    }
    #[test]
    fn test_prev_monday_and_so_on() {
        let date = calc_datetime(2023, 6, 15, 12, 12, 12);
        let prev_monday = date.previous_monday();
        let prev_tuesday = date.previous_tuesday();
        let prev_wednesday = date.previous_wednesday();
        let prev_thursday = date.previous_thursday();
        let prev_friday = date.previous_friday();
        let prev_saturday = date.previous_saturday();
        let prev_sunday = date.previous_sunday();
        assert_eq!(prev_monday, calc_datetime(2023, 6, 5, 0, 0, 0));
        assert_eq!(prev_tuesday, calc_datetime(2023, 6, 6, 0, 0, 0));
        assert_eq!(prev_wednesday, calc_datetime(2023, 6, 7, 0, 0, 0));
        assert_eq!(prev_thursday, calc_datetime(2023, 6, 8, 0, 0, 0));
        assert_eq!(prev_friday, calc_datetime(2023, 6, 9, 0, 0, 0));
        assert_eq!(prev_saturday, calc_datetime(2023, 6, 10, 0, 0, 0));
        assert_eq!(prev_sunday, calc_datetime(2023, 6, 11, 0, 0, 0));
    }
}

#[cfg(test)]
#[cfg(feature = "millisecond")]
mod millisecond {
    use super::*;
    use chrono::Duration;
    use date_utils::MillisecondHelper;
    #[test]
    fn test_add_millisecond() {
        let date = calc_datetime(2023, 6, 11, 0, 0, 0);
        let actual = date.add_millisecond(1000);
        assert_eq!(actual, calc_datetime(2023, 6, 11, 0, 0, 1));
    }
    #[test]
    fn test_sub_millisecond() {
        let date = calc_datetime(2023, 6, 11, 0, 0, 1);
        let actual = date.sub_millisecond(1000);
        assert_eq!(actual, calc_datetime(2023, 6, 11, 0, 0, 0));
    }
    #[test]
    fn test_diff_millisecond() {
        let date = calc_datetime(2023, 6, 11, 0, 0, 1);
        let other = calc_datetime(2023, 6, 11, 0, 0, 0);
        let actual = date.diff_milliseconds(&other);
        assert_eq!(actual, 1000);
    }
    #[test]
    fn test_millisecond() {
        let date = calc_datetime(2023, 6, 11, 0, 0, 1)
            .checked_add_signed(Duration::milliseconds(100))
            .unwrap();
        let actual = date.millisecond();
        assert_eq!(actual, 100);
    }
    #[test]
    fn test_set_millisecond() {
        let date = calc_datetime(2023, 6, 11, 0, 0, 1);
        let result = date.set_millisecond(100);
        let actual = result.millisecond();
        assert_eq!(actual, 100);
    }
}
