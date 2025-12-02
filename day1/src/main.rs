use nom::IResult;
use nom::character::{anychar, digit1};
use std::fs::read_to_string;

const file_path: &str = "day1a.txt";

fn direction(code: &str) -> IResult<&str, i32> {
    let (rem, dir) = anychar(code)?;

    let rot = match dir {
        'L' => 1,
        'R' => -1,
        _ => {
            return Err(nom::Err::Failure(nom::error::Error::new(
                "Invalid",
                nom::error::ErrorKind::Fail,
            )));
        } // _ => Error::new("Invalid direction character"),
    };
    return Ok((rem, rot));
}

// fn get_rotation(code: &str) -> i32 {}

fn main() {
    let input = read_to_string(file_path).expect("Failed to read input file");
    // println!("{}", input);
    for line in input.lines() {
        let (o, rot) = direction(line).unwrap();
        println!("{line} -> {rot}, {o}");
    }
}
