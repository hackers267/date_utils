use crate::DateOperator;
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Period {
    year: i16,
    month: i8,
    day: i8,
    house: i8,
    minute: i8,
    second: i8,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn period_between_simple_test() {
        let start = calc_date(2022, 2, 1).unwrap();
        let end = calc_date(2023, 3, 2).unwrap();
        let period = Period::bewteen(&start, &end);
        let year = period.year;
        let month = period.month;
        let day = period.day;
        assert_eq!(year, 1);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
    }

    fn calc_date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    #[test]
    fn period_between_less_one_month_test() {
        let start = calc_date(2022, 2, 10).unwrap();
        let end = calc_date(2022, 3, 9).unwrap();
        let period = Period::bewteen(&start, &end);
        let year = period.year;
        let month = period.month;
        let day = period.day;
        assert_eq!(year, 0);
        assert_eq!(month, 0);
        assert_eq!(day, 27);
    }
    #[test]
    fn period_between_test() {
        let start = calc_date(2022, 8, 28).unwrap();
        let end = calc_date(2035, 3, 10).unwrap();
        let period = Period::bewteen(&start, &end);
        assert_eq!(period.year, 12);
        assert_eq!(period.month, 6);
        assert_eq!(period.day, 10);
    }
}

impl Period {
    /// 计算两个日期之间的间隔，使用x年x月x日的记录方式
    pub fn bewteen(one: &NaiveDate, other: &NaiveDate) -> Period {
        let (another, year) = Self::calc_year(one, other);
        let (another, month) = Self::calc_month(other, another);
        let day = Self::calc_day(other, &another);

        Period {
            year,
            month,
            day,
            house: 0,
            minute: 0,
            second: 0,
        }
    }

    fn calc_day(other: &NaiveDate, another: &NaiveDate) -> i8 {
        let duration = *other - *another;
        duration.num_days() as i8
    }

    fn calc_month(other: &NaiveDate, another: NaiveDate) -> (NaiveDate, i8) {
        let month = (other.month() + 12 - another.month()) % 12;
        let total = (another.month() + month) % 12;
        let mut another = another.with_month(total).unwrap();
        let month = if another.after(other) {
            another = another.with_month(another.month() - 1).unwrap();
            month - 1
        } else {
            month
        } as i8;
        (another, month)
    }

    fn calc_year(one: &NaiveDate, other: &NaiveDate) -> (NaiveDate, i16) {
        let year = other.year() - one.year();
        let another = one.with_year(one.year() + year).unwrap();
        let year = if another.after(other) { year - 1 } else { year } as i16;
        (another, year)
    }
}
