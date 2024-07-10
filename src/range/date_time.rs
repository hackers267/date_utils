use crate::{
    DateRange, DayHelper, HourHelper, MinuteHelper, MonthHelper, QuarterHelper, SecondHelper,
    TimeRange, WeekHelper, YearHelper,
};
use chrono::{NaiveDateTime, Weekday};
use std::iter::from_fn;

impl DateRange<NaiveDateTime> for NaiveDateTime {
    fn days(&self) -> impl Iterator<Item = NaiveDateTime> {
        let time = self.time();
        self.date().iter_days().map(move |date| date.and_time(time))
    }

    fn day_in_month_iter(&self) -> impl Iterator<Item = NaiveDateTime> {
        let time = self.time();
        let date = self.date();
        let mut start = date.begin_of_month();
        let end = date.end_of_month();
        from_fn(move || {
            if start <= end {
                let result = start;
                start = start.add_days(1);
                Some(result.and_time(time))
            } else {
                None
            }
        })
    }

    fn weeks(&self) -> impl Iterator<Item = NaiveDateTime> {
        let mut start = self.begin_of_month();
        let end = self.end_of_month();
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

    fn months(&self) -> impl Iterator<Item = NaiveDateTime> {
        let date = self.date();
        let time = self.time();
        let mut start = date.begin_of_month();
        from_fn(move || {
            let result = start;
            start = start.add_months(1);
            Some(result.and_time(time))
        })
    }

    fn months_end(&self, end: &Self) -> impl Iterator<Item = NaiveDateTime> {
        self.months().take_while(move |&date| date <= *end)
    }

    fn month_in_year_iter(&self) -> impl Iterator<Item = NaiveDateTime> {
        let mut start = self.begin_of_year();
        let end = self.end_of_year();
        from_fn(move || {
            if start <= end {
                let result = start;
                start = start.add_months(1);
                Some(result)
            } else {
                None
            }
        })
    }

    fn day_in_week_iter(&self) -> impl Iterator<Item = NaiveDateTime> {
        self.day_in_week_with_iter(Weekday::Mon)
    }

    fn day_in_week0_iter(&self) -> impl Iterator<Item = NaiveDateTime> {
        self.day_in_week_with_iter(Weekday::Sun)
    }

    fn day_in_week_with_iter(&self, weekday: Weekday) -> impl Iterator<Item = NaiveDateTime> {
        let mut start = self.begin_of_week_with(weekday);
        let end = self.end_of_week_with(weekday);
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
    fn quarters(&self) -> impl Iterator<Item = NaiveDateTime> {
        let mut start = self.date().begin_of_quarter();
        from_fn(move || {
            let result: NaiveDateTime = start.and_hms_opt(0, 0, 0).unwrap();
            start = start.add_quarters(1);
            Some(result)
        })
    }
}
impl TimeRange<NaiveDateTime> for NaiveDateTime {
    fn hours(&self) -> impl Iterator<Item = NaiveDateTime> {
        let mut next = self.begin_of_hour();
        from_fn(move || {
            let result = next;
            next = next.add_hours(1);
            Some(result)
        })
    }

    fn minutes(&self) -> impl Iterator<Item = NaiveDateTime> {
        let mut next = self.begin_of_minute();
        from_fn(move || {
            let result = next;
            next = next.add_minutes(1);
            Some(result)
        })
    }

    fn seconds(&self) -> impl Iterator<Item = NaiveDateTime> {
        let mut next = self.begin_of_second();
        from_fn(move || {
            let result = next;
            next = next.add_second(1).unwrap();
            Some(result)
        })
    }

    fn hours_with_iter(&self, end: &Self) -> impl Iterator<Item = NaiveDateTime> {
        self.hours().take_while(move |&date| date <= *end)
    }

    fn minutes_with_iter(&self, end: &Self) -> impl Iterator<Item = NaiveDateTime> {
        self.minutes().take_while(move |&date| date <= *end)
    }

    fn seconds_with_iter(&self, end: &Self) -> impl Iterator<Item = NaiveDateTime> {
        self.seconds().take_while(move |&date| date <= *end)
    }
}
