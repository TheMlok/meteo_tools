use crate::constants::constants::{GAS_CONSTANT, KELVIN_DEGREE_CONSTANT, MOLAR_MASS_WATER};
use crate::{fahrenheit_to_celsius, meteo_round, saturation_vapor_pressure};

/// Calculates absolute humidity using Magnus-Tetens formula using Celsius with common atmospheric pressure using constant.
///
/// Returns g/m³
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let result = 0.0871;
///
/// let abs_humidity = meteo_tools::celsius_absolute_humidity(&temperature, &relative_humidity);
///
/// assert_eq!(abs_humidity, result);
/// ```
pub fn celsius_absolute_humidity(temperature: &f64, relative_humidity: &f64) -> f64 {
    let saturation_vapor_pressure = saturation_vapor_pressure(&temperature);

    // Calculate actual vapor pressure
    let actual_vapor_pressure = saturation_vapor_pressure * (relative_humidity / 100.0);

    // Convert actual vapor pressure from mbar to Pa
    let actual_vapor_pressure_pa = actual_vapor_pressure * 100.0;

    let absolute_humidity = (actual_vapor_pressure_pa
        / (GAS_CONSTANT * (*temperature + KELVIN_DEGREE_CONSTANT)))
        * MOLAR_MASS_WATER;

    meteo_round(&absolute_humidity)
}

/// Calculates absolute humidity using Magnus-Tetens formula using Fahrenheit with common atmospheric pressure using constant.
///
/// Returns g/m³
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let result = 0.0871;
///
/// let abs_humidity = meteo_tools::fahrenheit_absolute_humidity(&temperature, &relative_humidity);
///
/// assert_eq!(abs_humidity, result);
/// ```
pub fn fahrenheit_absolute_humidity(temperature: &f64, relative_humidity: &f64) -> f64 {
    let temperature_celsius = fahrenheit_to_celsius(temperature);
    let absolute_humidity = celsius_absolute_humidity(&temperature_celsius, relative_humidity);
    meteo_round(&absolute_humidity)
}
