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
}

#[cfg(test)]
#[cfg(feature = "week")]
mod weeks {
    use super::*;
    use date_utils::WeekHelper;

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
}
