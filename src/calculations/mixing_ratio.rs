use crate::constants::constants::AVG_ATMOSPHERIC_PRESSURE;
use crate::{fahrenheit_to_celsius, meteo_round, saturation_vapor_pressure};

/// Calculates mixing ratio using Magnus-Tetens formula using Celsius with common atmospheric pressure using constant.
///
/// Returns g/kg
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let result = 0.0826;
///
/// let mixing_ratio = meteo_tools::common_celsius_mixing_ratio(&temperature, &relative_humidity);
///
/// assert_eq!(mixing_ratio, result);
/// ```
pub fn common_celsius_mixing_ratio(temperature: &f64, relative_humidity: &f64) -> f64 {
    let saturation_vapor_pressure = saturation_vapor_pressure(temperature);
    let actual_vapor_pressure = saturation_vapor_pressure * (relative_humidity / 100.0);

    let mixing_ratio =
        0.622 * (actual_vapor_pressure / (AVG_ATMOSPHERIC_PRESSURE - actual_vapor_pressure));
    meteo_round(&mixing_ratio)
}

/// Calculates mixing ratio using Magnus-Tetens formula using Celsius with precise atm. pressure given.
///
/// Returns g/kg
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let atmospheric_pressure = 1013.25;
/// let result = 0.0826;
///
/// let mixing_ratio = meteo_tools::celsius_mixing_ratio(&temperature, &relative_humidity, &atmospheric_pressure);
///
/// assert_eq!(mixing_ratio, result);
/// ```
pub fn celsius_mixing_ratio(
    temperature: &f64,
    relative_humidity: &f64,
    atmospheric_pressure: &f64,
) -> f64 {
    let saturation_vapor_pressure = saturation_vapor_pressure(temperature);
    let actual_vapor_pressure = saturation_vapor_pressure * (relative_humidity / 100.0);

    let mixing_ratio =
        0.622 * (actual_vapor_pressure / (atmospheric_pressure - actual_vapor_pressure));
    meteo_round(&mixing_ratio)
}

/// Calculates mixing ratio using Magnus-Tetens formula using Celsius with common atmospheric pressure using constant.
///
/// Returns g/kg
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let result = 0.0826;
///
/// let mixing_ratio = meteo_tools::common_fahrenheit_mixing_ratio(&temperature, &relative_humidity);
///
/// assert_eq!(mixing_ratio, result);
/// ```
pub fn common_fahrenheit_mixing_ratio(temperature: &f64, relative_humidity: &f64) -> f64 {
    let temperature_celsius = fahrenheit_to_celsius(temperature);
    let mixing_ratio = common_celsius_mixing_ratio(&temperature_celsius, relative_humidity);
    meteo_round(&mixing_ratio)
}

/// Calculates mixing ratio using Magnus-Tetens formula using Celsius with with precise atm. pressure given.
///
/// Returns g/kg
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let atmospheric_pressure = 1013.25;
/// let result = 0.0826;
///
/// let mixing_ratio = meteo_tools::common_fahrenheit_mixing_ratio(&temperature, &relative_humidity);
///
/// assert_eq!(mixing_ratio, result);
/// ```
pub fn fahrenheit_mixing_ratio(
    temperature: &f64,
    relative_humidity: &f64,
    atmospheric_pressure: &f64,
) -> f64 {
    let temperature_celsius = fahrenheit_to_celsius(temperature);
    let mixing_ratio = celsius_mixing_ratio(
        &temperature_celsius,
        relative_humidity,
        atmospheric_pressure,
    );
    meteo_round(&mixing_ratio)
}
