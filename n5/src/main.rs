use std::fs::read_to_string;
use clap::Parser;
use std::collections::HashMap;


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
    let mut total_power: i32 = 0;


    for line in result{
        for char in line {
            print!("{char}");
        }
        println!("");
    }


    println!("Possible games {total_power}");
}



fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line_vec = line.chars().collect::<Vec<char>>();
        result.push(line_vec);
    }

    return result;
}

