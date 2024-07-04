use chrono::NaiveDate;

#[cfg(test)]
fn calc_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[cfg(feature = "common")]
#[cfg(test)]
mod common {
    use super::*;
    use date_utils::CommonHelper;

    #[test]
    fn test_before_date() {
        let now = calc_date(2008, 8, 8);
        let other = calc_date(2018, 8, 8);
        assert!(now.before(&other))
    }
    #[test]
    fn test_after_date() {
        let now = calc_date(2018, 8, 8);
        let other = calc_date(2008, 8, 8);
        assert!(now.after(&other))
    }
}

#[cfg(feature = "day")]
#[cfg(test)]
mod days {
    use super::*;
    use date_utils::DayHelper;
    #[test]
    fn test_is_same_day_date_true() {
        let date = calc_date(2008, 8, 8);
        let other = calc_date(2008, 8, 8);
        assert!(date.is_same_day(&other));
    }
    #[test]
    fn test_is_same_day_date_false() {
        let date = calc_date(2008, 8, 9);
        let other = calc_date(2008, 8, 8);
        assert!(!date.is_same_day(&other));
    }
}

#[cfg(feature = "year")]
#[cfg(test)]
mod years {
    use super::*;
    use date_utils::YearHelper;

    #[test]
    fn test_years_since_date() {
        let date = calc_date(2008, 8, 8);
        let other = calc_date(2018, 8, 8);
        let diff = other.years_since(date);
        assert_eq!(diff, Some(10))
    }
    #[test]
    fn test_is_same_year_date_true() {
        let date = calc_date(2008, 8, 8);
        let other = calc_date(2008, 1, 1);
        assert!(date.is_same_year(&other));
    }
    #[test]
    fn test_is_same_year_date_false() {
        let date = calc_date(2008, 8, 8);
        let other = calc_date(2007, 12, 31);
        assert!(!date.is_same_year(&other))
    }
    #[test]
    fn test_begin_of_year_date_true() {
        let date = calc_date(2008, 8, 8);
        let result = date.begin_of_year();
        let begin = calc_date(2008, 1, 1);
        assert_eq!(begin, result);
    }
    #[test]
    fn test_is_leap_year() {
        let date = calc_date(1900, 1, 1);
        assert!(!date.is_leap_year());
    }
}
#[cfg(feature = "month")]
#[cfg(test)]
mod months {
    use super::*;
    use date_utils::{MonthHelper, Range};
    #[test]
    fn test_is_same_month_date_true() {
        let date = calc_date(2008, 8, 8);
        let other = calc_date(2008, 8, 12);
        assert!(date.is_same_month(&other))
    }
    #[test]
    fn test_is_same_month_date_false() {
        let date = calc_date(2008, 8, 8);
        let other = calc_date(2008, 1, 1);
        assert!(!date.is_same_month(&other));
    }

    #[test]
    fn test_end_of_month_may_true() {
        let date = calc_date(2008, 5, 5);
        let end = calc_date(2008, 5, 31);
        let result = date.end_of_month();
        assert_eq!(result, end);
    }
    #[test]
    fn test_end_of_month_february_2006_true() {
        let date = calc_date(2006, 2, 2);
        let end = calc_date(2006, 2, 28);
        let result = date.end_of_month();
        assert_eq!(result, end);
    }
    #[test]
    fn test_end_of_month_april_true() {
        let date = calc_date(2008, 4, 5);
        let end = calc_date(2008, 4, 30);
        let result = date.end_of_month();
        assert_eq!(result, end);
    }

    #[test]
    fn test_end_of_month_february_2008_true() {
        let date = calc_date(2008, 2, 5);
        let end = calc_date(2008, 2, 29);
        let result = date.end_of_month();
        assert_eq!(result, end);
    }

