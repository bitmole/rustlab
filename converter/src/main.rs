//use std::io;
use std::env;
use std::collections::HashMap;

struct Unit {
    label: String,
    base_value: f32,
}

struct Config {
    units: f32,
    unit_code: String,
}

fn main() {

    // init
    let units = vec![
        Unit { label: String::from("mi"), base_value: 160_934.0 },
        Unit { label: String::from("km"), base_value: 100_000.0 },
        Unit { label: String::from("m"), base_value: 100.0 },
        Unit { label: String::from("ft"), base_value: 30.48 },
        Unit { label: String::from("in"), base_value: 2.54 },
        Unit { label: String::from("cm"), base_value: 1.0 },
        Unit { label: String::from("mm"), base_value: 0.1 },
    ];

    let unit_map: HashMap<_, _> = units.iter()
            .map(|item| (item.label.as_str(), item))
            .collect();

    // user input
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let source_unit = match unit_map.get(&config.unit_code[..]) {
        Some(&u) => u,
        None => {
            println!("Unsupported unit '{}'. Aborting.", config.unit_code);       
            return;
        }
    };

   // results
    println!("\n{}{}=", config.units, config.unit_code);

    for target_unit in units.iter() {
        if target_unit.label == source_unit.label { continue; }
        println!("\t{}{}", 
                 convert(&config, &source_unit, &target_unit),
                 target_unit.label);
    }
}

fn parse_config(args: &[String]) -> Config {
    let units: f32 = (&args[1]).parse().expect("Please enter a number!");
    let unit_code = args[2].clone();

    Config { units, unit_code }
}

fn convert(config: &Config, source: &Unit, target: &Unit) -> f32 {
    (config.units * source.base_value) / target.base_value
}
