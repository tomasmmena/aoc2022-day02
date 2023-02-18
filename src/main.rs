use std::env;
use std::fs;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Clone, Copy)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    fn value(&self) -> isize {
        match self {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3
        }
    }
}

enum RPSResult {
    Win,
    Lose,
    Draw,
}

impl RPSResult {
    fn value(&self) -> isize {
        match self {
            RPSResult::Win => 6,
            RPSResult::Lose => 0,
            RPSResult::Draw => 3,
        }
    }
}

struct RPSMatch {
    own: RPSChoice,
    other: RPSChoice,
}

impl RPSMatch {
    fn get_result(&self) -> RPSResult {
        match (&self.own.value() - &self.other.value()).rem_euclid(3) {
            0 => RPSResult::Draw,
            1 => RPSResult::Win,
            2 => RPSResult::Lose,
            _ => panic!("something ain't right")
        }
    }

    fn get_score(&self) -> isize {
        &self.get_result().value() + &self.own.value()
    }

    fn from_string(value: &str) -> RPSMatch {
        let re = Regex::new(r"^(A|B|C) (X|Y|Z)$").unwrap();
        if re.is_match(value) {
            let captures = re.captures_iter(value).next().expect("already matched");
            RPSMatch {
                own: match captures.get(2).unwrap().as_str() {
                    "X" => RPSChoice::Rock,
                    "Y" => RPSChoice::Paper,
                    "Z" => RPSChoice::Scissors,
                    _ => panic!("Error")
                },
                other: match captures.get(1).unwrap().as_str() {
                    "A" => RPSChoice::Rock,
                    "B" => RPSChoice::Paper,
                    "C" => RPSChoice::Scissors,
                    _ => panic!("Error")
                }
            }
        } else {
            panic!("Invalid pairing!")
        }

    }
}

fn main() {
    // Parse input params
    let path = env::args().nth(1).expect("No path provided.");

    // Load strategy file
    println!("Loading strategy from: {}", path);
    let data = io::BufReader::new(
        fs::File::open(path)
            .expect("Could not read file!")
        )
        .lines()
        // Parse games of rock paper scissors
        .map(| line | {
            let l = line.expect("don't care");
            RPSMatch::from_string(&l)
        });

    let total_score: isize = data
        .map(|game| game.get_score())
        .sum();

    println!("Total score is: {}", total_score);
}
