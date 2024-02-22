use crate::constants::constants::{INHG_CONVERSION, MMHG_CONVERSION};
use crate::meteo_round;

/// Converts hPa to mmHg.
///
/// # Examples
///
/// ```
/// let pressure = 1013.25;
/// let result = 760.0003;
///
/// let mmhg = meteo_tools::hpa_to_mmhg(&pressure);
///
/// assert_eq!(mmhg, result);
/// ```
pub fn hpa_to_mmhg(hpa: &f64) -> f64 {
    let mmhg = hpa * MMHG_CONVERSION;
    meteo_round(&mmhg)
}

/// Converts mmHg to hPa.
///
/// # Examples
///
/// ```
/// let pressure = 760.0003;
/// let result = 1013.25;
///
/// let hpa = meteo_tools::mmhg_to_hpa(&pressure);
///
/// assert_eq!(hpa, result);
/// ```
pub fn mmhg_to_hpa(mmhg: &f64) -> f64 {
    let hpa = mmhg / MMHG_CONVERSION;
    meteo_round(&hpa)
}

/// Converts hPa to inHg.
///
/// # Examples
///
/// ```
/// let pressure = 1013.25;
/// let result = 29.9212;
///
/// let inhg = meteo_tools::hpa_to_inhg(&pressure);
///
/// assert_eq!(inhg, result);
/// ```
pub fn hpa_to_inhg(hpa: &f64) -> f64 {
    let inhg = hpa / INHG_CONVERSION;
    meteo_round(&inhg)
}

/// Converts inHg to hPa.
///
/// # Examples
///
/// ```
/// let pressure = 29.9212;
/// let result = 1013.2485;
///
/// let hpa = meteo_tools::inhg_to_hpa(&pressure);
///
/// assert_eq!(hpa, result);
/// ```
pub fn inhg_to_hpa(inhg: &f64) -> f64 {
    let hpa = inhg * INHG_CONVERSION;
    meteo_round(&hpa)
}
