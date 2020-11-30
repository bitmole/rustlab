//use std::io;
use std::env;
use std::collections::HashMap;

struct Unit {
    label: String,
    base_value: f32,
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
    let distance: f32 = (&args[1]).parse().expect("Please enter a number!");
    let unit_code  = &args[2];

    let source_unit = match unit_map.get(&unit_code[..]) {
        Some(&u) => u,
        None => {
            println!("Unsupported unit '{}'. Aborting.", &unit_code);       
            return;
        }
    };

   // results
    println!("\n{}{}=", distance, source_unit.label);

    for target_unit in units.iter() {
        if target_unit.label == source_unit.label { continue; }
        println!("\t{}{}", 
                 convert(distance, source_unit.base_value, target_unit.base_value),
                 target_unit.label);
    }
}

//fn get_user_input(prompt: &str) -> String {
//    if prompt.trim() != "" {
//        println!("{}", prompt);
//    }
//
//    let mut input = String::new();
//
//    io::stdin()
//        .read_line(&mut input)
//        .expect("Failed to read input");
//
//    input
//} 

fn convert(distance: f32, source: f32, target: f32) -> f32 {
    (distance * source) / target
}
