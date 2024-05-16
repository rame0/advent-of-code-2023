use std::fs::read_to_string;
use clap::Parser;



static CHECK_REGION: [[i32; 2]; 8] = [ 
    [-1,-1], [-1, 0], [-1, 1],
    [ 0,-1],          [ 0, 1],
    [ 1,-1], [ 1, 0], [ 1, 1]
];



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

    let mut numbers: Vec<i32> = Vec::new();

    let rows = result.len();
    let cols = result[0].len();

    let mut possible_number=String::new();
    let mut number_found = false;

    for y in 0..rows {
        for x in 0..cols {
            let char = &result[y][x];
            if is_number(char) {
                possible_number.push(*char);
                number_found = number_found || has_adj_symbol(y, x, [rows, cols], &result);
            } else {
                if number_found && possible_number.len() > 0 {
                    print!("\x1b[32m{possible_number}\x1b[0m");
                    numbers.push(possible_number.parse::<i32>().unwrap());
                } else if possible_number.len() > 0 {
                    print!("\x1b[31m{possible_number}\x1b[0m");
                }
                possible_number = String::new();
                number_found = false;
                print!("{char}");
            }
        }
        println!("");
    }
    
    for number in numbers{
        total_power += number;
    }
    println!("Possible games {total_power}");
}

fn has_adj_symbol(x: usize, y:   usize, dim: [usize;2], array: &Vec<Vec<char>>) -> bool {
    //    print!("{}(", &array[x][y]);
    for cord in CHECK_REGION {
        let x_cord = cord[0] + x as i32;
        let y_cord = cord[1] + y as i32;

        if  x_cord < 0 || y_cord < 0 ||
            x_cord >= dim[1] as i32 || y_cord >= dim[0] as i32 {
                //                print!("o");
                continue;
            }

        if is_symbol(&array[x_cord as usize][y_cord as usize]){
            //            print!("\x1b[93m{}\x1b[0m", &array[x_cord as usize][y_cord as usize]);
            return true;
            //        }else{
            //            print!("{}", &array[x_cord as usize][y_cord as usize]);
    }
    }
    //print!(")");
    return false;
}

fn is_symbol(char: &char) -> bool {
    return !is_number(char) && !is_nothing(char);
}

fn is_number(char: &char) -> bool {
    match char.to_digit(10){
        Some(_) => { return true;},
        None => { return false }
    };
}

fn is_nothing(char: &char) -> bool {
    return char == &'.';
}

fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line_vec = line.chars().collect::<Vec<char>>();
        result.push(line_vec);
    }

    return result;
}

