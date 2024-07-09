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
    fn add_week(&self, week: u64) -> Self;
    /// English: Subtract the specified number of week to the given date.
    ///
    /// 中文: 给指定的日期减去指定的周数
    fn sub_week(&self, week: u64) -> Self;
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
    /// English: Get the number of calendar weeks between the given dates. The week starts on Monday.
    ///
    /// 中文: 获取两个日期之间的周数,周一为一周的第一天
    fn diff_calendar_weeks(&self, other: &Self) -> i64;
    /// English: Get the number of calendar weeks between the given dates. The week starts on Sunday.
    ///
    /// 中文: 获取两个日期之间的周数,周日为一周的第一天
    fn diff_calendar_weeks0(&self, other: &Self) -> i64;
    /// English: Get the number of calendar weeks between the given dates. The week starts on specified weekday.
    ///
    /// 中文: 获取两个日期之间的周数,指定的星期开始
    fn diff_calendar_weeks_with(&self, other: &Self, weekday: Weekday) -> i64;
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
    /// English:Get the number of calendar weeks the month in the given date spans. The week starts on Monday.
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
    /// English: Return the last day of a week. The week starts on Monday.
    ///
    /// 中文: 返回指定日期所在周的最后一天,周一为一周的第一天
    fn last_day_of_week(&self) -> Self;
    /// English: Return the last day of a week. The week starts on Sunday.
    ///
    /// 中文: 返回指定日期所在周的最后一天,周日为一周的第一天
    fn last_day_of_week0(&self) -> Self;
    /// English: Return the last day of a week. The week starts on specified weekday.
    ///
    /// 中文: 返回指定日期所在周的最后一天,指定的星期开始
    fn last_day_of_week_with(&self, weekday: Weekday) -> Self;
    /// English: Get the number of full weeks between two dates. Fractional weeks are truncated towards zero by default.
    ///
    /// 中文: 获取两个日期之间的完整周数,默认会截断小数部分
    fn diff_weeks(&self, other: &Self) -> i64;
    /// English: When is the next day of the week?
    ///
    /// 中文: 下一周的日期
    fn next_day(&self, weekday: Weekday) -> Self;
    /// English: When is the previous day of the week?
    ///
    /// 中文: 上一周的日期
    fn previous_day(&self, weekday: Weekday) -> Self;
    /// English: When is the next Monday
    ///
    /// 中文: 下周一的日期
    fn next_monday(&self) -> Self;
    /// English: When is the next Tuesday
    ///
    /// 中文: 下周二的日期
    fn next_tuesday(&self) -> Self;
    /// English: When is the next Wednesday
    ///
    /// 中文: 下周三的日期
    fn next_wednesday(&self) -> Self;
    /// English: When is the next Thursday
    ///
    /// 中文: 下周四的日期
    fn next_thursday(&self) -> Self;
    /// English: When is the next Friday
    ///
    /// 中文: 下周五的日期
    fn next_friday(&self) -> Self;
    /// English: When is the next Saturday
    ///
    /// 中文: 下周六的日期
    fn next_saturday(&self) -> Self;
    /// English: When is the next Sunday
    ///
    /// 中文: 下周日的日期
    fn next_sunday(&self) -> Self;
    /// English: When is the previous Monday
    ///
    /// 中文: 上周一的日期
    fn previous_monday(&self) -> Self;
    /// English: When is the previous Tuesday
    ///
    /// 中文: 上周二的日期
    fn previous_tuesday(&self) -> Self;
    /// English: When is the previous Wednesday
    ///
    /// 中文: 上周三的日期
    fn previous_wednesday(&self) -> Self;
    /// English: When is the previous Thursday
    ///
    /// 中文: 上周四的日期
    fn previous_thursday(&self) -> Self;
    /// English: When is the previous Friday
    ///
    /// 中文: 上周五的日期
    fn previous_friday(&self) -> Self;
    /// English: When is the previous Saturday
    ///
    /// 中文: 上周六的日期
    fn previous_saturday(&self) -> Self;
    /// English: When is the previous Sunday
    ///
    /// 中文: 上周日的日期
    fn previous_sunday(&self) -> Self;
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

    fn add_week(&self, week: u64) -> NaiveDate {
        self.add(Days::new(7 * week))
    }

    fn sub_week(&self, week: u64) -> Self {
        self.sub(Days::new(7 * week))
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

    fn diff_calendar_weeks(&self, other: &Self) -> i64 {
        let self_start = self.begin_of_week();
        let other_begin = other.begin_of_week();
        let diff = self_start.diff_days(&other_begin);
        diff / 7
    }

    fn diff_calendar_weeks0(&self, other: &Self) -> i64 {
        let first = self.begin_of_week0();
        let other = other.begin_of_week0();
        let diff = first.diff_days(&other);
        diff / 7
    }

    fn diff_calendar_weeks_with(&self, other: &Self, weekday: Weekday) -> i64 {
        let first = self.begin_of_week_with(weekday);
        let other = other.begin_of_week_with(weekday);
        let diff = first.diff_days(&other);
        diff / 7
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

    fn last_day_of_week(&self) -> Self {
        self.week(Weekday::Mon).last_day()
    }

    fn last_day_of_week0(&self) -> Self {
        self.week(Weekday::Sun).last_day()
    }

    fn last_day_of_week_with(&self, weekday: Weekday) -> Self {
        self.week(weekday).last_day()
    }

    fn diff_weeks(&self, other: &Self) -> i64 {
        let diff = self.diff_days(other);
        diff / 7
    }

    fn next_day(&self, weekday: Weekday) -> Self {
        let left_span = 7 - self.weekday().number_from_monday();
        let right_span = weekday.number_from_monday();
        let span = right_span + left_span;
        self.add_days(span as u64)
    }

    fn previous_day(&self, weekday: Weekday) -> Self {
        let left_span = self.weekday().number_from_monday();
        let right_span = 7 - weekday.number_from_monday();
        let span = left_span + right_span;
        self.sub_days(span as u64)
    }

    fn next_monday(&self) -> Self {
        self.next_day(Weekday::Mon)
    }

    fn next_tuesday(&self) -> Self {
        self.next_day(Weekday::Tue)
    }

    fn next_wednesday(&self) -> Self {
        self.next_day(Weekday::Wed)
    }

    fn next_thursday(&self) -> Self {
        self.next_day(Weekday::Thu)
    }

    fn next_friday(&self) -> Self {
        self.next_day(Weekday::Fri)
    }

    fn next_saturday(&self) -> Self {
        self.next_day(Weekday::Sat)
    }

    fn next_sunday(&self) -> Self {
        self.next_day(Weekday::Sun)
    }

    fn previous_monday(&self) -> Self {
        self.previous_day(Weekday::Mon)
    }

    fn previous_tuesday(&self) -> Self {
        self.previous_day(Weekday::Tue)
    }

    fn previous_wednesday(&self) -> Self {
        self.previous_day(Weekday::Wed)
    }

    fn previous_thursday(&self) -> Self {
        self.previous_day(Weekday::Thu)
    }

    fn previous_friday(&self) -> Self {
        self.previous_day(Weekday::Fri)
    }

    fn previous_saturday(&self) -> Self {
        self.previous_day(Weekday::Sat)
    }

    fn previous_sunday(&self) -> Self {
        self.previous_day(Weekday::Sun)
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

    fn add_week(&self, week: u64) -> Self {
        self.add(Days::new(7 * week))
    }

    fn sub_week(&self, week: u64) -> Self {
        self.sub(Days::new(7 * week))
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

    fn diff_calendar_weeks(&self, other: &Self) -> i64 {
        self.date().diff_calendar_weeks(&other.date())
    }

    fn diff_calendar_weeks0(&self, other: &Self) -> i64 {
        self.date().diff_calendar_weeks0(&other.date())
    }

    fn diff_calendar_weeks_with(&self, other: &Self, weekday: Weekday) -> i64 {
        self.date().diff_calendar_weeks_with(&other.date(), weekday)
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

    fn last_day_of_week(&self) -> Self {
        self.date().last_day_of_week().and_hms_opt(0, 0, 0).unwrap()
    }

    fn last_day_of_week0(&self) -> Self {
        self.date()
            .last_day_of_week0()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn last_day_of_week_with(&self, weekday: Weekday) -> Self {
        self.date()
            .last_day_of_week_with(weekday)
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn diff_weeks(&self, other: &Self) -> i64 {
        self.diff_days(other) / 7
    }

    fn next_day(&self, weekday: Weekday) -> Self {
        self.date().next_day(weekday).and_hms_opt(0, 0, 0).unwrap()
    }

    fn previous_day(&self, weekday: Weekday) -> Self {
        self.date()
            .previous_day(weekday)
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn next_monday(&self) -> Self {
        self.next_day(Weekday::Mon)
    }

    fn next_tuesday(&self) -> Self {
        self.next_day(Weekday::Tue)
    }

    fn next_wednesday(&self) -> Self {
        self.next_day(Weekday::Wed)
    }

    fn next_thursday(&self) -> Self {
        self.next_day(Weekday::Thu)
    }

    fn next_friday(&self) -> Self {
        self.next_day(Weekday::Fri)
    }

    fn next_saturday(&self) -> Self {
        self.next_day(Weekday::Sat)
    }

    fn next_sunday(&self) -> Self {
        self.next_day(Weekday::Sun)
    }

    fn previous_monday(&self) -> Self {
        self.previous_day(Weekday::Mon)
    }

    fn previous_tuesday(&self) -> Self {
        self.previous_day(Weekday::Tue)
    }

    fn previous_wednesday(&self) -> Self {
        self.previous_day(Weekday::Wed)
    }

    fn previous_thursday(&self) -> Self {
        self.previous_day(Weekday::Thu)
    }

    fn previous_friday(&self) -> Self {
        self.previous_day(Weekday::Fri)
    }

    fn previous_saturday(&self) -> Self {
        self.previous_day(Weekday::Sat)
    }

    fn previous_sunday(&self) -> Self {
        self.previous_day(Weekday::Sun)
    }
}
