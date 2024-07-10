use std::iter::from_fn;

use chrono::{NaiveDate, Weekday};

use crate::{DateRange, DayHelper, MonthHelper, QuarterHelper, WeekHelper, YearHelper};

impl DateRange<NaiveDate> for NaiveDate {
    fn days(&self) -> impl Iterator<Item = NaiveDate> {
        self.iter_days()
    }

    fn day_in_month_iter(&self) -> impl Iterator<Item = NaiveDate> {
        let start = self.begin_of_month();
        let end = self.end_of_month();
        with_end(start, end)
    }

    fn weeks(&self) -> impl Iterator<Item = NaiveDate> {
        self.iter_weeks()
    }

    fn weekend_in_year_iter(&self) -> impl Iterator<Item = NaiveDate> {
        let start = self.begin_of_year();
        start.iter_days().filter(move |&date| date.is_weekend())
    }

    fn months(&self) -> impl Iterator<Item = NaiveDate> {
        let start = self.begin_of_month();
        let mut next = start;
        from_fn(move || {
            let result = next;
            next = next.add_months(1);
            Some(result)
        })
    }

    fn months_end(&self, end: &Self) -> impl Iterator<Item = NaiveDate> {
        self.months().take_while(move |&date| date <= *end)
    }

    fn month_in_year_iter(&self) -> impl Iterator<Item = NaiveDate> {
        let start = self.begin_of_year();
        let end = self.end_of_year();
        let mut next = start;
        from_fn(move || {
            let result = next;
            if next <= end {
                next = next.add_months(1);
                Some(result)
            } else {
                None
            }
        })
    }

    fn day_in_week_iter(&self) -> impl Iterator<Item = NaiveDate> {
        self.day_in_week_with_iter(Weekday::Mon)
    }

    fn day_in_week0_iter(&self) -> impl Iterator<Item = NaiveDate> {
        self.day_in_week_with_iter(Weekday::Sun)
    }

    fn day_in_week_with_iter(&self, weekday: Weekday) -> impl Iterator<Item = NaiveDate> {
        let start = self.begin_of_week_with(weekday);
        let end = self.end_of_week_with(weekday);
        with_end(start, end)
    }
    fn quarters(&self) -> impl Iterator<Item = NaiveDate> {
        let mut start = self.begin_of_quarter();
        from_fn(move || {
            let result = start;
            start = start.add_quarters(1);
            Some(result)
        })
    }
}
fn with_end(mut start: NaiveDate, end: NaiveDate) -> impl Iterator<Item = NaiveDate> {
    from_fn(move || {
        if start <= end {
            let result = start;
            start = start.add_days(1);
            Some(result)
        } else {
            None
        }
    })
}
