use std::f64;

/// Calculates dew point using Magnus-Tetens formula
///
/// # Examples
///
/// ```
/// let temperature = 22.5;
/// let humidity = 62.4;
/// let result = 15.0;
///
/// let dew_point = meteo_tools::dew_point(temperature, humidity);
///
/// assert_eq!(dew_point, result);
/// ```
pub fn dew_point(temperature: f64, humidity: f64) -> f64 {
    let a = 17.27;
    let b = 237.7;
    let gamma = ((a * temperature) / (b + temperature)) + (humidity / 100.0).ln();
    let dew_point = (b * gamma / (a - gamma)).round();
    dew_point
}
