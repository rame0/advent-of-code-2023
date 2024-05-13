use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::num::ParseIntError;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let result = read_lines(args.file.as_str());
    let mut sum: i32 = 0;
    // let mut counter = 0;

    for line in result {
        let regex =
            Regex::new(r"(?x)(\d|(one|two|three|four|five|six|seven|eight|nine|ten))").unwrap();
        let digits: Vec<&str> = regex.find_iter(line.as_str()).map(|m| m.as_str()).collect();
        if digits.len() < 1 {
            println!("Nothing found in {line}");
            return;
        }

        let first = digits[0];
        let last = digits[digits.len() - 1];

        let number = match convert_digits(first, last) {
            Ok(n) => n,
            Err(err) => panic!("Number incorrect!\n {}", err),
        };

        // println!("{line}");
        // println!("{}", digits.join(", "));
        // println!("f: {first}, l: {last}, n: {number}");

        // counter += 1;
        // if counter > 10 {
        // return;
        // }

        sum += number
    }

    println!("{sum}");
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    return result;
}

fn convert_digits(first: &str, last: &str) -> Result<i32, ParseIntError> {
    let numbers_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let first_d: &str = numbers_map.get(first).or(Some(&first)).unwrap();
    let last_d: &str = numbers_map.get(last).or(Some(&last)).unwrap();

    return format!("{first_d}{last_d}").parse::<i32>();
}
