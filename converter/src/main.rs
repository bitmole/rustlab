use std::env;
use std::process;

struct Unit {
    code: String,
    base_value: f32,
}

impl Unit {
    fn convert(&self, distance: &Distance) -> Distance {
        Distance { 
            n: (distance.n * distance.unit.base_value) / self.base_value, 
            unit: self 
        }
    }
}

struct Distance<'a> {
    n: f32,
    unit: &'a Unit,
}

impl Distance<'_> {
    fn new(mut args: env::Args, supported_units: &[Unit]) -> Result<Distance, &'static str> {
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
}

fn main() {

    // init
    let units = vec![
        Unit { code: String::from("mi"), base_value: 160_934.0 },
        Unit { code: String::from("km"), base_value: 100_000.0 },
        Unit { code: String::from("m"), base_value: 100.0 },
        Unit { code: String::from("ft"), base_value: 30.48 },
        Unit { code: String::from("in"), base_value: 2.54 },
        Unit { code: String::from("cm"), base_value: 1.0 },
        Unit { code: String::from("mm"), base_value: 0.1 },
    ];

    let source_distance = Distance::new(env::args(), &units).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

   // results
    println!("\n{}{}=", source_distance.n, source_distance.unit.code);

    for target_unit in units.iter() {
        if target_unit.code == source_distance.unit.code { continue; }
        
        let target_distance = target_unit.convert(&source_distance);

        println!("\t{}{}", target_distance.n, target_distance.unit.code);
    }
}
