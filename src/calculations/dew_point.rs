use crate::constants::constants::{LATENT_HEAT_OF_VAPORIZATION, STANDARD_CONDENSATION_POINT};
use crate::fahrenheit_to_celsius;
use std::f64;

// TODO: add attribute for degree kind
//       add ability to count in fahrenheits, use crate function to recalculate the temperature
//       add ablility to set given atm. pressure

/// Calculates dew point using Magnus-Tetens formula using Celsius with common atmospheric pressure using constant.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::common_celsius_dew_point(&temperature, &relative_humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn common_celsius_dew_point(temperature: &f64, relative_humidity: &f64) -> f64 {
    let gamma = ((LATENT_HEAT_OF_VAPORIZATION * temperature)
        / (STANDARD_CONDENSATION_POINT + temperature))
        + (relative_humidity / 100.0).ln();
    (STANDARD_CONDENSATION_POINT * gamma / (LATENT_HEAT_OF_VAPORIZATION - gamma)).round()
}

/// Calculates dew point using Magnus-Tetens formula using Celsius with given atmospheric pressure correction in hPa.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let pressure = 1013.25;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::celsius_dew_point(&temperature, &relative_humidity, &pressure);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn celsius_dew_point(temperature: &f64, relative_humidity: &f64, pressure: &f64) -> f64 {
    let dew_point = common_celsius_dew_point(&temperature, &relative_humidity);
    if *pressure != 1013.25 {
        dew_point
            / (1.0 - (pressure - 1013.25) / 1013.25 * 0.190284)
                .abs()
                .round()
    } else {
        dew_point.round()
    }
}

/// Calculates common dew point using Magnus-Tetens formula using Fahrenheit with common atmospheric pressure using constant.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::common_fahrenheit_dew_point(&temperature, &relative_humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn common_fahrenheit_dew_point(temperature: &f64, relative_humidity: &f64) -> f64 {
    let temperature = fahrenheit_to_celsius(temperature);
    common_celsius_dew_point(&temperature, relative_humidity)
}

/// Calculates dew point using Magnus-Tetens formula using Fahrenheit with given atmospheric pressure correction in hPa.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let pressure = 1013.25;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::fahrenheit_dew_point(&temperature, &relative_humidity, &pressure);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn fahrenheit_dew_point(temperature: &f64, relative_humidity: &f64, pressure: &f64) -> f64 {
    let dew_point = common_fahrenheit_dew_point(&temperature, &relative_humidity);
    if *pressure != 1013.25 {
        dew_point
            / (1.0 - (pressure - 1013.25) / 1013.25 * 0.190284)
                .abs()
                .round()
    } else {
        dew_point.round()
    }
}
