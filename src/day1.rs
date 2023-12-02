fn main() {
    let input = include_str!("../assets/day1Input.txt");
    let output = map_calibration_sum(input);
    println!("{output}");
}

fn _parse_line_only_digits(line: &str) -> Option<u32> {
    let digits: Vec<&str> = line.matches(char::is_numeric).collect();

    if digits.len() == 0 {
        return None;
    }

    let first = digits[0].to_string();
    let last = digits[digits.len() - 1];

    let concated = first + &last;
    u32::from_str_radix(&concated, 10).ok()
}

const DIGIT_MATCHES: [&str; 18] = [
    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
    "eight", "8", "nine", "9",
];

fn into_digit(digit: &str) -> &str {
    match digit {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        other => other,
    }
}

fn parse_line(line: &str) -> Option<u32> {
    // Note this method only works because none of our needles are substrings of the others, and none start and begin with the same letter
    let digits_found: Vec<((usize, &str), (usize, &str))> = DIGIT_MATCHES
        .iter()
        .filter_map(|needle| {
            let mut digits = line.match_indices(needle).peekable();

            let first: Option<(usize, &str)> = digits.peek().copied();
            first.zip(digits.last())
        })
        .collect();

    if digits_found.len() == 0 {
        return None;
    }

    let mut first = digits_found[0].0;
    let mut last = digits_found[0].1;

    for (d_first, d_last) in &digits_found[1..] {
        if d_first.0 < first.0 {
            first = *d_first
        }
        if d_last.0 > last.0 {
            last = *d_last;
        }
    }

    let concated = into_digit(first.1).to_string() + into_digit(last.1);
    u32::from_str_radix(&concated, 10).ok()
}

fn map_calibration_sum(calibration_doc: &str) -> u32 {
    let mut sum = 0;
    for line in calibration_doc.lines() {
        sum += parse_line(line).unwrap();
    }

    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_example_input() {
        let input = r##"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"##;

        assert_eq!(map_calibration_sum(input), 142);
    }

    #[test]
    fn day1_part2_example_input() {
        let input = r##"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"##;

        assert_eq!(map_calibration_sum(input), 281);
    }

    #[test]
    fn test_overlapping_numbers() {
        let input = r##"twone
4fsdfjgj68fiveight
"##;
        // 21 + 48 = 69
        assert_eq!(map_calibration_sum(input), 69);
    }
}
