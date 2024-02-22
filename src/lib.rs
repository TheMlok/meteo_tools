//! # meteo_tools
//!
//! A crate for counting and converting meteorological units.
//!
//! This crate can convert and count meteorological values from measurements. Like conversion from C to F and Kelvin and vice versa. Counting
//! dew point, heat index, humidex, mixing ratio, absolute humidity, and others will be added soon. See the docs for all functionality.

mod calculations;
mod constants;
pub mod conversions;

use crate::constants::constants::{
    HUMIDEX_CONSTANT_OFFSET, LATENT_HEAT_OF_VAPORIZATION, SATURATION_VAPOR_PRESSURE_REFERENCE,
    STANDARD_CONDENSATION_POINT,
};
// Dew point
pub use self::calculations::dew_point::celsius_dew_point;
pub use self::calculations::dew_point::common_celsius_dew_point;
pub use self::calculations::dew_point::common_fahrenheit_dew_point;
pub use self::calculations::dew_point::fahrenheit_dew_point;

// Heat index
pub use self::calculations::heat_index::celsius_heat_index;
pub use self::calculations::heat_index::fahrenheit_heat_index;

// Humidex
pub use self::calculations::humidex::celsius_humidex;
pub use self::calculations::humidex::common_celsius_humidex;
pub use self::calculations::humidex::common_fahrenheit_humidex;
pub use self::calculations::humidex::fahrenheit_humidex;

// Mixing ratio
pub use self::calculations::mixing_ratio::celsius_mixing_ratio;
pub use self::calculations::mixing_ratio::common_celsius_mixing_ratio;
pub use self::calculations::mixing_ratio::common_fahrenheit_mixing_ratio;
pub use self::calculations::mixing_ratio::fahrenheit_mixing_ratio;

// Absolute humidity
pub use self::calculations::absolute_humidity::celsius_absolute_humidity;
pub use self::calculations::absolute_humidity::fahrenheit_absolute_humidity;

// Conversions
// temperature
pub use self::conversions::temperature::celsius_to_fahrenheit;
pub use self::conversions::temperature::celsius_to_kelvin;
pub use self::conversions::temperature::fahrenheit_to_celsius;
pub use self::conversions::temperature::fahrenheit_to_kelvin;
pub use self::conversions::temperature::kelvin_to_celsius;
pub use self::conversions::temperature::kelvin_to_fahrenheit;
// pressure
pub use self::conversions::pressure::hpa_to_inhg;
pub use self::conversions::pressure::hpa_to_mmhg;
pub use self::conversions::pressure::inhg_to_hpa;
pub use self::conversions::pressure::mmhg_to_hpa;
// speed
pub use self::conversions::wind_speed::kmph_to_mps;
pub use self::conversions::wind_speed::mps_to_kmph;

// TODO: Specific relative_humidity, Vapor pressure, Relative saturation
// TODO: heat index variants: Steadman's Apparent Temperature, Australian Apparent Temperature, New Zealand Apparent Temperature
// TODO: add accessivle constants for different measurements and values

/// Rounds given value to 4 decimal places
fn meteo_round(number: &f64) -> f64 {
    (number * 10000.0).round() / 10000.0
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
