use chrono::NaiveDateTime;

use crate::utils::utc_now;

/// English: The helper of daytime
///
/// 中文: 日时间助手
pub trait DayTimeHelper {
    /// English: Get the end of today.
    ///
    /// 中文: 获取今日的结束时间
    fn end_of_today() -> Self;

    /// English: Get the start of today.
    ///
    /// 中文: 获取今日的开始时间
    fn start_of_today() -> Self;
}
impl DayTimeHelper for NaiveDateTime {
    fn end_of_today() -> Self {
        utc_now().date_naive().and_hms_opt(23, 59, 59).unwrap()
    }

    fn start_of_today() -> Self {
        utc_now().date_naive().and_hms_opt(0, 0, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_time_opt;
    use chrono::NaiveDateTime;

    #[test]
    fn test_datetime_start_of_today() {
        let result = NaiveDateTime::start_of_today();
        let actual = get_time_opt(2000, 1, 1, 0, 0, 0);
        assert_eq!(Some(result), actual);
    }

    #[test]
    fn test_end_of_today() {
        let result = NaiveDateTime::end_of_today();
        let actual = get_time_opt(2000, 1, 1, 23, 59, 59);
        assert_eq!(Some(result), actual);
    }
}
