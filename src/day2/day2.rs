use std::fs::File;
use std::io::prelude::*;

// A & X = Rock (1 pt)
// B & Y = Paper (2 pt)
// C & Z = Scissors (3 pt)

pub fn main() {
    let mut file = File::open("./src/day2/rps.txt").expect("Unable to open file");
    let mut rounds_file = String::new();
    file.read_to_string(&mut rounds_file)
        .expect("Error reading the file");

    let rounds: Vec<&str> = rounds_file.split("\n").collect();
    let mut results: Vec<u16> = Vec::new();

    for round in rounds {
        let split: Vec<&str> = round.split(" ").collect();
        let opponent_roll_str = split.first().unwrap().to_owned();
        let my_roll_str = split.last().unwrap().to_owned();

        let opponent_roll = parse_roll(opponent_roll_str);
        let my_roll = parse_roll(my_roll_str);

        let mut round_score: u16 = 0;

        if opponent_roll == my_roll { // there was a tie
            round_score += roll_to_points(&my_roll) + 3;
            println!("There was a tie! We both rolled: {:?}", my_roll);
        } else if opponent_roll == Roll::PAPER && my_roll == Roll::SCISSORS {
            round_score += roll_to_points(&my_roll) + 6; // i won
            println!("I won! Opponent rolled {:?} and I rolled {:?}", opponent_roll, my_roll);
        } else if opponent_roll == Roll::SCISSORS && my_roll == Roll::ROCK {
            round_score += roll_to_points(&my_roll) + 6; // i won
            println!("I won! Opponent rolled {:?} and I rolled {:?}", opponent_roll, my_roll);
        } else if opponent_roll == Roll::ROCK && my_roll == Roll::PAPER {
            round_score += roll_to_points(&my_roll) + 6; // i won
            println!("I won! Opponent rolled {:?} and I rolled {:?}", opponent_roll, my_roll);
        } else {
            round_score += roll_to_points(&my_roll) + 0; // i lost
            println!("I lost! Opponent rolled {:?} and I rolled {:?}", opponent_roll, my_roll);
        }

        results.push(round_score);
    }

    let final_score = results.iter().fold(0, |acc, cur| acc + cur);

    println!("My total score is: {:?}", final_score);
}

#[derive(PartialEq)] // this trait allows you to check Rolls against each other.
#[derive(Debug)] // this trait allows for println! formatting
enum Roll {
    ROCK,
    PAPER,
    SCISSORS
}

fn parse_roll(roll: &str) -> Roll {
    match roll {
        "A" => Roll::ROCK,
        "B" => Roll::PAPER,
        "C" => Roll::SCISSORS,
        "X" => Roll::ROCK,
        "Y" => Roll::PAPER,
        "Z" => Roll::SCISSORS,
        _ => panic!("Invalid roll: {}", roll)
    }
}

fn roll_to_points(roll: &Roll) -> u16 {
    match roll {
        Roll::ROCK => 1,
        Roll::PAPER => 2,
        Roll::SCISSORS => 3
    }
}