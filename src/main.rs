use advent_of_code_2022;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {}

    let task = args.get(1);
    let result = match task.map(|value| value.as_str()) {
        Some("day01/task1") => advent_of_code_2022::day01::task1("./inputs/day01").to_string(),
        Some("day01/task2") => advent_of_code_2022::day01::task2("./inputs/day01").to_string(),
        Some("day02/task1") => {
            advent_of_code_2022::day02::task1::task1("./inputs/day02").to_string()
        }
        _ => {
            println!("[usage] aoc2022 dayXY/taskZ");
            exit(1);
        }
    };

    println!("{}: {}", task.unwrap(), result);
}
