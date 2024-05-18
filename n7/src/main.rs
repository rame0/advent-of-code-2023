use std::fs::read_to_string;
use clap::Parser;


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
    let mut total_points = 0;

    let lines = read_lines(&args.file);
   
    let mut ticket_number = 0;
    for line in lines {
        let ticket = parse_line(&line);
        ticket_number += 1;

        println!("\x1b[32m{}\x1b[0m", line.iter().collect::<String>());
        println!("{:?}, {:?}", ticket[0], ticket[1]);

        let mut count_match = 0;

        print!("Wining in ticket {}: ", ticket_number);
        for my_num in &ticket[1]{
            if ticket[0].contains(&my_num){
                count_match += 1;
                print!(" {} ", my_num);
            }
        }
        // 1 2 4 8 16 32 64
        let points: i32 = if count_match > 0 {
            2_i32.pow(count_match - 1)
        }else{
            0
        };
        total_points += points;
        println!("\nCount: {}, points \x1b[31m{}\x1b[0m", count_match, points);
    }

    println!("Total points: {}", total_points);
}

fn parse_line(line: &Vec<char>) -> [Vec<i32>; 2] {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let mut k = 0;
    let mut cur_num = 0;
    let mut win_nums: Vec<i32> = Vec::new();
    let mut my_nums: Vec<i32> = Vec::new();

    let mut cur_arr = &mut win_nums;

    while k < line.len() {
        if line[k] == ':' {
            k += 1;
            break;
        }
        k += 1;
    }

    while k < line.len(){
        k += 1;
        if k == line.len() || line[k] == ' ' {
            if cur_num > 0 {
                let _ = &cur_arr.push(cur_num);
                cur_num = 0;
            }
            continue;
        }
        if line[k] == '|' {
            cur_arr = &mut my_nums;
            continue;
        }

        let num: i32 = (line[k] as i32) - 48;
        cur_num = cur_num * 10 + num;

//        print!("{} ", num);
    }
//    println!();

    return [win_nums, my_nums];
}



fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line_vec = line.chars().collect::<Vec<char>>();
        result.push(line_vec);
    }

    return result;
}


