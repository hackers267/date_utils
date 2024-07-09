pub trait QuarterHelper {
    /// English: Get the first day of the quarter
    ///
    /// 中文: 获取季度的第一天
    fn begin_of_quarter(&self) -> Self;
    /// English: Get the last day of the quarter
    ///
    /// 中文: 获取季度的最后一天
    fn end_of_quarter(&self) -> Self;
    /// English: Whether the two dates are in the same quarter
    ///
    /// 中文: 两个日期是否在同一个季度
    fn is_same_quarter(&self, other: &Self) -> bool;
    /// English: Get the quarter of the year
    ///
    /// 中文: 获取年份的季度
    fn quarter(&self) -> u32;
    /// English: Get the quarter of the year
    ///
    /// 中文: 获取年份的季度
    fn quarters(&self) -> impl Iterator<Item = Self>;
    /// English: Add the specified number of year quarters to the given date.
    ///
    /// 中文: 给定日期增加指定的年份季度数。
    fn add_quarters(&self, quarters: i32) -> Self;
    /// English: Subtract the specified number of year quarters from the given date.
    ///
    /// 中文: 给定日期减去指定的年份季度数。
    fn sub_quarters(&self, quarters: i32) -> Self;
    /// English: Get the number of calendar quarters between the given dates.
    ///
    /// 中文: 获取两个日期之间的日历季度数。
    fn diff_calendar_quarters(&self, other: &Self) -> i64;
    /// English: Get the number of quarters between the given dates.
    ///
    /// 中文: 获取两个日期之间的季度数。
    fn diff_quarters(&self, other: &Self) -> i64;
}
