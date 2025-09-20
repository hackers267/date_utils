#[cfg(test)]
#[cfg(feature = "quarter")]
mod decade {
    use chrono::{NaiveDate, NaiveDateTime};
    use date_utils::DecadeHelper;

    #[test]
    fn test_begin_of_decade() {
        let date = calc_date(2023, 1, 1);
        let result = date.begin_of_decade();
        assert_eq!(result, calc_date(2020, 1, 1));
    }
    #[test]
    fn test_end_of_decade() {
        let date = calc_date(2023, 1, 1);
        let result = date.end_of_decade();
        assert_eq!(result, calc_date(2029, 12, 31));
    }

    #[test]
    fn test_last_day_of_decade() {
        let date = calc_date(2023, 1, 1);
        let result = date.last_day_of_decade();
        assert_eq!(result, calc_date(2029, 12, 31));
    }

    #[test]
    fn test_decade() {
        let date = calc_date(2023, 1, 1);
        let result = date.decade();
        assert_eq!(result, 2020);
    }

    #[test]
    fn test_begin_of_decade_time() {
        let date = calc_date_time(2023, 1, 1, 1, 1, 1);
        let result = date.begin_of_decade();
        assert_eq!(result, calc_date_time(2020, 1, 1, 0, 0, 0));
    }

    #[test]
    fn test_end_of_decade_time() {
        let date = calc_date_time(2023, 1, 1, 1, 1, 1);
        let result = date.end_of_decade();
        assert_eq!(result, calc_date_time(2029, 12, 31, 23, 59, 59));
    }

    #[test]
    fn test_last_day_of_decade_time() {
        let date = calc_date_time(2023, 1, 1, 1, 1, 1);
        let result = date.last_day_of_decade();
        assert_eq!(result, calc_date_time(2029, 12, 31, 0, 0, 0));
    }

    #[test]
    fn test_decade_time() {
        let date = calc_date_time(2023, 1, 1, 1, 1, 1);
        let result = date.decade();
        assert_eq!(result, 2020);
    }

    fn calc_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn calc_date_time(
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
}
