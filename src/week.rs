use std::ops::Add;
use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, Weekday};

pub trait WeekHelper {
    /// English: is monday
    ///
    /// 中文: 是否是周一
    fn is_monday(&self) -> bool;
    /// English: is tuesday
    ///
    /// 中文: 是否是周二
    fn is_tuesday(&self) -> bool;
    /// English: is wednesday
    ///
    /// 中文: 是否是周三
    fn is_wednesday(&self) -> bool;
    /// English: is thursday
    ///
    /// 中文: 是否是周四
    fn is_thursday(&self) -> bool;
    /// English: is friday
    ///
    /// 中文: 是否是周五
    fn is_friday(&self) -> bool;
    /// English: is saturday
    ///
    /// 中文: 是否是周六
    fn is_saturday(&self) -> bool;
    /// English: is sunday
    ///
    /// 中文: 是否是周日
    fn is_sunday(&self) -> bool;
    /// English: is weekend
    ///
    /// 中文: 是否是周末
    fn is_weekend(&self) -> bool;
    /// English: is working day
    ///
    /// 中文: 是否是工作日
    fn is_workday(&self) -> bool;
    /// English: Add the specified number of week to the given date.
    ///
    /// 中文: 给指定的日期添加指定的周数
    fn add_week(&self, week: i32) -> Self;
    /// English: Return the end of a week for the given date. The result will be in the local timezone. The week starts on Monday.
    ///
    /// 中文: 返回指定日期所在周的结束日期，以周一为一个周的开始日期
    fn end_of_week(&self) -> Self;
    /// English: Return the start of a week for the given date. The result will be in the local timezone. The week starts on Sunday.
    ///
    /// 中文: 返回指定日期所在周的开始日期，以周日为一个周的开始日期
    fn end_of_week0(&self) -> Self;
}
impl WeekHelper for NaiveDate {
    fn is_monday(&self) -> bool {
        matches!(self.weekday(), Weekday::Mon)
    }
    fn is_tuesday(&self) -> bool {
        matches!(self.weekday(), Weekday::Tue)
    }
    fn is_wednesday(&self) -> bool {
        matches!(self.weekday(), Weekday::Wed)
    }
    fn is_thursday(&self) -> bool {
        matches!(self.weekday(), Weekday::Thu)
    }
    fn is_friday(&self) -> bool {
        matches!(self.weekday(), Weekday::Fri)
    }
    fn is_saturday(&self) -> bool {
        matches!(self.weekday(), Weekday::Sat)
    }
    fn is_sunday(&self) -> bool {
        matches!(self.weekday(), Weekday::Sun)
    }
    fn is_weekend(&self) -> bool {
        matches!(self.weekday(), Weekday::Sat | Weekday::Sun)
    }
    fn is_workday(&self) -> bool {
        !self.is_weekend()
    }

    fn add_week(&self, week: i32) -> NaiveDate {
        self.add(Days::new(7))
    }

    fn end_of_week(&self) -> Self {
        self.week(Weekday::Mon).last_day()
    }

    fn end_of_week0(&self) -> Self {
        self.week(Weekday::Sun).last_day()
    }
}

impl WeekHelper for NaiveDateTime {
    fn is_monday(&self) -> bool {
        matches!(self.weekday(), Weekday::Mon)
    }
    fn is_tuesday(&self) -> bool {
        matches!(self.weekday(), Weekday::Tue)
    }
    fn is_wednesday(&self) -> bool {
        matches!(self.weekday(), Weekday::Wed)
    }
    fn is_thursday(&self) -> bool {
        matches!(self.weekday(), Weekday::Thu)
    }
    fn is_friday(&self) -> bool {
        matches!(self.weekday(), Weekday::Fri)
    }
    fn is_saturday(&self) -> bool {
        matches!(self.weekday(), Weekday::Sat)
    }
    fn is_sunday(&self) -> bool {
        matches!(self.weekday(), Weekday::Sun)
    }
    fn is_weekend(&self) -> bool {
        matches!(self.weekday(), Weekday::Sat | Weekday::Sun)
    }
    fn is_workday(&self) -> bool {
        !self.is_weekend()
    }

    fn add_week(&self, week: i32) -> Self {
        self.add(Days::new(7))
    }

    fn end_of_week(&self) -> Self {
        self.date().week(Weekday::Mon).last_day().and_hms(23, 59, 59)
    }

    fn end_of_week0(&self) -> Self {
        self.date().week(Weekday::Sun).last_day().and_hms(23, 59, 59)
    }
}
