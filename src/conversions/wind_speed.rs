use crate::constants::constants::{
    IMPERIAL_SPEED_CONVERSION, KNOTS_IMPERIAL_SPEED_CONVERSION, KNOTS_METRIC_SPEED_CONVERSION,
    METRIC_SPEED_CONVERSION,
};
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

/// Converts speed from Kilometers per hour to Miles per hour.
///
/// # Examples
///
/// ```
/// let speed = 36.0;
/// let result = 22.3694;
///
/// let mps = meteo_tools::kmph_to_mph(&speed);
///
/// assert_eq!(mps, result);
/// ```
pub fn kmph_to_mph(speed_kmph: &f64) -> f64 {
    let mph = speed_kmph * IMPERIAL_SPEED_CONVERSION;
    meteo_round(&mph)
}

/// Converts speed from Miles per hour to Kilometers per hour.
///
/// # Examples
///
/// ```
/// let speed = 22.3694;
/// let result = 36.0001;
///
/// let mps = meteo_tools::mph_to_kmph(&speed);
///
/// assert_eq!(mps, result);
/// ```
pub fn mph_to_kmph(speed_mph: &f64) -> f64 {
    let kmph = speed_mph / IMPERIAL_SPEED_CONVERSION;
    meteo_round(&kmph)
}

// TODO mph to m/s

/// Converts speed from Meters per second to Miles per hour.
///
/// # Examples
///
/// ```
/// let speed = 10.0;
/// let result = 22.3694;
///
/// let mph = meteo_tools::mps_to_mph(&speed);
///
/// assert_eq!(mph, result);
/// ```
pub fn mps_to_mph(speed_mps: &f64) -> f64 {
    let kmph = mps_to_kmph(speed_mps);
    let mph = kmph_to_mph(&kmph);
    meteo_round(&mph)
}

// TODO knots to kmph knots to mph, m/s to knots, knots to m/s

/// Converts speed from Kilometers per hour to Knots per hour.
///
/// # Examples
///
/// ```
/// let speed = 10.0;
/// let result = 5.3996;
///
/// let kts = meteo_tools::kmph_to_knots(&speed);
///
/// assert_eq!(kts, result);
/// ```
pub fn kmph_to_knots(speed_kmph: &f64) -> f64 {
    let kts = speed_kmph / KNOTS_METRIC_SPEED_CONVERSION;
    meteo_round(&kts)
}

/// Converts speed from Knots per hour to Kilometers per hour.
///
/// # Examples
///
/// ```
/// let speed = 5.3996;
/// let result = 10.0001;
///
/// let kts = meteo_tools::knots_to_kmph(&speed);
///
/// assert_eq!(kts, result);
/// ```
pub fn knots_to_kmph(speed_knots: &f64) -> f64 {
    let kmh = speed_knots * KNOTS_METRIC_SPEED_CONVERSION;
    meteo_round(&kmh)
}

/// Converts speed from Miles per hour to Knots per hour.
///
/// # Examples
///
/// ```
/// let speed = 22.3694;
/// let result = 19.4385
/// ;
///
/// let kts = meteo_tools::mph_to_knots(&speed);
///
/// assert_eq!(kts, result);
/// ```
pub fn mph_to_knots(speed_mph: &f64) -> f64 {
    let kts = speed_mph / KNOTS_IMPERIAL_SPEED_CONVERSION;
    meteo_round(&kts)
}
