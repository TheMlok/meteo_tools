mod calculations;
mod constants;
mod conversions;

// Dew point
pub use self::calculations::dew_point::celsius_dew_point;
pub use self::calculations::dew_point::common_celsius_dew_point;
pub use self::calculations::dew_point::common_fahrenheit_dew_point;
pub use self::calculations::dew_point::fahrenheit_dew_point;

// Heat index
pub use self::calculations::heat_index::celsius_heat_index;
pub use self::calculations::heat_index::fahrenheit_heat_index;

// Conversions
pub use self::conversions::temperature::celsius_to_fahrenheit;
pub use self::conversions::temperature::fahrenheit_to_celsius;

// TODO: heat index, humidex, mixing ratio, absolute relative_humidity, Specific relative_humidity, Mixing Ratio, Absolute relative_humidity, Vapor pressure, Relative saturation
