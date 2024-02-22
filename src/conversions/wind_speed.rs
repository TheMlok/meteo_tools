use crate::constants::constants::METRIC_SPEED_CONVERSION;
use crate::meteo_round;

/// Converts speed from Meters per second to Kilometers per hour.
///
/// # Examples
///
/// ```
/// let speed = 10.0;
/// let result = 36.0;
///
/// let kmph = meteo_tools::mps_to_kmph(&speed);
///
/// assert_eq!(kmph, result);
/// ```
pub fn mps_to_kmph(speed_mps: &f64) -> f64 {
    let kmph = speed_mps * METRIC_SPEED_CONVERSION;
    meteo_round(&kmph)
}

/// Converts speed from Kilometers per second to Meters per hour.
///
/// # Examples
///
/// ```
/// let speed = 36.0;
/// let result = 10.0;
///
/// let mps = meteo_tools::kmph_to_mps(&speed);
///
/// assert_eq!(mps, result);
/// ```
pub fn kmph_to_mps(speed_kmph: &f64) -> f64 {
    let mps = speed_kmph / METRIC_SPEED_CONVERSION;
    meteo_round(&mps)
}
