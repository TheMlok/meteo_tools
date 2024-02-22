# meteo_tools

A crate for counting and converting meteorological units. Accepts temperature in C or F, relative humidity and optionally
atmospheric pressure.

## Features

### Calculations

* Dew point
* Heat index
* Humidex
* Mixing ratio
* Absolute humidity

### Conversions
* Between Celsius, Fahrenheit and Kelvin
* Between hPa to mmHg and inHg
* Wind speed km/h, m/s

Conversions are interchangeably, aka from Celsius to Fahrenheit and vice versa.

More will be coming soon. See the documentation.

Using meteorological algorithms as Magnus-Tetens formula, Clausius-Clapeyron equation and Rothfusz regression equation with 
standard constants. When working with atmospheric pressure (humidex etc.) are available functions whether you have atmospheric 
pressure measurements or uses constants.

Note that this is common purpose crate. I round values to 4 decimal places, I think it is enough precision for common use.
I am trying to implement a variety of algorithms. Also, this crate is maybe not suitable for exact laboratory measurements,
because values of constants for algorithms differ by different sources, you may need another constant for your application.

Also, I use resolution prefix `common_` on algorithms using common constants (ex: dew point using constants instead of exact pressure) to be useful
when you do not have atmospheric pressure measurements. Functions without `common_` needs exact measurements.

I created this crate for my API to personal meteo project to count values from measured data sent from Raspberry PIs and Arduinos.# meteo_tools

## Example of usage

It is simple and fun, let's enjoy :).

```rust
use meteo_tools::common_celsius_dew_point;

fn main() {
    let temperature = 22.5;
    let relative_humidity = 62.4;
    
    // returns dew point in f64
    let dew_point = common_celsius_dew_point(&temperature, &humidity);
}
```

Or converting degrees units.

```rust 
use meteo_tools::celsius_to_fahrenheit;

fn main() {
    let celsius_temperature = 22.5;
    
    // Converts units to another scale
    let fahrenheit_temperature = celsius_to_fahrenheit(&celsius_temperature);
}
```
