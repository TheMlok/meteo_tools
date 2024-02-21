use crate::constants::constants::{KELVIN_DEGREE_CONSTANT, VOLUME_VAPOR_STD};
use crate::{meteo_round, saturation_vapor_pressure};

/// Calculates absolute humidity using Magnus-Tetens formula using Fahrenheit with common atmospheric pressure using constant.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let result = 87.0602;
///
/// let abs_humidity = meteo_tools::celsius_absolute_humidity(&temperature, &relative_humidity);
///
/// assert_eq!(abs_humidity, result);
/// ```
pub fn celsius_absolute_humidity(temperature: &f64, relative_humidity: &f64) -> f64 {
    // Calculate saturation vapor pressure
    let saturation_vapor_pressure = saturation_vapor_pressure(&temperature);

    // Calculate actual vapor pressure
    let actual_vapor_pressure = saturation_vapor_pressure * (relative_humidity / 100.0);

    // Convert actual vapor pressure to absolute humidity
    let absolute_humidity =
        (actual_vapor_pressure * VOLUME_VAPOR_STD) / (temperature + KELVIN_DEGREE_CONSTANT);
    meteo_round(&absolute_humidity)
}
