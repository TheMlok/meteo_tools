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
