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

   // results
    println!("\n{}{}=", source_distance.n, source_distance.unit.code);

    for target_unit in config.units.iter() {
        if target_unit.code == source_distance.unit.code { continue; }
        
        // TODO: change to 
        // let target_distance = source_distance.convert_to(target_unit);
        let target_distance = target_unit.convert(&source_distance);

        println!("\t{}{}", target_distance.n, target_distance.unit.code);
    }
}
