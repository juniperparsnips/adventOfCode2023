use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let start = Instant::now();
    let input = include_str!("../assets/day3Input.txt");
    let res = run_part_2(input);
    println!("{res}");

    let time = start.elapsed();
    println!("{time:?}");
}

fn run_part_1(input: &str) -> u32 {
    let potential_parts = read_part_numbers(input);

    let char_array = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for p in potential_parts {
        if p.is_part(&char_array) {
            sum += p.number
        }
    }

    sum
}

fn run_part_2(input: &str) -> u32 {
    let potential_parts = read_part_numbers(input);

    let char_array = input.lines().map(|l| l.chars().collect()).collect();

    let mut gears: HashMap<Gear, Vec<u32>> = HashMap::new();

    for p in potential_parts {
        let part_gears = p.gears(&char_array);
        for gear in part_gears {
            gears
                .entry(gear)
                .and_modify(|rs| rs.push(p.number))
                .or_insert(vec![p.number]);
        }
    }

    gears
        .iter()
        .filter_map(|(g, parts)| {
            if parts.len() == 2 {
                Some(parts[0] * parts[1])
            } else {
                None
            }
        })
        .reduce(core::ops::Add::add)
        .unwrap()
}

#[derive(Debug, Clone, PartialEq)]
struct PartNumber {
    index: usize,
    line_num: usize,
    length: usize,
    number: u32,
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Gear {
    row: usize,
    column: usize,
}

impl PartNumber {
    fn is_part(&self, input: &Vec<Vec<char>>) -> bool {
        // for simplicity I'm assuming the input is rectangular
        // which it is for the AOC input

        let left_start = if self.index == 0 { 0 } else { self.index - 1 };
        // non-inclusive
        let right_end = (self.index + self.length + 1).min(input[0].len());

        // top row
        if self.line_num != 0 {
            for i in left_start..right_end {
                if is_symbol(input[self.line_num - 1][i]) {
                    return true;
                }
            }
        }

        // left
        if self.index != 0 {
            if is_symbol(input[self.line_num][self.index - 1]) {
                return true;
            }
        }

        //right
        if self.index + self.length < input[self.line_num].len() {
            if is_symbol(input[self.line_num][self.index + self.length]) {
                return true;
            }
        }

        // bottom row
        if self.line_num + 1 < input.len() {
            for i in left_start..right_end {
                if is_symbol(input[self.line_num + 1][i]) {
                    return true;
                }
            }
        }

        return false;
    }

    fn gears(&self, input: &Vec<Vec<char>>) -> HashSet<Gear> {
        // for simplicity I'm assuming the input is rectangular
        // which it is for the one input this has to work for

        let mut gears = HashSet::new();

        let left_start: usize = if self.index == 0 { 0 } else { self.index - 1 };
        // non-inclusive
        let right_end = (self.index + self.length + 1).min(input[0].len());

        // top row
        if self.line_num != 0 {
            for i in left_start..right_end {
                if is_gear(input[self.line_num - 1][i]) {
                    gears.insert(Gear {
                        row: self.line_num - 1,
                        column: i,
                    });
                }
            }
        }

        // left
        if self.index != 0 {
            if is_gear(input[self.line_num][self.index - 1]) {
                gears.insert(Gear {
                    row: self.line_num,
                    column: self.index - 1,
                });
            }
        }

        //right
        if self.index + self.length < input[self.line_num].len() {
            if is_gear(input[self.line_num][self.index + self.length]) {
                gears.insert(Gear {
                    row: self.line_num,
                    column: self.index + self.length,
                });
            }
        }

        // bottom row
        if self.line_num + 1 < input.len() {
            for i in left_start..right_end {
                if is_gear(input[self.line_num + 1][i]) {
                    gears.insert(Gear {
                        row: self.line_num + 1,
                        column: i,
                    });
                }
            }
        }

        return gears;
    }
}

// Gets all sets of horizontally adjacent numbers from the string
fn read_part_numbers(input: &str) -> Vec<PartNumber> {
    let digits: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(line_num, line)| {
            let m = line.match_indices(char::is_numeric);
            let line_num_iter = vec![line_num; m.clone().count()];
            m.zip(line_num_iter)
        })
        .collect();

    // this is patently un-rustlike, but idk what else to do
    let mut numbers = Vec::new();

    let mut i = 0;
    while i < digits.len() - 1 {
        let mut running_number = String::new();
        let ((start_index, digit), line_num) = digits[i];
        let mut index = start_index;
        running_number += digit;
        for j in i + 1..digits.len() {
            i += 1;
            let ((index_next, digit_next), line_num_next) = digits[j];
            if line_num_next == line_num && index_next == index + 1 {
                running_number += digit_next;
                index += 1;
            } else {
                // i = j - 1;
                break;
            }
        }

        numbers.push(PartNumber {
            index: start_index,
            line_num,
            length: running_number.len(),
            number: u32::from_str_radix(&running_number, 10).unwrap(),
        });
        // i += 1;
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_known_input() {
        let input = r##"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"##;
        assert_eq!(run_part_1(input), 4361);
    }

    #[test]
    fn part2_known_input() {
        let input = r##"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"##;
        assert_eq!(run_part_2(input), 467835);
    }

    #[test]
    fn part1_num_on_end() {
        let input = r##"3...+467
......33
........
"##;
        assert_eq!(run_part_1(input), 467);
    }

    #[test]
    fn test_read_numbers() {
        let input = r##"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"##;

        let expected = vec![
            PartNumber {
                index: 0,
                line_num: 0,
                length: 3,
                number: 467,
            },
            PartNumber {
                index: 5,
                line_num: 0,
                length: 3,
                number: 114,
            },
            PartNumber {
                index: 2,
                line_num: 2,
                length: 2,
                number: 35,
            },
            PartNumber {
                index: 6,
                line_num: 2,
                length: 3,
                number: 633,
            },
            PartNumber {
                index: 0,
                line_num: 4,
                length: 3,
                number: 617,
            },
            PartNumber {
                index: 7,
                line_num: 5,
                length: 2,
                number: 58,
            },
            PartNumber {
                index: 2,
                line_num: 6,
                length: 3,
                number: 592,
            },
            PartNumber {
                index: 6,
                line_num: 7,
                length: 3,
                number: 755,
            },
            PartNumber {
                index: 1,
                line_num: 9,
                length: 3,
                number: 664,
            },
            PartNumber {
                index: 5,
                line_num: 9,
                length: 3,
                number: 598,
            },
        ];

        assert_eq!(read_part_numbers(input), expected)
    }
}