    #[test]
    fn test_add_months() {
        let one = calc_date(2023, 4, 8);
        let result = one.add_months(8);
        let actual = calc_date(2023, 12, 8);
        assert_eq!(result, actual);
    }
    #[test]
    fn test_diff_in_calendar_months() {
        let one = calc_date(2023, 4, 8);
        let other = calc_date(2022, 1, 7);
        let result = one.diff_calendar_month(&other);
        assert_eq!(result, 15);
    }
    #[test]
    fn test_diff_in_months() {
        let one = calc_date(2023, 4, 8);
        let other = calc_date(2022, 1, 9);
        let result = one.diff_month(&other);
        assert_eq!(result, 14);
    }
    #[test]
    fn test_each_weekend() {
        let date = calc_date(2024, 6, 1);
        let weekends = date.each_weekend();
        let actual = vec![
            (Some(calc_date(2024, 6, 1)), Some(calc_date(2024, 6, 2))),
            (Some(calc_date(2024, 6, 8)), Some(calc_date(2024, 6, 9))),
            (Some(calc_date(2024, 6, 15)), Some(calc_date(2024, 6, 16))),
            (Some(calc_date(2024, 6, 22)), Some(calc_date(2024, 6, 23))),
            (Some(calc_date(2024, 6, 29)), Some(calc_date(2024, 6, 30))),
        ];
        assert_eq!(weekends, actual);
    }
    #[test]
    fn test_each_weekend_first_sat_is_none() {
        let date = calc_date(2023, 10, 1);
        let weekends = date.each_weekend();
        let actual = vec![
            (None, Some(calc_date(2023, 10, 1))),
            (Some(calc_date(2023, 10, 7)), Some(calc_date(2023, 10, 8))),
            (Some(calc_date(2023, 10, 14)), Some(calc_date(2023, 10, 15))),
            (Some(calc_date(2023, 10, 21)), Some(calc_date(2023, 10, 22))),
            (Some(calc_date(2023, 10, 28)), Some(calc_date(2023, 10, 29))),
        ];
        assert_eq!(weekends, actual);
    }
    #[test]
    fn test_each_weekend_last_none_is_none() {
        let date = calc_date(2023, 9, 1);
        let weekends = date.each_weekend();
        let actual = vec![
            (Some(calc_date(2023, 9, 2)), Some(calc_date(2023, 9, 3))),
            (Some(calc_date(2023, 9, 9)), Some(calc_date(2023, 9, 10))),
            (Some(calc_date(2023, 9, 16)), Some(calc_date(2023, 9, 17))),
            (Some(calc_date(2023, 9, 23)), Some(calc_date(2023, 9, 24))),
            (Some(calc_date(2023, 9, 30)), None),
        ];
        assert_eq!(weekends, actual);
    }
    #[test]
    fn test_weekend_list() {
        let date = calc_date(2023, 9, 1);
        let weekends = date.weekend_list();
        let actual = vec![
            calc_date(2023, 9, 2),
            calc_date(2023, 9, 3),
            calc_date(2023, 9, 9),
            calc_date(2023, 9, 10),
            calc_date(2023, 9, 16),
            calc_date(2023, 9, 17),
            calc_date(2023, 9, 23),
            calc_date(2023, 9, 24),
            calc_date(2023, 9, 30),
        ];
        assert_eq!(actual, weekends);
    }
    #[test]
    fn test_range() {
        let date = calc_date(2023, 9, 1);
        let list = date.range().collect::<Vec<_>>();
        assert_eq!(list.len(), 30);
        assert_eq!(list[0], calc_date(2023, 9, 1));
        assert_eq!(list.last(), Some(&calc_date(2023, 9, 30)));
    }
    #[test]
    fn test_days_of_month() {
        let list = [
            calc_date(2023, 1, 3),
            calc_date(2023, 2, 3),
            calc_date(2023, 3, 3),
            calc_date(2023, 4, 3),
            calc_date(2023, 5, 3),
            calc_date(2023, 6, 3),
            calc_date(2023, 7, 3),
            calc_date(2023, 8, 3),
            calc_date(2023, 9, 3),
            calc_date(2023, 10, 3),
            calc_date(2023, 11, 3),
            calc_date(2023, 12, 3),
        ];
        let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
        let actual = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        assert_eq!(result, actual);
    }
    #[test]
    fn test_days_of_month_leap_year() {
        let list = [
            calc_date(2024, 1, 3),
            calc_date(2024, 2, 3),
            calc_date(2024, 3, 3),
            calc_date(2024, 4, 3),
            calc_date(2024, 5, 3),
            calc_date(2024, 6, 3),
            calc_date(2024, 7, 3),
            calc_date(2024, 8, 3),
            calc_date(2024, 9, 3),
            calc_date(2024, 10, 3),
            calc_date(2024, 11, 3),
            calc_date(2024, 12, 3),
        ];
        let result = list.iter().map(|v| v.days_in_month()).collect::<Vec<_>>();
        let actual = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        assert_eq!(result, actual);
    }
    #[test]
    fn test_is_first_day_of_month_true() {
        let date = calc_date(2023, 1, 1);
        assert!(date.is_first_day_of_month());
    }
    #[test]
    fn test_is_last_day_of_month_false() {
        let date = calc_date(2023, 1, 31);
        assert!(!date.is_first_day_of_month());
    }
    #[test]
    fn test_is_last_day_of_month_true() {
        let date = calc_date(2023, 1, 31);
        assert!(date.is_last_day_of_month());
    }
    #[test]
    fn test_is_first_day_of_month_false() {
        let date = calc_date(2023, 1, 1);
        assert!(!date.is_last_day_of_month());
    }
    #[test]
    fn test_sub_months() {
        let date = calc_date(2023, 1, 1);
        let result = date.sub_months(1);
        assert_eq!(result, calc_date(2022, 12, 1));
    }
    #[test]
    fn test_last_day_of_month() {
        let date = calc_date(2023, 1, 1);
        let result = date.last_day_of_month();
        assert_eq!(result, calc_date(2023, 1, 31));
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
        let date = calc_date(2023, 10, 2);
        assert!(date.is_monday());
    }
    #[test]
    fn test_is_tuesday() {
        let date = calc_date(2023, 10, 3);
        assert!(date.is_tuesday());
    }
    #[test]
    fn test_is_wednesday() {
        let date = calc_date(2023, 10, 4);
        assert!(date.is_wednesday());
    }
    #[test]
    fn test_is_thursday() {
        let date = calc_date(2023, 10, 5);
        assert!(date.is_thursday());
    }
    #[test]
    fn test_is_friday() {
        let date = calc_date(2023, 10, 6);
        assert!(date.is_friday());
    }
    #[test]
    fn test_is_saturday() {
        let date = calc_date(2023, 10, 7);
        assert!(date.is_saturday());
    }
    #[test]
    fn test_is_sunday() {
        let date = calc_date(2023, 10, 8);
        assert!(date.is_sunday());
    }
    #[test]
    fn test_is_weekend() {
        let sat = calc_date(2023, 10, 7);
        let sun = calc_date(2023, 10, 8);
        let result = [sat, sun].iter().all(|date| date.is_weekend());
        assert!(result);
    }
    #[test]
    fn test_is_workday() {
        let monday = calc_date(2023, 10, 2);
        let tuesday = calc_date(2023, 10, 3);
        let wednesday = calc_date(2023, 10, 4);
        let thursday = calc_date(2023, 10, 5);
        let friday = calc_date(2023, 10, 6);
        let result = [monday, tuesday, wednesday, thursday, friday]
            .iter()
            .all(|date| date.is_workday());
        assert!(result);
    }
    #[test]
    fn test_add_week() {
        let date = calc_date(2023, 10, 2);
        let result = date.add_week(1);
        assert_eq!(result, calc_date(2023, 10, 9));
    }
    #[test]
    fn test_end_of_week() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week();
        assert_eq!(result, calc_date(2023, 10, 8));
    }
    #[test]
    fn test_end_of_week0() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week0();
        assert_eq!(result, calc_date(2023, 10, 7));
    }
    #[test]
    fn test_begin_of_week() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week();
        assert_eq!(result, calc_date(2023, 10, 2));
    }
    #[test]
    fn test_begin_of_week0() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week0();
        assert_eq!(result, calc_date(2023, 10, 1));
    }
    #[test]
    fn test_begin_of_week_with_sun() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Sun);
        assert_eq!(result, calc_date(2023, 10, 1));
    }
    #[test]
    fn test_begin_of_week_with_sat() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Sat);
        assert_eq!(result, calc_date(2023, 9, 30));
    }
    #[test]
    fn test_begin_of_week_with_mon() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Mon);
        assert_eq!(result, calc_date(2023, 10, 2));
    }
    #[test]
    fn test_begin_of_week_with_tue() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Tue);
        assert_eq!(result, calc_date(2023, 9, 26));
    }
    #[test]
    fn test_begin_of_week_with_wed() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Wed);
        assert_eq!(result, calc_date(2023, 9, 27));
    }
    #[test]
    fn test_begin_of_week_with_thu() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Thu);
        assert_eq!(result, calc_date(2023, 9, 28));
    }
    #[test]
    fn test_begin_of_week_with_fri() {
        let date = calc_date(2023, 10, 2);
        let result = date.begin_of_week_with(Weekday::Fri);
        assert_eq!(result, calc_date(2023, 9, 29));
    }
    #[test]
    fn test_end_of_week_with_sun() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Sun);
        assert_eq!(result, calc_date(2023, 10, 7));
    }
    #[test]
    fn test_end_of_week_with_sat() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Sat);
        assert_eq!(result, calc_date(2023, 10, 6));
    }
    #[test]
    fn test_end_of_week_with_mon() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Mon);
        assert_eq!(result, calc_date(2023, 10, 8));
    }
    #[test]
    fn test_end_of_week_with_tue() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Tue);
        assert_eq!(result, calc_date(2023, 10, 2));
    }
    #[test]
    fn test_end_of_week_with_wed() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Wed);
        assert_eq!(result, calc_date(2023, 10, 3));
    }
    #[test]
    fn test_end_of_week_with_thu() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Thu);
        assert_eq!(result, calc_date(2023, 10, 4));
    }
    #[test]
    fn test_end_of_week_with_fri() {
        let date = calc_date(2023, 10, 2);
        let result = date.end_of_week_with(Weekday::Fri);
        assert_eq!(result, calc_date(2023, 10, 5));
    }
    #[test]
    fn test_sub_weeks() {
        let date = calc_date(2023, 10, 2);
        let result = date.sub_week(1);
        assert_eq!(result, calc_date(2023, 9, 25));
    }
    #[test]
    fn test_week_of_month() {
        let date = calc_date(2017, 11, 9);
        let result = date.week_of_month();
        assert_eq!(result, 2);
    }
    #[test]
    fn test_week_of_month0() {
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(1).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(1).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(2).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(3).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(4).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(5).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(6).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let mut list = date.range();
        let first = list.by_ref().take(7).collect::<Vec<_>>();
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
        let date = calc_date(2023, 7, 2);
        let weeks = date.weeks_of_month();
        assert_eq!(weeks, 6);
    }
    #[test]
    fn test_weeks_of_month0() {
        let date = calc_date(2023, 7, 2);
        let weeks = date.weeks_of_month0();
        assert_eq!(weeks, 6);
    }
    #[test]
    fn test_weeks_of_month_with() {
        let weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        let date = calc_date(2023, 7, 2);
        let weeks = weekdays
            .iter()
            .map(|d| date.weeks_of_month_with(*d))
            .collect::<Vec<_>>();
        let actual = vec![6, 5, 5, 5, 5, 5, 6];
        assert_eq!(weeks, actual);
    }
    #[test]
    fn test_is_same_week_true() {
        let date = calc_date(2023, 7, 3);
        let other = calc_date(2023, 7, 9);
        let is_same = date.is_same_week(&other);
        assert!(is_same);
    }
    #[test]
    fn test_is_same_week_false() {
        let date = calc_date(2023, 7, 3);
        let other = calc_date(2023, 7, 10);
        let is_same = date.is_same_week(&other);
        assert!(!is_same);
    }
    #[test]
    fn test_is_same_week0_true() {
        let date = calc_date(2023, 7, 2);
        let other = calc_date(2023, 7, 8);
        let is_same = date.is_same_week0(&other);
        assert!(is_same);
    }
    #[test]
    fn test_is_same_week0_false() {
        let date = calc_date(2023, 7, 8);
        let other = calc_date(2023, 7, 9);
        let is_same = date.is_same_week0(&other);
        assert!(!is_same);
    }
    #[test]
    fn test_is_same_week_with() {
        let weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        let one_days = (5..=11)
            .into_iter()
            .map(|d: u32| calc_date(2023, 6, d))
            .collect::<Vec<_>>();
        let next_days = (6..=12)
            .into_iter()
            .map(|d: u32| calc_date(2023, 7, d))
            .collect::<Vec<_>>();
        let result = zip(zip(one_days, next_days),weekdays).any(|((d1, d2),weekday)| d1.is_same_week_with(&d2,weekday));
        assert!(!result);
    }
    #[test]
    fn test_last_day_of_week() {
        let date = calc_date(2023, 7, 2);
        let last_day = date.last_day_of_week();
        assert_eq!(last_day, calc_date(2023, 7, 2));
    }
    #[test]
    fn test_last_day_of_week0() {
        let date = calc_date(2023, 7, 2);
        let last_day = date.last_day_of_week0();
        assert_eq!(last_day, calc_date(2023, 7, 8));
    }
    #[test]
    fn test_diff_calendar_weeks_postive() {
        let date = calc_date(2014, 7, 21);
        let other = calc_date(2014, 7, 6);
        let diff = date.diff_calendar_weeks(&other);
        assert_eq!(diff, 3);
    }
    #[test]
    fn test_diff_calendar_weeks_navigate() {
        let date = calc_date(2014, 7, 6);
        let other = calc_date(2014, 7, 21);
        let diff = date.diff_calendar_weeks(&other);
        assert_eq!(diff, -3);
    }

}
