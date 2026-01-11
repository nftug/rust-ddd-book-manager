use chrono::{DateTime, Utc};
use derive_new::new;

pub trait Clock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

pub struct SystemClock;

impl Clock for SystemClock {}

#[derive(new)]
pub struct FixedClock {
    fixed_time: DateTime<Utc>,
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.fixed_time
    }
}
