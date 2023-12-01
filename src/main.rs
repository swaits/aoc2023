pub mod utils;

// HACK: include build.rs generated file which declares modules for every dayXX.rs file found
include!("days_mod.inc");

// a struct for a single day's implementation, includes its day number and the function to run
#[derive(Debug)]
pub struct Day {
    pub number: usize,
    pub run: fn(),
}

// collects all the days registered in the inventory
inventory::collect!(Day);

// main takes one argument: 'all' or a day number
//
// if no argument is provided, runs the latest day
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("all") => run_all_days(),
        Some(day_str) if day_str.parse::<usize>().is_ok() => {
            run_specific_day(day_str.parse::<usize>().unwrap())
        }
        None => run_latest_day(),
        _ => println!("Invalid input. Please specify a day or 'all'."),
    }
}

// runs the day's function, if it exists
fn run_day(day_option: Option<&Day>) {
    match day_option {
        Some(day_module) => {
            println!("Running Day: {}", day_module.number);
            (day_module.run)();
        }
        None => println!("Day is not implemented yet."),
    }
}

// runs the day with the given number
fn run_specific_day(day: usize) {
    let day_module = inventory::iter::<Day>().find(|d| d.number == day);
    run_day(day_module);
}

// runs all the days in the inventory
fn run_all_days() {
    for day_module in inventory::iter::<Day>() {
        run_day(Some(day_module));
    }
}

// run the day in the inventory with the highest number
fn run_latest_day() {
    let latest_day = inventory::iter::<Day>().max_by_key(|d| d.number);
    run_day(latest_day);
}
