// Day 1 - Calorie Counting

use arcblroth_aoc2022::input;

fn main() {
    let mut curr_calories = 0;
    let mut top_calories = [0u32; 3];
    for line in input!() {
        match line.parse::<u32>() {
            Ok(calories) => {
                curr_calories += calories;
            }
            Err(_) => {
                // assume blank line
                for i in 0..top_calories.len() {
                    if curr_calories > top_calories[i] {
                        for j in i..(top_calories.len() - 1) {
                            top_calories[j + 1] = top_calories[j];
                        }
                        top_calories[i] = curr_calories;
                        break;
                    }
                }
                curr_calories = 0;
            }
        }
    }
    // (input ends with a blank line, so top_calories should always be accurate here)
    let most_calories = top_calories[0];
    let sum_top_calories: u32 = top_calories.iter().sum();
    println!("{most_calories} {sum_top_calories}");
}
