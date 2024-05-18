use std::fs::read_to_string;
use clap::Parser;



static CHECK_REGION: [[i32; 2]; 8] = [ 
    [-1,-1], [-1, 0], [-1, 1],
    [ 0,-1],          [ 0, 1],
    [ 1,-1], [ 1, 0], [ 1, 1]
];



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}


fn main() {
    let args = Args::parse();
    
    let letters = read_lines(args.file.as_str());
    let mut total_power: i32 = 0;

    let rows = letters.len() as i32;
    let cols = letters[0].len() as i32;


    for y in 0..rows {
        for x in 0..cols {
            let char = &letters[y as usize][x as usize];
            if is_star(char) {
                    let mut adj_numbers: Vec<i32> = Vec::default();
                    let mut visited: Vec<[usize; 2]> = Vec::default();

                    for reg_pos in CHECK_REGION {
                        let x_pos: usize = if x + reg_pos[1] >= 0
                            && x + reg_pos[1] < rows {
                            (x + reg_pos[1]) as usize
                        } else {
                            continue;
                        }; 
                        
                        let y_pos: usize = if y + reg_pos[0] >= 0
                            && y + reg_pos[0] < cols {
                            (y + reg_pos[0]) as usize
                        } else {
                            continue;
                        }; 

                        let num: String = retrive_number(y_pos, x_pos, &letters, &mut visited);
                        match num.parse::<i32>() {
                            Ok(x) =>  adj_numbers.push(x),
                            Err(_) => continue
                        };
                    }

                    if adj_numbers.len() > 1 {
                        total_power += adj_numbers.iter().product::<i32>();
                        print!("\x1b[32m{char}\x1b[0m");
                    } else {
                        print!("\x1b[31m{char}\x1b[0m");
                    }
            } else {
                print!("{char}");
            }
        }
        println!("");
    }

//    let mut visited: Vec<[i32; 2]> = Vec::default();
//    println!("{}", retrive_number([0,2], &letters, &mut visited));
//    println!("{}", retrive_number([0,4], &letters, &mut visited));
//    println!("{}", retrive_number([4,0], &letters, &mut visited));
//
//    println!("{}", retrive_number([4,8], &letters, &mut visited));
//    println!("{}", retrive_number([4,9], &letters, &mut visited));
//
//    println!("{}", retrive_number([11,9], &letters, &mut visited));
    
    println!("Possible games {total_power}");
}

fn retrive_number(y: usize, x: usize, letters: &Vec<Vec<char>>, visited: &mut Vec<[usize;2]>) -> String {

    if visited.contains(&[y,x]){
        return String::new();
    }


    visited.push([y,x]);
    let char: &char = &letters[y][x];
//    print!("\x1b[31m{char}\x1b[0m");
    let num: i32 = (*char as i32)-48;

    if num >= 0 && num <= 9 {
        let l_char = if x == 0 {
            "".to_string()
        } else {
            retrive_number(y, x - 1, letters, visited)
        };

        let r_char = if x == letters[0].len()-1 {
            "".to_string()
        } else {
            retrive_number(y, x + 1, letters, visited)
        };

        return format!("{l_char}{num}{r_char}");
    }
    return String::new();
}


fn is_star(char: &char) -> bool {
    return char == &'*';
}


fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut letters: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line_vec = line.chars().collect::<Vec<char>>();
        letters.push(line_vec);
    }

    return letters;
}

