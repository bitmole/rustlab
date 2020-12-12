use std::env;
use std::fmt;

pub struct Unit {
    code: String,
    base_value: f32,
    label: String,
}

impl Unit {
    pub fn is_same_as(&self, other: &Unit) -> bool {
        self.code == other.code 
    }
}

pub struct Distance<'a> {
    n: f32,
    pub unit: &'a Unit,
}

impl Distance<'_> {
    pub fn new(mut args: env::Args, supported_units: &[Unit]) -> Result<Distance, &'static str> {
        args.next(); // program name

        let n: f32 = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Didn't get number of units.")
        };

        let unit_code = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get unit code.")
        };

        let unit = match supported_units
                    .iter()
                    .filter(|u| u.code == unit_code)
                    .next() {
            Some(u) => u,
            None => return Err("Unsupported unit. Aborting.")
        };

        Ok(Distance { n: n, unit: unit })
    }

    pub fn convert_to<'a>(&self, target: &'a Unit) -> Distance<'a> {
        Distance {
            n: (self.n * self.unit.base_value) / target.base_value,
            unit: target
        }
    }
}

impl fmt::Display for Distance<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(format!("{} {}", self.n, self.unit.label).as_str())?;
        Ok(())
    }
}

pub struct Config {
    pub units: Vec<Unit>
}

impl Config {
    pub fn new() -> Config {
        Config {
            units: vec![
                Unit { code: String::from("mi"), label: String::from("miles"), base_value: 160_934.0 },
                Unit { code: String::from("km"), label: String::from("kilometers"), base_value: 100_000.0 },
                Unit { code: String::from("m"), label: String::from("meters"), base_value: 100.0 },
                Unit { code: String::from("ft"), label: String::from("feet"), base_value: 30.48 },
                Unit { code: String::from("in"), label: String::from("inches"), base_value: 2.54 },
                Unit { code: String::from("cm"), label: String::from("centimeters"), base_value: 1.0 },
                Unit { code: String::from("mm"), label: String::from("millimeters"), base_value: 0.1 },
            ]
        }
    }
}

