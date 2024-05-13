use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>  {
    let result = read_lines("input");
    let mut file = File::create("out")?;
    let mut sum: i32 = 0;


    for line in result {
        let digits: Vec<_> = line.chars().filter(|char| char.is_digit(10)).collect();
        writeln!(&mut file, "{}{}", digits[0], digits[digits.len()-1])?;
        //println!("{line} {} {}", digits[0], digits[digits.len()-1]);
        //println!("{}{}", digits[0], digits[digits.len()-1]);

        let s = format!("{}{}", digits[0], digits[digits.len()-1]);

        sum += s.parse::<i32>()?;

    }


    println!("{sum}");

    return Ok(());
}

fn read_lines(filename: &str) -> Vec<String> {

    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    return result;
}
