use crate::constants::constants::{
    AVG_ATMOSPHERIC_PRESSURE, HUMIDEX_CONSTANT_OFFSET, LATENT_HEAT_OF_VAPORIZATION,
    SATURATION_VAPOR_PRESSURE_REFERENCE, STANDARD_CONDENSATION_POINT,
};
use crate::{fahrenheit_to_celsius, meteo_round};

// TODO change this AVG_ATMOSPHERIC_PRESSURE with attribute to create other than common

/// Calculates mixing ratio using Magnus-Tetens formula using Celsius with common atmospheric pressure using constant.
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

/// Calculates mixing ratio using Magnus-Tetens formula using Celsius with common atmospheric pressure using constant.
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

// Function to calculate saturation vapor pressure using the Magnus-Tetens formula
fn saturation_vapor_pressure(temperature_celsius: &f64) -> f64 {
    let saturation_vapor_pressure = SATURATION_VAPOR_PRESSURE_REFERENCE
        * HUMIDEX_CONSTANT_OFFSET.powf(
            (LATENT_HEAT_OF_VAPORIZATION * temperature_celsius)
                / (STANDARD_CONDENSATION_POINT + temperature_celsius),
        );
    saturation_vapor_pressure
}
