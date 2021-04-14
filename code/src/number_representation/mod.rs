pub enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

impl Unit {
    pub fn convert_temperature(&self, temperature: &f32) -> f32 {
        match self {
            Unit::Fahrenheit => {
                let temperature = (temperature * 9.0_f32 / 5.0_f32) + 32.0_f32;
                temperature
            }
            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                temperature
            }
            Unit::Celsius => *temperature,
        }
    }
}
