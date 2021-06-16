/// normalized time must be in 0..1
#[inline(always)]
pub fn check_time(time: f64) -> f64 {
    debug_assert!(time >= 0.0 || time <= 1.0);
    time
}
