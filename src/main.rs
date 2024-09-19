struct CelsiusThermometer;

impl CelsiusThermometer {
    pub fn get_temperature(&self) -> f64 {
        25.0
    }
}

trait FahrenheitThermometer {
    fn get_temperature_in_fahrenheit(&self) -> f64;
}

struct CelsiusToFahrenheitAdapter {
    celsius_thermometer: CelsiusThermometer,
}

impl CelsiusToFahrenheitAdapter {
    pub fn new(thermometer: CelsiusThermometer) -> Self {
        Self {
            celsius_thermometer: thermometer,
        }
    }
}

impl FahrenheitThermometer for CelsiusToFahrenheitAdapter {
    fn get_temperature_in_fahrenheit(&self) -> f64 {
        let celsius = self.celsius_thermometer.get_temperature();
        celsius * 9.0 / 5.0 + 32.0
    }
}

fn main() {
    let celsius_thermometer = CelsiusThermometer;

    let adapter = CelsiusToFahrenheitAdapter::new(celsius_thermometer);

    println!(
        "Temperature in Fahrenheit: {:.2}",
        adapter.get_temperature_in_fahrenheit()
    );
}
