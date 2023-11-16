use std::ops::Deref;

use chrono::{FixedOffset, NaiveDateTime, Utc};

/// 当前时间
pub struct Now;

#[derive(Clone, Copy, Debug)]
pub struct ZoneNum(i32);

impl ZoneNum {
    pub fn new(value: i32) -> Self {
        assert!(value > -12 && value < 12);
        Self(value)
    }
}

impl Deref for ZoneNum {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i32> for ZoneNum {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

/// 时区属性
pub enum ZoneType {
    /// 东部时区
    East,
    /// 西部时区
    West,
}

enum Timestamp {
    Micro,
    Nano,
    Milli,
    Second,
}

impl Now {
    /// 以UTC形式显示当前时间
    pub fn utc() -> NaiveDateTime {
        Utc::now().naive_local()
    }

    /// 以当前时区显示当前时间
    ///
    /// # 参数
    /// - n 所属时间
    /// - zone 时区属性，东部时区/西部时区
    ///
    pub fn local(n: ZoneNum, zone: ZoneType) -> NaiveDateTime {
        let an_hour = 60 * 60;
        let offset_seconds = *n * an_hour;
        let offset = match zone {
            ZoneType::East => FixedOffset::east_opt(offset_seconds).unwrap(),
            ZoneType::West => FixedOffset::west_opt(offset_seconds).unwrap(),
        };
        Utc::now().with_timezone(&offset).naive_local()
    }

    fn timestamp_utc(time_type: Timestamp) -> i64 {
        match time_type {
            Timestamp::Micro => Utc::now().timestamp_micros(),
            Timestamp::Nano => Utc::now().timestamp_nanos(),
            Timestamp::Milli => Utc::now().timestamp_millis(),
            Timestamp::Second => Utc::now().timestamp(),
        }
    }

    /// 以时间戳的形式表示当前时间
    pub fn timestamp() -> i64 {
        Self::timestamp_utc(Timestamp::Second)
    }

    /// 以纳秒为单位的时间戳表示当前时间
    pub fn timestamp_nanos() -> i64 {
        Self::timestamp_utc(Timestamp::Nano)
    }

    /// 以微秒为单位的时间戳表示当前时间
    pub fn timestamp_micros() -> i64 {
        Self::timestamp_utc(Timestamp::Micro)
    }

    /// 以毫秒为单位的时间戳表示当前时间
    pub fn timestamp_millis() -> i64 {
        Self::timestamp_utc(Timestamp::Milli)
    }

    fn timestamp_with_local<T>(n: T, zone_type: ZoneType, time_type: Timestamp) -> i64
    where
        T: Into<ZoneNum>,
    {
        let n = n.into();
        let time = Self::local(n, zone_type);
        match time_type {
            Timestamp::Micro => time.timestamp_micros(),
            Timestamp::Nano => time.timestamp_nanos(),
            Timestamp::Milli => time.timestamp_millis(),
            Timestamp::Second => time.timestamp(),
        }
    }

    /// 以时间戳的形式表示当地当前时间
    pub fn timestamp_local<T>(n: T, zone_type: ZoneType) -> i64
    where
        T: Into<ZoneNum>,
    {
        Self::timestamp_with_local(n, zone_type, Timestamp::Second)
    }
    /// 以纳秒为单位时间戳的形式表示当地当前时间
    pub fn timestamp_local_nanos<T>(n: T, zone_type: ZoneType) -> i64
    where
        T: Into<ZoneNum>,
    {
        Self::timestamp_with_local(n, zone_type, Timestamp::Nano)
    }
    /// 以毫秒为单位时间戳的形式表示当地当前时间
    pub fn timestamp_local_milli<T>(n: T, zone_type: ZoneType) -> i64
    where
        T: Into<ZoneNum>,
    {
        Self::timestamp_with_local(n, zone_type, Timestamp::Milli)
    }
    /// 以微秒为单位时间戳的形式表示当地当前时间
    pub fn timestamp_local_micro<T>(n: T, zone_type: ZoneType) -> i64
    where
        T: Into<ZoneNum>,
    {
        Self::timestamp_with_local(n, zone_type, Timestamp::Micro)
    }
}
