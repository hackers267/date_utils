use chrono::Datelike;
pub use chrono::{Date, DateTime, TimeZone};

pub trait DateOperator {
    fn years_since(&self, base: Self) -> Option<u32>;
    /// To check the current date or datetime is before the given date or datetime or not.
    /// 判断当前日期或时间是否在指定的日期或时间之前
    fn before(&self, other: &Self) -> bool;
    fn after(&self, other: &Self) -> bool;
    fn is_same_day(&self, other: &Self) -> bool;
    fn is_same_month(&self, other: &Self) -> bool;
    /// To check the other date or datetime is same year or not with the date. 判断另一个日期或时间是否和本日期或时间在同一年
    fn is_same_year(&self, other: &Self) -> bool;
    /// Get the begin of year. 获取一年的开始
    fn begin_of_year(&self) -> Self;
}

impl<Tz: TimeZone> DateOperator for Date<Tz> {
    fn years_since(&self, base: Self) -> Option<u32> {
        self.years_since(base)
    }

    /// To check the current date or datetime is before the given date or datetime or not.
    /// 判断当前日期是否在指定的日期前。
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let other = Utc.ymd(2008,1,1);
    /// assert!(!date.before(&other));
    /// ```
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

    /// To check whether the other date is the same year with the date.
    /// 判断另一个日期是否和当前日期在同一年
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let other = Utc.ymd(2008,1,1);
    /// assert!(date.is_same_year(&other))
    /// ```
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

    /// To check whether the another datetime is the same year with the datatime.
    /// 判断另一个时间是否和当前时间在同一年
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let other = Utc.ymd(2008,1,1).and_hms(0,0,0);
    /// assert!(datetime.is_same_year(&other))
    /// ```
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
