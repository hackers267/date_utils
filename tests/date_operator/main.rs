mod months;
mod weeks;
mod years;

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
