use crate::constants::constants::KELVIN_DEGREE_CONSTANT;

/// Converts celsius to fahrenheit degrees
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let result = 72.5;
///
/// let fahrenheit = meteo_tools::celsius_to_fahrenheit(&temperature);
///
/// assert_eq!(fahrenheit, result);
/// ```
pub fn celsius_to_fahrenheit(celsius: &f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

/// Converts fahrenheit to celsius degrees
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let result = 22.5;
///
/// let celsius = meteo_tools::fahrenheit_to_celsius(&temperature);
///
/// assert_eq!(celsius, result);
/// ```
pub fn fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Converts Celsius to Kelvin
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let result = 295.65;
///
/// let kelvin = meteo_tools::celsius_to_kelvin(&temperature);
///
/// assert_eq!(kelvin, result);
/// ```
pub fn celsius_to_kelvin(celsius: &f64) -> f64 {
    celsius + KELVIN_DEGREE_CONSTANT
}

/// Converts Kelvin to Celsius
///
/// # Examples
///
/// ```
/// let temperature = 295.65;
/// let result = 22.5;
///
/// let celsius = meteo_tools::kelvin_to_celsius(&temperature);
///
/// assert_eq!(celsius, result);
/// ```
pub fn kelvin_to_celsius(kelvin: &f64) -> f64 {
    kelvin - KELVIN_DEGREE_CONSTANT
}

/// Converts Fahrenheit to Kelvin
///
/// # Examples
///
/// ```
/// let temperature = 72.5;
/// let result = 295.65;
///
/// let kelvin = meteo_tools::fahrenheit_to_kelvin(&temperature);
///
/// assert_eq!(kelvin, result);
/// ```
pub fn fahrenheit_to_kelvin(fahrenheit: &f64) -> f64 {
    let temperature_celsius = fahrenheit_to_celsius(fahrenheit);
    temperature_celsius + KELVIN_DEGREE_CONSTANT
}

/// Converts Kelvin to Celsius
///
/// # Examples
///
/// ```
/// let temperature = 295.65;
/// let result = 72.5;
///
/// let fahrenheit = meteo_tools::kelvin_to_fahrenheit(&temperature);
///
/// assert_eq!(fahrenheit, result);
/// ```
pub fn kelvin_to_fahrenheit(kelvin: &f64) -> f64 {
    let temperature_celsius = kelvin - KELVIN_DEGREE_CONSTANT;
    celsius_to_fahrenheit(&temperature_celsius)
}
