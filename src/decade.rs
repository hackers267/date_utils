use chrono::{Datelike, NaiveDate, NaiveDateTime};
use crate::day::DayHelper;
use crate::year::YearHelper;

pub trait DecadeHelper {
    /// English: Get the start of one decade
    ///
    /// 中文: 获取十年的开始时间
    fn begin_of_decade(&self) -> Self;
    /// English: Get the end of one decade
    ///
    /// 中文: 获取十年的结束时间
    fn end_of_decade(&self) -> Self;
    /// English: Get the last day of decade
    ///
    /// 中文: 获取十年的最后一天
    fn last_day_of_decade(&self) -> Self;
    /// English: Get the decade
    ///
    /// 中文: 获取年代代表
    fn decade(&self) -> i32;
}

impl DecadeHelper for NaiveDate {
    fn begin_of_decade(&self) -> Self {
        let year = get_decade_start(self.year());
        self.with_year(year).unwrap().begin_of_year()
    }

    fn end_of_decade(&self) -> Self {
        let year = get_decade_end(self.year());
        self.with_year(year).unwrap().end_of_year()
    }

    fn last_day_of_decade(&self) -> Self {
        self.end_of_decade()
    }

    fn decade(&self) -> i32 {
        get_decade_start(self.year())
    }
}

fn get_decade_start(year: i32) -> i32 {
    let year = format!("{}0", year / 10);
    year.parse::<i32>().unwrap()
}
fn get_decade_end(year: i32) -> i32 {
    let year = format!("{}9", year / 10);
    year.parse::<i32>().unwrap()
}

impl DecadeHelper for NaiveDateTime {
    fn begin_of_decade(&self) -> Self {
        let year = get_decade_start(self.year());
        self.with_year(year).unwrap().begin_of_year()
    }

    fn end_of_decade(&self) -> Self {
        let year = get_decade_end(self.year());
        self.with_year(year).unwrap().end_of_year()
    }

    fn last_day_of_decade(&self) -> Self {
        self.end_of_decade().begin_of_day()
    }

    fn decade(&self) -> i32 {
        get_decade_start(self.year())
    }
}
