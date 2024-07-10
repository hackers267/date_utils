#[cfg(test)]
mod quarter_tests {
    use chrono::NaiveDate;
    use date_utils::{Quarter, QuarterHelper};

    #[test]
    fn test_begin_of_quarter() {
        let date = calc_date(2019, 3, 1);
        assert_eq!(date.begin_of_quarter(), calc_date(2019, 1, 1));
    }

    #[test]
    fn test_end_of_quarter() {
        let date = calc_date(2019, 3, 1);
        assert_eq!(date.end_of_quarter(), calc_date(2019, 3, 31));
    }

    #[test]
    fn test_is_same_quarter() {
        let date = calc_date(2019, 3, 1);
        let other = calc_date(2019, 4, 1);
        assert!(!date.is_same_quarter(&other));
        let other = calc_date(2019, 2, 1);
        assert!(date.is_same_quarter(&other));
    }

    #[test]
    fn test_quarter() {
        let date = calc_date(2019, 3, 1);
        assert_eq!(date.quarter(), Quarter::Q1);
        let date = calc_date(2019, 4, 1);
        assert_eq!(date.quarter(), Quarter::Q2);
        let date = calc_date(2019, 7, 1);
        assert_eq!(date.quarter(), Quarter::Q3);
        let date = calc_date(2019, 10, 1);
        assert_eq!(date.quarter(), Quarter::Q4);
    }
    #[test]
    fn test_add_quarter() {
        let date = calc_date(2019, 3, 1);
        assert_eq!(date.add_quarters(1), calc_date(2019, 6, 1));
    }
    #[test]
    fn test_sub_quarter() {
        let date = calc_date(2019, 7, 1);
        assert_eq!(date.sub_quarters(1), calc_date(2019, 4, 1));
    }
    #[test]
    fn test_diff_calendar_quarters() {
        let date = calc_date(2019, 3, 1);
        let other = calc_date(2019, 4, 1);
        assert_eq!(date.diff_calendar_quarters(&other), 1);
        let other = calc_date(2019, 2, 1);
        assert_eq!(date.diff_calendar_quarters(&other), 0);
    }
    #[test]
    fn test_diff_quarters() {
        let date = calc_date(2019, 5, 1);
        let other = calc_date(2019, 4, 1);
        assert_eq!(date.diff_quarters(&other), 0);
        let other = calc_date(2019, 2, 1);
        assert_eq!(date.diff_quarters(&other), 1);
    }
    fn calc_date(year: i32, month:u32,day:u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}