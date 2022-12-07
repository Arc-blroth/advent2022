// Day 1 - Calorie Counting

use arcblroth_aoc2022::input;

fn main() {
    let mut curr_calories = 0;
    let mut most_calories = 0;
    for line in input!() {
        match line.parse::<u32>() {
            Ok(calories) => {
                curr_calories += calories;
            }
            Err(_) => {
                // assume blank line
                if curr_calories > most_calories {
                    most_calories = curr_calories;
                }
                curr_calories = 0;
            }
        }
    }
    if curr_calories > most_calories {
        most_calories = curr_calories;
    }
    println!("{most_calories}");
}
