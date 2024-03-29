pub mod constants {
    pub const LATENT_HEAT_OF_VAPORIZATION: f64 = 17.27; // constant related to the latent heat of vaporization of water and the gas constant for water vapor
    pub const STANDARD_CONDENSATION_POINT: f64 = 237.7; // constant related to the temperature at which water vapor starts to condense at standard atmospheric pressure
    pub const SATURATION_VAPOR_PRESSURE_REFERENCE: f64 = 6.112; // from Clausius-Clapeyron equation,
    pub const HUMIDEX_CONSTANT_OFFSET: f64 = 10.0;
    pub const ROTHFUSZ_COEFS: [f64; 10] = [
        // Coeficient for Rothfusz regression equation
        -42.379,
        2.04901523,
        10.14333127,
        -0.22475541,
        -6.83783e-3,
        -5.481717e-2,
        1.22874e-3,
        8.5282e-4,
        -1.99e-6,
        1.040e-8,
    ];
    pub const KELVIN_DEGREE_CONSTANT: f64 = 273.15;
    pub const AVG_ATMOSPHERIC_PRESSURE: f64 = 1013.25;
    //pub const VOLUME_VAPOR_STD: f64 = 216.7; // specific volume of water vapor at standard temperature and pressure
    pub const GAS_CONSTANT: f64 = 8.314; // J/(mol·K)
    pub const MOLAR_MASS_WATER: f64 = 0.0180153; // kg/mol
    pub const MMHG_CONVERSION: f64 = 0.750062;
    pub const INHG_CONVERSION: f64 = 33.8639;
    pub const METRIC_SPEED_CONVERSION: f64 = 3.6;
    pub const IMPERIAL_SPEED_CONVERSION: f64 = 0.621371;
    pub const KNOTS_METRIC_SPEED_CONVERSION: f64 = 1.852;
    pub const KNOTS_IMPERIAL_SPEED_CONVERSION: f64 = 1.15078;
}
