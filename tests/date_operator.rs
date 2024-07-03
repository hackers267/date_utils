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
    use date_utils::MonthHelper;
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
}
