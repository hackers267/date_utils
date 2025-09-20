#[cfg(test)]
#[cfg(feature = "range")]
mod ranges {
    use chrono::{NaiveDate, Weekday};
    use date_utils::DateRange;

    #[test]
    fn test_day_in_month_iter() {
        let date = calc_date(2022, 1, 1);
        let mut iter = date.day_in_month_iter();
        assert_eq!(iter.next(), Some(calc_date(2022, 1, 1)));
        assert_eq!(iter.last(), Some(calc_date(2022, 1, 31)));
    }

    #[test]
    fn test_days() {
        let date = calc_date(2022, 1, 1);
        let mut iter = date.days();
        assert_eq!(iter.next(), Some(calc_date(2022, 1, 1)));
        assert_eq!(iter.take(8).last(), Some(calc_date(2022, 1, 9)));
    }

    #[test]
    fn test_months() {
        let date = calc_date(2022, 1, 1);
        let mut iter = date.months();
        assert_eq!(iter.next(), Some(calc_date(2022, 1, 1)));
        assert_eq!(iter.take(12).last(), Some(calc_date(2023, 1, 1)));
    }

    #[test]
    fn test_months_end() {
        let date = calc_date(2022, 1, 1);
        let end = calc_date(2023, 1, 1);
        let mut iter = date.months_end(&end);
        assert_eq!(iter.next(), Some(calc_date(2022, 1, 1)));
        assert_eq!(iter.last(), Some(calc_date(2023, 1, 1)));
    }

    #[test]
    fn month_in_year_iter() {
        let date = calc_date(2022, 3, 1);
        let mut iter = date.month_in_year_iter();
        assert_eq!(iter.next(), Some(calc_date(2022, 1, 1)));
        assert_eq!(iter.last(), Some(calc_date(2022, 12, 1)));
    }

    #[test]
    fn test_day_in_week_iter() {
        let date = calc_date(2022, 1, 1);
        let mut iter = date.day_in_week_iter();
        assert_eq!(iter.next(), Some(calc_date(2021, 12, 27)));
        assert_eq!(iter.last(), Some(calc_date(2022, 1, 2)));
    }
    #[test]
    fn test_day_in_week0_iter() {
        let date = calc_date(2022, 1, 1);
        let mut iter = date.day_in_week0_iter();
        assert_eq!(iter.next(), Some(calc_date(2021, 12, 26)));
        assert_eq!(iter.last(), Some(calc_date(2022, 1, 1)));
    }
    #[test]
    fn test_day_in_week_with_iter() {
        let date = calc_date(2022, 1, 1);
        let mut iter = date.day_in_week_with_iter(Weekday::Fri);
        assert_eq!(iter.next(), Some(calc_date(2021, 12, 31)));
        assert_eq!(iter.last(), Some(calc_date(2022, 1, 6)));
    }

    fn calc_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

#[cfg(test)]
#[cfg(feature = "range")]
mod times {
    use chrono::NaiveDate;
    use date_utils::TimeRange;

    #[test]
    fn test_hours() {
        let date = calc_datetime(2022, 1, 1, 1, 1, 1);
        let mut iter = date.hours();
        assert_eq!(iter.next(), Some(calc_datetime(2022, 1, 1, 1, 0, 0)));
        assert_eq!(
            iter.take(10).last(),
            Some(calc_datetime(2022, 1, 1, 11, 0, 0))
        );
    }

    #[test]
    fn test_minutes() {
        let date = calc_datetime(2022, 1, 1, 1, 1, 1);
        let mut iter = date.minutes();
        assert_eq!(iter.next(), Some(calc_datetime(2022, 1, 1, 1, 1, 0)));
        assert_eq!(
            iter.take(10).last(),
            Some(calc_datetime(2022, 1, 1, 1, 11, 0))
        )
    }
    #[test]
    fn test_seconds() {
        let date = calc_datetime(2022, 1, 1, 1, 1, 1);
        let mut iter = date.seconds();
        assert_eq!(iter.next(), Some(calc_datetime(2022, 1, 1, 1, 1, 1)));
        assert_eq!(
            iter.take(10).last(),
            Some(calc_datetime(2022, 1, 1, 1, 1, 11))
        )
    }
    #[test]
    fn test_hours_with_iter() {
        let date = calc_datetime(2022, 1, 1, 1, 1, 1);
        let end = calc_datetime(2023, 1, 1, 1, 1, 1);
        let mut iter = date.hours_with_iter(&end);
        assert_eq!(iter.next(), Some(calc_datetime(2022, 1, 1, 1, 0, 0)));
        assert_eq!(iter.last(), Some(calc_datetime(2023, 1, 1, 1, 0, 0)));
        let iter = date.hours_with_iter(&end);
        let count = iter.count();
        assert_eq!(count, 365 * 24 + 1);
    }
    #[test]
    fn test_minutes_with_iter() {
        let date = calc_datetime(2022, 1, 1, 1, 1, 1);
        let end = calc_datetime(2023, 1, 1, 1, 1, 1);
        let mut iter = date.minutes_with_iter(&end);
        assert_eq!(iter.next(), Some(calc_datetime(2022, 1, 1, 1, 1, 0)));
        assert_eq!(iter.last(), Some(calc_datetime(2023, 1, 1, 1, 1, 0)));
        let iter = date.minutes_with_iter(&end);
        let count = iter.count();
        assert_eq!(count, 365 * 24 * 60 + 1);
    }
    #[test]
    fn test_seconds_with_iter() {
        let date = calc_datetime(2022, 1, 1, 1, 1, 1);
        let end = calc_datetime(2022, 1, 8, 1, 1, 1);
        let mut iter = date.seconds_with_iter(&end);
        assert_eq!(iter.next(), Some(calc_datetime(2022, 1, 1, 1, 1, 1)));
        assert_eq!(iter.last(), Some(calc_datetime(2022, 1, 8, 1, 1, 1)));
        let iter = date.seconds_with_iter(&end);
        let count = iter.count();
        assert_eq!(count, 7 * 24 * 60 * 60 + 1);
    }
    fn calc_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }
}
