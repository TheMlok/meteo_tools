use crate::constants::constants::{LATENT_HEAT_OF_VAPORIZATION, STANDARD_CONDENSATION_POINT};
use crate::{celsius_to_fahrenheit, fahrenheit_to_celsius, meteo_round};

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
/// let result = 14.9477;
///
/// let dew_point = meteo_tools::common_celsius_dew_point(&temperature, &relative_humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn common_celsius_dew_point(temperature: &f64, relative_humidity: &f64) -> f64 {
    let gamma = ((LATENT_HEAT_OF_VAPORIZATION * temperature)
        / (STANDARD_CONDENSATION_POINT + temperature))
        + (relative_humidity / 100.0).ln();
    let dew_point = STANDARD_CONDENSATION_POINT * gamma / (LATENT_HEAT_OF_VAPORIZATION - gamma);
    meteo_round(&dew_point)
}

/// Calculates dew point using Magnus-Tetens formula using Celsius with given atmospheric pressure correction in hPa.
/// Needs atmospheric pressure measurement in hPa in f64.
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let relative_humidity = 62.4;
/// let atmospheric_pressure = 1013.25;
/// let result = 14.9477;
///
/// let dew_point = meteo_tools::celsius_dew_point(&temperature, &relative_humidity, &atmospheric_pressure);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn celsius_dew_point(
    temperature: &f64,
    relative_humidity: &f64,
    atmospheric_pressure: &f64,
) -> f64 {
    let dew_point = common_celsius_dew_point(&temperature, &relative_humidity);
    calculate_exact_pressure_offset(&atmospheric_pressure, &dew_point)
}

/// Calculates common dew point using Magnus-Tetens formula using Fahrenheit with common atmospheric pressure using constant.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let result = 58.9059;
///
/// let dew_point = meteo_tools::common_fahrenheit_dew_point(&temperature, &relative_humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn common_fahrenheit_dew_point(temperature: &f64, relative_humidity: &f64) -> f64 {
    let temperature = fahrenheit_to_celsius(temperature);
    let dew_point =
        celsius_to_fahrenheit(&common_celsius_dew_point(&temperature, relative_humidity));
    meteo_round(&dew_point)
}

/// Calculates dew point using Magnus-Tetens formula using Fahrenheit with given atmospheric pressure correction in hPa.
/// Needs atmospheric pressure measurement in hPa in f64.
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let relative_humidity = 62.4;
/// let atmospheric_pressure = 1013.25;
/// let result = 58.9059;
///
/// let dew_point = meteo_tools::fahrenheit_dew_point(&temperature, &relative_humidity, &atmospheric_pressure);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn fahrenheit_dew_point(
    temperature: &f64,
    relative_humidity: &f64,
    atmospheric_pressure: &f64,
) -> f64 {
    let dew_point = common_fahrenheit_dew_point(&temperature, &relative_humidity);
    calculate_exact_pressure_offset(&atmospheric_pressure, &dew_point)
}

fn calculate_exact_pressure_offset(atmospheric_pressure: &f64, dew_point: &f64) -> f64 {
    if *atmospheric_pressure != 1013.25 {
        let dew_point_full =
            dew_point / (1.0 - (atmospheric_pressure - 1013.25) / 1013.25 * 0.190284).abs();
        meteo_round(&dew_point_full)
    } else {
        meteo_round(&dew_point)
    }
}
