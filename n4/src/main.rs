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

struct Game {
    number: i32,
    bag: HashMap<String,i32>,
}

fn main() {
    let args = Args::parse();

    let result = read_lines(args.file.as_str());
    let mut total_power: i32 = 0;


    for line in result{
        println!("{line}");
        let mut game_power: i32 = 1;

        let game = parce_line(line);
        if game.bag.get("red").is_some() {
            game_power *= game.bag.get("red").unwrap();
        }

        if game.bag.get("green").is_some() {
            game_power *= game.bag.get("green").unwrap();
        }

        if game.bag.get("blue").is_some() {
            game_power *= game.bag.get("blue").unwrap();
        }

        total_power += game_power;

    }


    println!("Possible games {total_power}");
}

fn parce_line(line: String) -> Game {
    let mut cursor: usize = 0;

    let char_vec: Vec<char> = line.chars().collect();
    let len = char_vec.len();


    let pair = get_pair(&char_vec, &mut cursor);
    println!("{}", pair.join("_ "));
    let number = pair[1].parse::<i32>().unwrap();


    let mut bag: HashMap<String,i32> = Default::default();
    while cursor < len {
        let pair = get_pair(&char_vec, &mut cursor);
        println!("-{}-", pair.join("-")); 
        let count = pair[0].parse::<i32>().unwrap();

        let p =(&pair[1]).to_string(); 
        let item = bag.get(&p);


        match item {
            Some(i) => {
                if i < &count {
                    *bag.get_mut(&p).unwrap() = count;
                }
            },
            None => {
                bag.insert(p, count);
            }

        }
    }

    return Game {
        number,
        bag,
    }

}


fn get_pair(char_vec: &Vec<char>, cursor: &mut usize) -> [String; 2]{
    let len = char_vec.len();

    let mut first = String::new();
    let mut last = String::new();

    while *cursor < len && !skip(&char_vec[*cursor]) {
        first.push(char_vec[*cursor]);
        *cursor +=1;
    }

    while  *cursor < len && skip(&char_vec[*cursor]){
        *cursor+=1;
    }


    while *cursor < len && !skip(&char_vec[*cursor]) {
        last.push(char_vec[*cursor]);
        *cursor +=1;
    }

    while  *cursor < len && skip(&char_vec[*cursor]){
        *cursor+=1;
    }

    first = first.trim().to_string();
    last = last.trim().to_string();


    return [first, last];

}


fn skip(leter: &char) -> bool {
    return [' ', ',', ';', ':'].contains(leter); 
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    return result;
}

