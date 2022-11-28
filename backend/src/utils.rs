use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_timestamp_8_hours_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let eight_hours_from_now = since_the_epoch + Duration::from_secs(60 * 8 * 60);
    eight_hours_from_now.as_secs()
}
