use chrono::Datelike;
pub use chrono::{Date, DateTime, TimeZone};

pub trait DateOperator {
    fn years_since(&self, base: Self) -> Option<u32>;
    /// To check the current date or datetime is before the given date or datetime or not.
    /// 判断当前日期或时间是否在指定的日期或时间之前
    fn before(&self, other: &Self) -> bool;
    /// To check the current date or datetime is after the given date or datetime or not.
    /// 判断当前日期或时间是否在指定的日期或时间之后
    fn after(&self, other: &Self) -> bool;
    /// To check the other date or datetime is the same day with the given date or datetime or not.
    /// 判断当前日期或时间是否与指定的日期或时间在同一天
    fn is_same_day(&self, other: &Self) -> bool;
    /// To check the other date or datetime is the same month with the given date or datetime or not.
    /// 判断当前日期或时间是否与指定的日期或时间在同一个月
    fn is_same_month(&self, other: &Self) -> bool;
    /// To check the other date or datetime is same year or not with the date. 判断另一个日期或时间是否和本日期或时间在同一年
    fn is_same_year(&self, other: &Self) -> bool;
    /// Get the begin of year.
    /// 获取一年的开始
    fn begin_of_year(&self) -> Self;
    /// Get the end of year.
    /// 获取一个年的结束。
    fn end_of_year(&self) -> Self;
    /// Get the begin of month.
    /// 获取某个月的开始
    fn begin_of_month(&self) -> Self;
    ///  Get the end of month.
    /// 获取某个月的结束
    fn end_of_month(&self) -> Self;
    /// Get the date or datetime is a leap year.
    /// 获取日期或时间所在的年是不是闰年。
    fn is_leap_year(&self) -> bool;
}

impl<Tz: TimeZone> DateOperator for Date<Tz> {
    fn years_since(&self, base: Self) -> Option<u32> {
        self.years_since(base)
    }

    /// To check the current date  is before the given date or not.
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

    /// To check the current date is after the given date or datetime or not.
    /// 判断当前刵是是否在指定的日期后。
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let other = Utc.ymd(2006,6,6);
    /// assert!(date.after(&other));
    /// ```
    fn after(&self, other: &Self) -> bool {
        self > other
    }

    /// To check the current date is the same date with the given date or not.
    /// 判断当前日期和指定的日期是否在同一天
    ///
    /// #Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let other = Utc.ymd(2006,6,6);
    /// assert!(!date.is_same_day(&other))
    /// ```
    ///
    fn is_same_day(&self, other: &Self) -> bool {
        self == other
    }

    /// To check the current date is the same date with the given date or not.
    /// 判断当前日期和指定的日期是否在同一个月
    ///
    /// # Example 示例
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let other = Utc.ymd(2008,8,31);
    /// assert!(date.is_same_month(&other));
    /// ```
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

    /// Get the end of year.
    /// 获取一年的结束日期。
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let result = date.end_of_year();
    /// let end = Utc.ymd(2008,12,31);
    /// assert_eq!(result, end);
    /// ```
    fn end_of_year(&self) -> Self {
        self.with_month(12).unwrap().with_day(31).unwrap()
    }

    /// Get the begin date of a month.
    /// 获取某个月的开始日
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let result = date.begin_of_month();
    /// let begin = Utc.ymd(2008,8,1);
    /// assert_eq!(result,begin);
    /// ```
    fn begin_of_month(&self) -> Self {
        self.with_day(1).unwrap()
    }

    /// Get the end of month.
    /// 获取某个月的结束日期
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8);
    /// let result = date.end_of_month();
    /// let end = Utc.ymd(2008,8,31);
    /// assert_eq!(result,end);
    /// ```
    fn end_of_month(&self) -> Self {
        let day31 = [1, 3, 5, 7, 8, 10, 12];
        let day30 = [4, 6, 9, 11];
        let month = self.month();
        if day31.binary_search(&month).is_ok() {
            return self.with_day(31).unwrap();
        }
        if day30.binary_search(&month).is_ok() {
            return self.with_day(30).unwrap();
        }
        if self.is_leap_year() {
            return self.with_day(29).unwrap();
        }
        self.with_day(28).unwrap()
    }

    /// To check the date's year is leap year or not.
    /// 判断日期所在的年份是可是闰年
    ///
    /// #  Examples 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date =  Utc.ymd(2008,8,8);
    /// assert!(date.is_leap_year());
    /// ```
    fn is_leap_year(&self) -> bool {
        let year = self.year();
        if year % 400 == 0 {
            return true;
        }
        if year % 4 == 0 {
            return true;
        }
        false
    }
}

impl<Tz: TimeZone> DateOperator for DateTime<Tz> {
    fn years_since(&self, base: Self) -> Option<u32> {
        self.years_since(base)
    }

    /// To check the current datetime is before the given  datetime or not.
    /// 判断当前时间是否在指定的时间前。
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let date = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let other = Utc.ymd(2008,1,1).and_hms(6,6,6);
    /// assert!(!date.before(&other));
    /// ```
    fn before(&self, other: &Self) -> bool {
        self < other
    }

    /// To check the current datetime is after the given datetime or not.
    /// 判断当前时间是否在指定的时间后。
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let other = Utc.ymd(2006,6,6).and_hms(6,6,6);
    /// assert!(datetime.after(&other));
    /// ```
    ///
    fn after(&self, other: &Self) -> bool {
        self > other
    }

    /// To check the current datetime is the same datetime with the given datetime or not.
    /// 判断当前时间和指定的时间是否在同一天
    ///
    /// #Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let other = Utc.ymd(2008,8,8).and_hms(6,6,6);
    /// assert!(datetime.is_same_day(&other));
    /// ```
    ///
    fn is_same_day(&self, other: &Self) -> bool {
        self.date() == other.date()
    }

    /// To check the current datetime is the same month with the given datetime or not.
    /// 判断当前时间和指定的时间是否在同一个月
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let other = Utc.ymd(2008,8,31).and_hms(6,6,6);
    /// assert!(datetime.is_same_month(&other))
    /// ```
    ///
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

    /// Get the end time of the year.
    /// 获取一年的结束时间
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let result = datetime.end_of_year();
    /// let end = Utc.ymd(2008,12,31).and_hms(23,59,59);
    /// ```
    fn end_of_year(&self) -> Self {
        self.date()
            .with_month(12)
            .unwrap()
            .with_day(31)
            .unwrap()
            .and_hms(23, 59, 59)
    }

    /// Get the start time of the month.
    /// 获取某个月的开始时间
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime =  Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let result = datetime.begin_of_month();
    /// let begin = Utc.ymd(2008,8,1).and_hms(0,0,0);
    /// assert_eq!(result,begin);
    /// ```
    ///
    fn begin_of_month(&self) -> Self {
        self.date().with_day(1).unwrap().and_hms(0, 0, 0)
    }

    /// Get the end time of the month.
    /// 获取某个月的结束时间
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// let result = datetime.end_of_month();
    /// let end = Utc.ymd(2008,8,31).and_hms(23, 59, 59);
    /// assert_eq!(result,end);
    /// ```
    fn end_of_month(&self) -> Self {
        let date = self.date();
        let end_date = date.end_of_month();
        end_date.and_hms(23, 59, 59)
    }

    /// To check whether the datetime is leap year or not.
    /// 判断时间所有的年是否是闰年
    ///
    /// # Example 示例
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    /// use date_utils::DateOperator;
    /// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
    /// assert!(datetime.is_leap_year());
    /// ```
    fn is_leap_year(&self) -> bool {
        self.date().is_leap_year()
    }
}
