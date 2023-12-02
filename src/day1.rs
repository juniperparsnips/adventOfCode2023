fn main() {
    let input = include_str!("../assets/day1Input.txt");
    let output = map_calibration_sum(input);
    println!("{output}");
}

fn map_calibration_sum(calibration_doc: &str) -> u32 {
    let mut sum = 0;
    for line in calibration_doc.lines() {
        let mut found_any = false;
        let mut first = "".to_string();
        let mut last = "".to_string();
        for char in line.chars() {
            if char.is_ascii_digit() {
                last = char.into();
                if !found_any {
                    first = last.clone();
                    found_any = true;
                }
            }
        }
        if found_any {
            let concated = first + &last;
            sum += u32::from_str_radix(&concated, 10).unwrap();
        }
    }

    return sum;
}

#[cfg(test)]
mod test {
    use crate::map_calibration_sum;

    #[test]
    fn day1_example_input() {
        let input = r##"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"##;

        assert_eq!(map_calibration_sum(input), 142)
    }
}
