use std::process;
use std::env;

#[path="./01-calorie.rs"]
mod calorie;
#[path="./02-rock-paper-scissors.rs"]
mod rock_paper_scissors;
#[path="./03-rucksack.rs"]
mod rucksack;
#[path="./04-camp-cleanup.rs"]
mod camp_cleanup;
#[path="./05-stacks.rs"]
mod stacks;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [day-num]", args[0]);
        process::exit(1);
    }
    let day: u8 = args[1].parse::<u8>().unwrap();
    match day {
        1 => calorie::run(),
        2 => rock_paper_scissors::run(),
        3 => rucksack::run(),
        4 => camp_cleanup::run(),
        5 => stacks::run(),
        _ => {
            eprintln!("Invalid Day Value.");
            process::exit(1)
        }
    };
}
