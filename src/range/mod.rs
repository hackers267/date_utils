use chrono::Weekday;

mod date;
mod date_time;

pub trait DateRange<T> {
    /// 返回表示天数的迭代器
    ///
    /// 此函数返回一个实现了`Iterator`接口的迭代器，迭代器中的元素类型为`T`。
    /// 这些元素代表了某个时间范围或时间序列中的天数。
    ///
    /// # 参数：
    /// - `self`：调用此方法的对象，通常是时间范围或时间序列的持有者。
    ///
    /// # 返回值：
    /// - `impl Iterator<Item = T>`：表示天数的迭代器，元素类型为`T`。
    fn days(&self) -> impl Iterator<Item = T>;
    fn day_in_month_iter(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的周数。
    ///
    /// 迭代器中的每个元素类型为 `T`，表示该对象中的一个周。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历周数，元素类型为 `T`。
    fn weeks(&self) -> impl Iterator<Item = T>;
    /// English: Get all the Saturdays and Sundays in the year.
    ///
    /// 中文： 获取年份的所有周六和周日。
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质,元素为周末的日期。
    fn weekend_in_year_iter(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的月份。
    ///
    /// 迭代器中的每个元素类型为 `T`，表示该对象中的一个月份。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历月份，元素类型为 `T`。
    fn months(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的月份。
    ///
    /// 迭代器中的每个元素类型为 `T`，表示该对象中的一个月份。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    /// - `end`：结束日期。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历月份，元素类型为 `T`。
    fn months_end(&self, end: &Self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的月份。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历月份，元素类型为 `T`。
    fn month_in_year_iter(&self) -> impl Iterator<Item = T>;
    ///
    /// 返回一个迭代器，用于遍历该对象表示的周数。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历周数，元素类型为 `T`。
    fn day_in_week_iter(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的周数。周以周日为第一天。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历周数，元素类型为 `T`。
    fn day_in_week0_iter(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的周数。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历周数，元素类型为 `T`。
    fn day_in_week_with_iter(&self, weekday: Weekday) -> impl Iterator<Item = T>;
    /// English: Get the quarter of the year
    ///
    /// 中文: 获取年份的季度
    fn quarters(&self) -> impl Iterator<Item = T>;
}

pub trait TimeRange<T> {
    /// 返回一个迭代器，用于遍历该对象表示的小时数。
    ///
    /// 迭代器中的每个元素类型为`T`，表示该对象中的一个小时。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 一个实现了`Iterator`特质的迭代器，用于遍历小时数，元素类型为`T`。
    fn hours(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的分钟数。
    ///
    /// 迭代器中的每个元素类型为 `T`，表示该对象中的一个分钟。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历分钟数，元素类型为 `T`。
    fn minutes(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的秒数。
    ///
    /// 迭代器中的每个元素类型为 `T`，表示该对象中的一个秒。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历秒数，元素类型为 `T`。
    fn seconds(&self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的小时数。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历小时数，元素类型为 `T`。
    fn hours_with_iter(&self, end: &Self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的分钟数。
    ///
    /// # 参数:
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历分钟数，元素类型为 `T`。
    fn minutes_with_iter(&self, end: &Self) -> impl Iterator<Item = T>;
    /// 返回一个迭代器，用于遍历该对象表示的秒数。
    ///
    /// # 参数：
    /// - `self`：当前对象的引用。
    ///
    /// # 返回值：
    /// - 实现了 `Iterator` 特质的迭代器，用于遍历秒数，元素类型为 `T`。
    fn seconds_with_iter(&self, end: &Self) -> impl Iterator<Item = T>;
}
