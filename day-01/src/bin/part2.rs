use std::fs;

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct CalibrationDigitIterator<I: Iterator<Item = char>> {
    spelled_digit_state: [usize; SPELLED_DIGITS.len()],
    iter: I,
}

impl<I> CalibrationDigitIterator<I>
where
    I: Iterator<Item = char>,
{
    fn new(stream: I) -> Self {
        let iter = stream;
        let spelled_digit_state = [0; SPELLED_DIGITS.len()];

        Self {
            iter,
            spelled_digit_state,
        }
    }
}

impl<I> Iterator for CalibrationDigitIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(to_parse_char) = self.iter.next() {
            if to_parse_char.is_ascii_digit() {
                return to_parse_char.to_digit(10);
            }

            let mut item = None;

            for (index, state) in
                self.spelled_digit_state.iter_mut().enumerate()
            {
                let spelled_digit = SPELLED_DIGITS[index];
                let is_still_candidate = spelled_digit
                    .chars()
                    .nth(*state)
                    .is_some_and(|word_char| to_parse_char == word_char);

                if is_still_candidate {
                    *state += 1;
                } else if spelled_digit.starts_with(to_parse_char) {
                    *state = 1;
                } else {
                    *state = 0;
                }

                if *state == spelled_digit.len() {
                    item = Some((index + 1) as u32);
                }
            }

            if item.is_some() {
                return item;
            }
        }

        None
    }
}

fn parse_calibration_document(input: &str) -> Vec<u32> {
    let mut calibrations = vec![];

    for line in input.lines() {
        let line_numbers: Vec<u32> =
            CalibrationDigitIterator::new(line.chars()).collect();

        println!("{:?}", line_numbers);

        let calibration = match line_numbers[..] {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn supper_posed_spelled_digits() {
        let input = "foursixtwoninevtzzgntnlg6oneightbxp";
        let iter = CalibrationDigitIterator::new(input.chars());

        assert_eq!(iter.collect::<Vec<u32>>()[..], [4, 6, 2, 9, 6, 1, 8]);
    }

    #[test]
    fn doble_starting_letter() {
        let input = "jtqsrmmbonentvmnxbdsseven4"; // sseven
        let iter = CalibrationDigitIterator::new(input.chars());

        assert_eq!(iter.collect::<Vec<u32>>()[..], [1, 7, 4]);
    }
}
