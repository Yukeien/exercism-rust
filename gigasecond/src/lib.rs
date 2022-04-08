use time::PrimitiveDateTime as DateTime;
use time::Duration;

pub fn after(start: DateTime) -> DateTime {
    let delay: u32 = 1000000000;

    match start.checked_add(Duration::new(delay as i64, 0)) {
        Some(date) => date,
        None => start
    }
}
