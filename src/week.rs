use crate::{DayHelper, MonthHelper};
use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, Weekday};
use std::ops::{Add, Sub};

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
    /// English: Subtract the specified number of week to the given date.
    ///
    /// 中文: 给指定的日期减去指定的周数
    fn sub_week(&self, week: i32) -> Self;
    /// English: Return the end of a week for the given date. The result will be in the local timezone. The week starts on Monday.
    ///
    /// 中文: 返回指定日期所在周的结束日期，以周一为一个周的开始日期
    fn end_of_week(&self) -> Self;
    /// English: Return the start of a week for the given date. The result will be in the local timezone. The week starts on Sunday.
    ///
    /// 中文: 返回指定日期所在周的开始日期，以周日为一个周的开始日期
    fn end_of_week0(&self) -> Self;
    /// English: Return the end of a week for the given date. The result will be in the local timezone. The week starts on specified weekday.
    ///
    /// 中文: 返回指定日期所在周的开始日期,以指定的星期开始
    fn end_of_week_with(&self, weekday: Weekday) -> Self;
    /// English: Get the number of calendar weeks between the given dates.
    ///
    /// 中文: 获取两个日期之间的周数
    fn diff_calendar_weeks(&self, other: &Self) -> i32;
    /// English: Return the start of a week for the given date. The result will be in the local timezone. The week starts on Monday.
    ///
    /// 中文: 返回指定日期所在周的开始日期,以周一为一个周的开始日期
    fn begin_of_week(&self) -> Self;
    /// English: Return the start of a week for the given date. The result will be in the local timezone. The week starts on Sunday.
    ///
    /// 中文: 返回指定日期所在周的开始日期,以周日为一个周的开始日期
    fn begin_of_week0(&self) -> Self;
    /// English: Return the start of a week for the given date. The week starts on specified weekday.
    ///
    /// 中文: 返回指定日期所在周的开始日期,以指定的星期开始
    fn begin_of_week_with(&self, weekday: Weekday) -> Self;
    /// English: Get the week of the month of the given date. The week starts on Monday
    ///
    /// 中文: 获取指定日期所在月的第几周,周一为一周的第一天
    fn week_of_month(&self) -> u8;
    /// English: Get the week of the month of the given date. The week starts on Sunday.
    ///
    /// 中文: 获取指定日期所在月的第几周,周日为一周的第一天
    fn week_of_month0(&self) -> u8;
    /// English: Get the week of the month of the given date. The week starts on specified weekday.
    ///
    /// 中文: 获取指定日期所在月的第几周,指定的星期开始
    fn week_of_month_with(&self, weekday: Weekday) -> u8;
    /// English:Get the number of calendar weeks the month in the given date spans. The wekk starts on Monday.
    ///
    /// 中文: 获取指定日期所在月的周数,周一为一周的第一天
    fn weeks_of_month(&self) -> u8;
    /// English: Get the number of calendar weeks the month in the given date spans. The week starts on Sunday.
    ///
    /// 中文: 获取指定日期所在月的周数,周日为一周的第一天
    fn weeks_of_month0(&self) -> u8;
    /// English: Get the number of calendar weeks the month in the given date spans. The week starts on specified weekday.
    ///
    /// 中文: 获取指定日期所在月的周数,指定的星期开始
    fn weeks_of_month_with(&self, weekday: Weekday) -> u8;
    /// English:Are the given dates in the same week. The week starts on Monday.
    ///
    /// 中文: 两个日期是否在同一周,周一为一周的第一天
    fn is_same_week(&self, other: &Self) -> bool;
    /// English:Are the given dates in the same week. The week starts on Sunday.
    ///
    /// 中文: 两个日期是否在同一周,周日为一周的第一天
    fn is_same_week0(&self, other: &Self) -> bool;
    /// English:Are the given dates in the same week. The week starts on specified weekday.
    ///
    /// 中文: 两个日期是否在同一周,指定的星期开始
    fn is_same_week_with(&self, other: &Self, weekday: Weekday) -> bool;
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

    fn sub_week(&self, week: i32) -> Self {
        self.sub(Days::new(7))
    }

    fn end_of_week(&self) -> Self {
        self.week(Weekday::Mon).last_day()
    }

    fn end_of_week0(&self) -> Self {
        self.week(Weekday::Sun).last_day()
    }

    fn end_of_week_with(&self, weekday: Weekday) -> Self {
        self.week(weekday).last_day()
    }

    fn diff_calendar_weeks(&self, other: &Self) -> i32 {
        todo!()
    }

    fn begin_of_week(&self) -> Self {
        self.week(Weekday::Mon).first_day()
    }

    fn begin_of_week0(&self) -> Self {
        self.week(Weekday::Sun).first_day()
    }

    fn begin_of_week_with(&self, weekday: Weekday) -> Self {
        self.week(weekday).first_day()
    }

    fn week_of_month(&self) -> u8 {
        let first_day = self.begin_of_month();
        let first_day_week = first_day.week(Weekday::Mon).first_day();
        let diff_day = self.diff_days(&first_day_week);
        (diff_day / 7) as u8 + 1
    }

    fn week_of_month0(&self) -> u8 {
        let first_day = self.begin_of_month();
        let first_day_week = first_day.week(Weekday::Sun).first_day();
        let diff = self.diff_days(&first_day_week);
        (diff / 7) as u8 + 1
    }

    fn week_of_month_with(&self, weekday: Weekday) -> u8 {
        let first_day = self.begin_of_month();
        let first_day_week = first_day.week(weekday).first_day();
        let diff = self.diff_days(&first_day_week);
        (diff / 7) as u8 + 1
    }

    fn weeks_of_month(&self) -> u8 {
        let last_day = self.end_of_month();
        last_day.week_of_month()
    }

    fn weeks_of_month0(&self) -> u8 {
        let last_day = self.end_of_month();
        last_day.week_of_month0()
    }

    fn weeks_of_month_with(&self, weekday: Weekday) -> u8 {
        let last_day = self.end_of_month();
        last_day.week_of_month_with(weekday)
    }

    fn is_same_week(&self, other: &Self) -> bool {
        self.week(Weekday::Mon).first_day() == other.week(Weekday::Mon).first_day()
    }

    fn is_same_week0(&self, other: &Self) -> bool {
        self.week(Weekday::Sun).first_day() == other.week(Weekday::Sun).first_day()
    }

    fn is_same_week_with(&self, other: &Self, weekday: Weekday) -> bool {
        self.week(weekday).first_day() == other.week(weekday).first_day()
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

    fn sub_week(&self, week: i32) -> Self {
        self.sub(Days::new(7))
    }

    fn end_of_week(&self) -> Self {
        self.date().end_of_week().and_hms_opt(23, 59, 59).unwrap()
    }

    fn end_of_week0(&self) -> Self {
        self.date().end_of_week0().and_hms_opt(23, 59, 59).unwrap()
    }

    fn end_of_week_with(&self, weekday: Weekday) -> Self {
        self.date()
            .end_of_week_with(weekday)
            .and_hms_opt(23, 59, 59)
            .unwrap()
    }

    fn diff_calendar_weeks(&self, other: &Self) -> i32 {
        todo!()
    }

    fn begin_of_week(&self) -> Self {
        self.date().begin_of_week().and_hms_opt(0, 0, 0).unwrap()
    }

    fn begin_of_week0(&self) -> Self {
        self.date().begin_of_week0().and_hms_opt(0, 0, 0).unwrap()
    }

    fn begin_of_week_with(&self, weekday: Weekday) -> Self {
        self.date()
            .begin_of_week_with(weekday)
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn week_of_month(&self) -> u8 {
        self.date().week_of_month()
    }

    fn week_of_month0(&self) -> u8 {
        self.date().week_of_month0()
    }

    fn week_of_month_with(&self, weekday: Weekday) -> u8 {
        self.date().week_of_month_with(weekday)
    }

    fn weeks_of_month(&self) -> u8 {
        self.date().weeks_of_month()
    }

    fn weeks_of_month0(&self) -> u8 {
        self.date().weeks_of_month0()
    }

    fn weeks_of_month_with(&self, weekday: Weekday) -> u8 {
        self.date().weeks_of_month_with(weekday)
    }

    fn is_same_week(&self, other: &Self) -> bool {
        self.date().is_same_week(&other.date())
    }

    fn is_same_week0(&self, other: &Self) -> bool {
        self.date().is_same_week0(&other.date())
    }

    fn is_same_week_with(&self, other: &Self, weekday: Weekday) -> bool {
        self.date().is_same_week_with(&other.date(), weekday)
    }
}
