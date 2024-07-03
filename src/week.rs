use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday};

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
}
