use crate::constants::constants::{LATENT_HEAT_OF_VAPORIZATION, STANDARD_CONDENSATION_POINT};
use std::f64;

// TODO: add attribute for degree kind
//       add ability to count in fahrenheits, use crate function to recalculate the temperature
//       add constants somewhere
//       add ablility to set given atm. pressure

/// Calculates dew point using Magnus-Tetens formula
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let humidity = 62.4;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::dew_point(&temperature, &humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn dew_point(temperature: &f64, humidity: &f64) -> f64 {
    let gamma = ((LATENT_HEAT_OF_VAPORIZATION * temperature)
        / (STANDARD_CONDENSATION_POINT + temperature))
        + (humidity / 100.0).ln();
    let dew_point =
        (STANDARD_CONDENSATION_POINT * gamma / (LATENT_HEAT_OF_VAPORIZATION - gamma)).round();
    dew_point
}
