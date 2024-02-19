# meteo_tools

A crate for counting and converting meteorological units.


This crate can convert and count meteorological values from measurements. Like conversion from C to F and vice versa. Counting
dew point, heat index, humidex, mixing ratio, absolute relative_humidity, and others will be added soon. See the docs for all functionality.

Note that this is common purpose crate. I round values to 4 decimal places, I think it is enough precision for common use.
I am trying to implement a variety of algorithms.

Also I use resolution prefix common_ on algorithms using common constants (ex: dew point using constants instead of exact pressure) to be useful
when you does not have atmospheric pressure measurements. Functions without common_ needs exact measurements.

I created this crate for my API to personal meteo project to count values from measured data sent from Raspberry PIs and Arduinos.# meteo_tools
