use crate::constants::constants::HUMIDEX_CONSTANT_OFFSET;
use crate::{
    celsius_dew_point, common_celsius_dew_point, common_fahrenheit_dew_point, fahrenheit_dew_point,
    meteo_round,
};

/// Counts humidex for Celsius from given values. Uses common dew point algorithm.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let result = 25.2487;
///
/// let celsius = meteo_tools::common_celsius_humidex(&temperature, &relative_humidity);
///
/// assert_eq!(celsius, result);
/// ```
pub fn common_celsius_humidex(temperature: &f64, relative_humidity: &f64) -> f64 {
    let dew_point = common_celsius_dew_point(temperature, relative_humidity);
    let humidex = *temperature + (5.0 / 9.0) * (dew_point - HUMIDEX_CONSTANT_OFFSET);
    meteo_round(&humidex)
}

/// Counts humidex for Celsius from given values. Uses Celsius with given atmospheric pressure correction in hPa.
/// Needs atmospheric pressure measurement in hPa in f64.
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let atmospheric_pressure = 1013.25;
/// let result = 25.2487;
///
/// let celsius = meteo_tools::celsius_humidex(&temperature, &relative_humidity, &atmospheric_pressure);
///
/// assert_eq!(celsius, result);
/// ```
pub fn celsius_humidex(
    temperature: &f64,
    relative_humidity: &f64,
    atmospheric_pressure: &f64,
) -> f64 {
    let dew_point = celsius_dew_point(temperature, relative_humidity, atmospheric_pressure);
    let humidex = *temperature + (5.0 / 9.0) * (dew_point - HUMIDEX_CONSTANT_OFFSET);
    meteo_round(&humidex)
}

/// Counts humidex for Fahrenheit from given values. Uses common dew point algorithm.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let result = 99.6699;
///
/// let fahrenheit = meteo_tools::common_fahrenheit_humidex(&temperature, &relative_humidity);
///
/// assert_eq!(fahrenheit, result);
/// ```
pub fn common_fahrenheit_humidex(temperature: &f64, relative_humidity: &f64) -> f64 {
    let dew_point = common_fahrenheit_dew_point(temperature, relative_humidity);
    let humidex = *temperature + (5.0 / 9.0) * (dew_point - HUMIDEX_CONSTANT_OFFSET);
    meteo_round(&humidex)
}

/// Counts humidex for Fahrenheit from given values. Uses Fahrenheit with given atmospheric pressure correction in hPa.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let atmospheric_pressure = 1013.25;
/// let result = 99.6699;
///
/// let fahrenheit = meteo_tools::fahrenheit_humidex(&temperature, &relative_humidity, &atmospheric_pressure);
///
/// assert_eq!(fahrenheit, result);
/// ```
pub fn fahrenheit_humidex(
    temperature: &f64,
    relative_humidity: &f64,
    atmospheric_pressure: &f64,
) -> f64 {
    let dew_point = fahrenheit_dew_point(temperature, relative_humidity, atmospheric_pressure);
    let humidex = *temperature + (5.0 / 9.0) * (dew_point - HUMIDEX_CONSTANT_OFFSET);
    meteo_round(&humidex)
}
