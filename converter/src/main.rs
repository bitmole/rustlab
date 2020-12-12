use std::env;
use std::process;

use converter::Config;
use converter::Distance;

fn main() {

    let config = Config::new();

    let source_distance = Distance::new(env::args(), &config.units)
            .unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {}", err);
                process::exit(1);
            });

    println!("\n{} =", source_distance);
    for target_unit in config.units.iter() {
        if target_unit.is_same_as(source_distance.unit) { continue; }

        println!("\t{}", source_distance.convert_to(target_unit));
    }
}
