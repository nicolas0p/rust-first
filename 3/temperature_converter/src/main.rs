use std::io;
use std::io::Write; //so I can use flush()
use std::str::FromStr;
use std::fmt::Display;

#[derive(PartialEq)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin
}

struct Temperature {
    value: f32,
    unit: TemperatureUnit
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let mut result = String::new();
        let result: &str = match self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
            TemperatureUnit::Kelvin => "Kelvin"
        };
        std::fmt::Display::fmt(result, f)
    }
}

impl FromStr for TemperatureUnit {
    type Err = ();

    fn from_str(input: &str) -> Result<TemperatureUnit, Self::Err> {
        match input {
            "C" => Ok(TemperatureUnit::Celsius),
            "F" => Ok(TemperatureUnit::Fahrenheit),
            "K" => Ok(TemperatureUnit::Kelvin),
            _   => Err(())
        }
    }
}

impl FromStr for Temperature {
    type Err = ();

    fn from_str(input: &str) -> Result<Temperature, Self::Err> {
        //last char determines unit
        let unit = match input.get(input.len()-1..) {
            Some(unit) => unit,
            None => return Err(())
        };
        let unit: TemperatureUnit = match TemperatureUnit::from_str(unit) {
            Ok(unit) => unit,
            Err(()) => return Err(())
        };
        let value = match input.get(..input.len()-1) {
            Some(value) => value,
            None => return Err(())
        };
        let value: f32 = match value.parse() {
            Ok(value) => value,
            Err(e) => {println!("{}", e); return Err(())}
        };
        Ok(Temperature{value, unit})
    }
}

fn main() {
    
    println!("Welcome to temperature converter 5000!");

    let source_temp = get_temperature_from_user();
    let target_unit = get_temperature_unit_from_user();
    let target_temp = transform_temperature(source_temp, target_unit);
    println!("Result: {:.2} {}", target_temp.value, target_temp.unit);
}

fn transform_temperature(source_temp: Temperature, target_unit: TemperatureUnit) -> Temperature {
    let mut target_temp = source_temp;
    while target_temp.unit != target_unit {
        target_temp = match target_temp.unit {
            TemperatureUnit::Celsius => celsius_to_kelvin(target_temp),
            TemperatureUnit::Fahrenheit => fahrenheit_to_celsius(target_temp),
            TemperatureUnit::Kelvin => kelvin_to_fahrenheit(target_temp),
        }
    }
    target_temp
}

fn get_temperature_unit_from_user() -> TemperatureUnit {
    let mut user_input = String::new();

    loop {
        user_input.clear();

        println!("To which unit would you like to convert? C, F or K");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let unit = match TemperatureUnit::from_str(&user_input.trim().to_uppercase()) {
            Ok(unit) => unit,
            Err(_) => {
                println!("No temperature unit related to {user_input}. Options: C, F or K");
                continue;
            },
        };

        break unit;
    }
}

fn get_temperature_from_user() -> Temperature {
    let mut user_input = String::new();

    loop {
        user_input.clear();

        println!("From which temperature and unit would you like to convert? Type Number followed by C, F or K");
        println!("Example: 50C");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let temp = match Temperature::from_str(&user_input.trim().to_uppercase()) {
            Ok(unit) => unit,
            Err(_) => {
                println!("{user_input} is not correctly formated. Options: number followed by C, F or K");
                continue;
            },
        };

        break temp;
    }
}

fn fahrenheit_to_celsius(temperature: Temperature) -> Temperature {
    if temperature.unit != TemperatureUnit::Fahrenheit
    {
        panic!("Incorrect Temperature unit type: Received {}, but should be Fahrenheit"
               , temperature.unit);
    }
    let value = ((temperature.value - 32.0) * 5.0) / 9.0;
    Temperature{value, unit: TemperatureUnit::Celsius}
}

fn celsius_to_kelvin(temperature: Temperature) -> Temperature {
    if temperature.unit != TemperatureUnit::Celsius
    {
        panic!("Incorrect Temperature unit type: Received {}, but should be Celsius"
               , temperature.unit);
    }
    let value = temperature.value + 273.15;
    Temperature{value, unit: TemperatureUnit::Kelvin}
}

fn kelvin_to_fahrenheit(temperature: Temperature) -> Temperature {
    if temperature.unit != TemperatureUnit::Kelvin
    {
        panic!("Incorrect Temperature unit type: Received {}, but should be Kelvin"
               , temperature.unit);
    }
    let value = ((temperature.value - 273.15) * 9.0) / 5.0 + 32.0;
    Temperature{value, unit: TemperatureUnit::Fahrenheit}
}
