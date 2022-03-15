// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed == 0 {
      return 0.0
    } else if speed > 0 && speed <= 4 {
        return speed as f64 * 221.0;
    } else if speed > 4 && speed <= 8 {
        return speed as f64 * 221.0 * 90.0 / 100.0;
    } else if speed > 8 && speed <= 10 {
        return speed as f64 * 221.0 * 77.0 / 100.0;
    }

    0.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
