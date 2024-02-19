pub mod constants {
    pub const LATENT_HEAT_OF_VAPORIZATION: f64 = 17.27; // constant related to the latent heat of vaporization of water and the gas constant for water vapor
    pub const STANDARD_CONDENSATION_POINT: f64 = 237.7; // constant related to the temperature at which water vapor starts to condense at standard atmospheric pressure
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
}
