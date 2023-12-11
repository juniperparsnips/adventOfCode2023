use core::str::FromStr;

fn main() {
    let input = include_str!("../assets/day9Input.txt");
    let part_1 = run_part_1(input);
    println!("part 1: {part_1}");

    let part_2 = run_part_2(input);
    println!("part 2: {part_2}");
}

fn run_part_1(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let h: History = l.parse()?;
            Ok(h.next())
        })
        .sum::<Result<isize, String>>()
        .unwrap()
}

fn run_part_2(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let h: History = l.parse()?;
            Ok(h.previous())
        })
        .sum::<Result<isize, String>>()
        .unwrap()
}

#[derive(Debug, Clone)]
struct History(Vec<isize>);

impl History {
    fn diffs(&self) -> Self {
        let mut diffs = History(Vec::with_capacity(self.0.len() - 1));
        for i in 1..self.0.len() {
            diffs.0.push(self.0[i] - self.0[i - 1]);
        }
        diffs
    }

    fn next(&self) -> isize {
        let diffs = self.diffs();
        if diffs.0.iter().all(|d| *d == 0) {
            return self.0[0];
        }
        self.0[self.0.len() - 1] + diffs.next()
    }

    fn previous(&self) -> isize {
        let diffs = self.diffs();

        if diffs.0.iter().all(|d| *d == 0) {
            return self.0[0];
        }
        self.0[0] - diffs.previous()
    }
}

impl FromStr for History {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_ascii_whitespace()
            .map(|d| d.parse::<isize>().or_else(|e| Err(format!("{e}"))))
            .collect::<Result<_, String>>()?;
        Ok(History(nums))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_input() {
        let input = r##"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"##;

        assert_eq!(run_part_1(input), 114);
    }

    #[test]
    fn part_2_known_input() {
        let input = r##"10  13  16  21  30  45
"##;

        assert_eq!(run_part_2(input), 5);
    }
}
