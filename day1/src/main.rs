use nom::character::{self, anychar, digit1};

use nom::IResult;
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

fn get_rotation(code: &str) -> IResult<&str, i32> {
    let (rem, mut val) = character::complete::i32(code)?;
    return Ok((rem, val));
}

fn main() {
    let input = read_to_string(file_path).expect("Failed to read input file");
    // println!("{}", input);
    let mut cumsum = 50;
    let mut cnt = 0;
    for line in input.lines() {
        let (o, rot) = direction(line).unwrap();
        let (_, magnitude) = get_rotation(o).unwrap();
        let mut angle = rot * magnitude;

        cumsum += angle;
        if (cumsum == 0) {
            cnt += 1;
        } else if (cumsum < 0) {
            cumsum += 100;
        } else if (cumsum >= 100) {
            cumsum -= 100;
        }

        println!("{line} -> {rot}, {o}: {angle}, {cumsum}");
    }
    println!("The number of zero crossings: {cnt}");
}
