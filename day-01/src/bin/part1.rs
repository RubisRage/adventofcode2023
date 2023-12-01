use std::fs;

fn parse_calibration_document(input: &str) -> Vec<u32> {
    let mut calibrations = vec![];

    for line in input.lines() {
        let line_numbers = &line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()[..];

        let calibration = match line_numbers {
            [first, .., last] => first * 10 + last,
            [unique] => unique * 10 + unique,
            _ => panic!("Invalid input"),
        };

        calibrations.push(calibration);
    }

    calibrations
}

fn main() {
    let input = fs::read_to_string("./input/input.txt").unwrap();

    println!("{}", parse_calibration_document(&input).iter().sum::<u32>());
}
