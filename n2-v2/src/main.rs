use clap::Parser;
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
    let mut counter = 0;

    for line in result {
        let first = "";
        let last = "";

        let number = match convert_digits(line.as_str()) {
            Ok(n) => n,
            Err(err) => panic!("Number incorrect!\n {}", err),
        };

        println!("{line} = {number}");

        // counter += 1;
        // if counter > 30 {
        //     return;
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

fn convert_digits(line: &str) -> Result<i32, ParseIntError> {
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
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ]);
    let mut min = 100000;
    let mut max = 0;
    let mut first: &str = "";
    let mut last: &str = "";

    for (key, value) in &numbers_map {
        // let pos = line.find(key);
        let positions: Vec<usize> = line.match_indices(key).map(|(i, _)| i).collect();
        let mut max_pos_tmp = positions.iter().max();
        let mut min_pos_tmp = positions.iter().min();

        if min_pos_tmp == None && max_pos_tmp == None {
            continue;
        } else if min_pos_tmp == None {
            min_pos_tmp = max_pos_tmp;
        } else if max_pos_tmp == None {
            max_pos_tmp = min_pos_tmp;
        }

        let max_pos = max_pos_tmp.unwrap() + 1;
        let min_pos = min_pos_tmp.unwrap() + 1;

        if min > min_pos {
            min = min_pos;
            first = value;
        }
        if max < max_pos {
            max = max_pos;
            last = value;
        }
    }

    return format!("{first}{last}").parse::<i32>();
}
