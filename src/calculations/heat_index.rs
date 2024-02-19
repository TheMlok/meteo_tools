use crate::constants::constants::ROTHFUSZ_COEFS;
use crate::{celsius_to_fahrenheit, fahrenheit_to_celsius};

/// Calculates heat index based on Rothfusz regression equation for Fahrenheits.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let result = 76.0;
///
/// let heat_index = meteo_tools::fahrenheit_heat_index(&temperature, &relative_humidity);
///
/// assert_eq!(heat_index, result);
/// ```
pub fn fahrenheit_heat_index(temperature: &f64, relative_humidity: &f64) -> f64 {
    let heat_index = ROTHFUSZ_COEFS[0]
        + ROTHFUSZ_COEFS[1] * temperature
        + ROTHFUSZ_COEFS[2] * relative_humidity
        + ROTHFUSZ_COEFS[3] * temperature * relative_humidity
        + ROTHFUSZ_COEFS[4] * temperature.powi(2)
        + ROTHFUSZ_COEFS[5] * relative_humidity.powi(2)
        + ROTHFUSZ_COEFS[6] * temperature.powi(2) * relative_humidity
        + ROTHFUSZ_COEFS[7] * temperature * relative_humidity.powi(2)
        + ROTHFUSZ_COEFS[8] * temperature.powi(2) * relative_humidity.powi(2);
    heat_index.round()
}

/// Calculates heat index based on Rothfusz regression equation for Celsius.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let result = 24.0;
///
/// let heat_index = meteo_tools::celsius_heat_index(&temperature, &relative_humidity);
///
/// assert_eq!(heat_index, result);
/// ```
pub fn celsius_heat_index(temperature: &f64, relative_humidity: &f64) -> f64 {
    let fahrenheit_temperature = celsius_to_fahrenheit(temperature);
    let heat_index = fahrenheit_heat_index(&fahrenheit_temperature, relative_humidity);
    fahrenheit_to_celsius(&heat_index).round()
}
