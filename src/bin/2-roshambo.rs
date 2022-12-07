// Day 2 - Rock Paper Scissors

use arcblroth_aoc2022::input;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn decode(c: char) -> Shape {
    match c {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => panic!("SCP 4633 DETECTED"),
    }
}

fn outcome(them: Shape, you: Shape) -> u8 {
    match (them, you) {
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        (them, you) if them == you => 3,
        _ => 0,
    }
}

fn part2_outcome(them: Shape, force: char) -> u8 {
    match force {
        'X' => ((them as u8 + 1) % 3 + 1) as u8 + 0,
        'Y' => (them as u8) + 3,
        'Z' => ((them as u8) % 3 + 1) as u8 + 6,
        _ => panic!("invalid move"),
    }
}

fn main() {
    let mut score = 0u32;
    let mut part2_score = 0u32;
    for line in input!() {
        let mut chars = line.chars();
        let them = decode(chars.next().unwrap());
        let second_char = chars.nth(1).unwrap();
        let you = decode(second_char);
        score += ((you as u8) + outcome(them, you)) as u32;
        part2_score += part2_outcome(them, second_char) as u32;
    }
    println!("{score} {part2_score}");
}
