use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(mut start: DateTime) -> DateTime {
    const BILLION_SECONDS: i64 = 1_000_000_000;
    
    start + Duration::seconds(BILLION_SECONDS)
}
