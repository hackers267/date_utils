use chrono::Datelike;
pub use chrono::{Date, DateTime, TimeZone};

pub trait DateOperator {
    fn years_since(&self, base: Self) -> Option<u32>;
    fn before(&self, other: &Self) -> bool;
    fn after(&self, other: &Self) -> bool;
    fn is_same_day(&self, other: &Self) -> bool;
    fn is_same_month(&self, other: &Self) -> bool;
    fn is_same_year(&self, other: &Self) -> bool;
    /// 获取一年的开始
    fn begin_of_year(&self) -> Self;
}

impl<Tz: TimeZone> DateOperator for Date<Tz> {
    fn years_since(&self, base: Self) -> Option<u32> {
        self.years_since(base)
    }

    fn before(&self, other: &Self) -> bool {
        self < other
    }

    fn after(&self, other: &Self) -> bool {
        self > other
    }

    fn is_same_day(&self, other: &Self) -> bool {
        self == other
    }

    fn is_same_month(&self, other: &Self) -> bool {
        self.year() == other.year() && self.month() == other.month()
    }

    fn is_same_year(&self, other: &Self) -> bool {
        self.year() == other.year()
    }

    /// Get the start date of a year.
    /// 获取一年的开始日期
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let result = date.begin_of_year();
    /// let begin = Utc.ymd(2008,1,1);
    /// assert_eq!(begin,result);
    /// ```
    fn begin_of_year(&self) -> Self {
        self.with_month(1).unwrap().with_day(1).unwrap()
    }
}

impl<Tz: TimeZone> DateOperator for DateTime<Tz> {
    fn years_since(&self, base: Self) -> Option<u32> {
        self.years_since(base)
    }

    fn before(&self, other: &Self) -> bool {
        self < other
    }

    fn after(&self, other: &Self) -> bool {
        self > other
    }

    fn is_same_day(&self, other: &Self) -> bool {
        self.date() == other.date()
    }

    fn is_same_month(&self, other: &Self) -> bool {
        self.year() == other.year() && self.month() == other.month()
    }

    fn is_same_year(&self, other: &Self) -> bool {
        self.year() == other.year()
    }
    /// Get the start time of a year.
    /// 获取一年的开始时间
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let result = datetime.begin_of_year();
    /// let begin = Utc.ymd(2008,1,1).and_hms(0,0,0);
    /// assert_eq!(begin, result);
    /// ```
    fn begin_of_year(&self) -> Self {
        self.date()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .and_hms(0, 0, 0)
    }
}
