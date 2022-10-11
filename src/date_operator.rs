pub use chrono::{Date, DateTime, TimeZone};

pub trait DateOperator {
    fn years_since(&self, base: Self) -> Option<u32>;
    fn before(&self, other: &Self) -> bool;
    fn after(&self, other: &Self) -> bool;
    fn is_same_day(&self, other: &Self) -> bool;
}

impl<Tz: TimeZone> DateOperator for Date<Tz> {
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
        self == other
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
}
