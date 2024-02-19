use crate::constants::constants::{LATENT_HEAT_OF_VAPORIZATION, STANDARD_CONDENSATION_POINT};
use crate::fahrenheit_to_celsius;
use std::f64;

// TODO: add attribute for degree kind
//       add ability to count in fahrenheits, use crate function to recalculate the temperature
//       add ablility to set given atm. pressure

/// Calculates dew point using Magnus-Tetens formula using Celsius with common atmospheric pressure.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let humidity = 62.4;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::celsius_dew_point(&temperature, &humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn celsius_dew_point(temperature: &f64, humidity: &f64) -> f64 {
    let gamma = ((LATENT_HEAT_OF_VAPORIZATION * temperature)
        / (STANDARD_CONDENSATION_POINT + temperature))
        + (humidity / 100.0).ln();
    (STANDARD_CONDENSATION_POINT * gamma / (LATENT_HEAT_OF_VAPORIZATION - gamma)).round()
}

/// Calculates dew point using Magnus-Tetens formula using Fahrenheit with common atmospheric pressure.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let humidity = 62.4;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::fahrenheit_dew_point(&temperature, &humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn fahrenheit_dew_point(temperature: &f64, humidity: &f64) -> f64 {
    let temperature = fahrenheit_to_celsius(temperature);
    celsius_dew_point(&temperature, humidity)
}
