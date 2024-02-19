mod calculations;
mod constants;
pub mod conversions;

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
pub use self::calculations::mixing_ratio::common_celsius_mixing_ratio;
pub use self::calculations::mixing_ratio::common_fahrenheit_mixing_ratio;

// Conversions
pub use self::conversions::temperature::celsius_to_fahrenheit;
pub use self::conversions::temperature::celsius_to_kelvin;
pub use self::conversions::temperature::fahrenheit_to_celsius;
pub use self::conversions::temperature::fahrenheit_to_kelvin;
pub use self::conversions::temperature::kelvin_to_celsius;
pub use self::conversions::temperature::kelvin_to_fahrenheit;

// TODO: Specific relative_humidity, Mixing Ratio, Absolute relative_humidity, Vapor pressure, Relative saturation
// TODO: heat index variants: Steadman's Apparent Temperature, Australian Apparent Temperature, New Zealand Apparent Temperature

/// Rounds given value to 4 decimal places
fn meteo_round(number: &f64) -> f64 {
    (number * 10000.0).round() / 10000.0
}
