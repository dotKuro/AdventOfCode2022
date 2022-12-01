use std::cmp::Ordering;
use std::fs;

type Elf = Vec<i32>;

fn get_elves_from_input(input_file_path: &str) -> Vec<Elf> {
    let content = fs::read_to_string(input_file_path).expect("failed to read input file");

    let lines: Vec<&str> = content.split("\n").collect();
    let mut elves: Vec<Elf> = Vec::new();

    for line in lines {
        let mut current_elf = elves.pop().unwrap_or(Vec::new());
        match line {
            "" => {
                elves.push(current_elf);
                elves.push(Vec::new())
            }
            calorie_string => {
                let calorie_value: i32 = calorie_string
                    .parse()
                    .expect(&format!("failed to parse calorie value in line: {}", line));
                current_elf.push(calorie_value);
                elves.push(current_elf);
            }
        }
    }

    return elves.into_iter().filter(|elem| !elem.is_empty()).collect();
}

struct WithMostCaloriesResult {
    calories: i32,
    elves: Vec<Elf>,
}

trait Elves {
    fn with_most_calories(&self) -> WithMostCaloriesResult;
}

impl Elves for Vec<Elf> {
    fn with_most_calories(&self) -> WithMostCaloriesResult {
        let mut elves_with_current_max: Vec<Elf> = Vec::new();
        let mut current_max: i32 = 0;

        for current_elf in self {
            let calories_of_current_elf: i32 = current_elf.iter().sum();
            match calories_of_current_elf.cmp(&current_max) {
                Ordering::Less => {}
                Ordering::Equal => elves_with_current_max.push(current_elf.clone()),
                Ordering::Greater => {
                    current_max = calories_of_current_elf;
                    elves_with_current_max = vec![current_elf.clone()]
                }
            }
        }

        return WithMostCaloriesResult {
            calories: current_max,
            elves: elves_with_current_max,
        };
    }
}

pub fn task1(input_file_path: &str) -> i32 {
    let elves = get_elves_from_input(input_file_path);
    let result = elves.with_most_calories();

    if result.elves.is_empty() {
        panic!("found no elf with valid calorie value");
    }

    return result.calories;
}
