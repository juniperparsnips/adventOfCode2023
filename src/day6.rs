use core::str::FromStr;

fn main() {
    let input = include_str!("../assets/day6Input.txt");
    let part_1_res = run_part_1(input);
    println!("part 1: {part_1_res}");
    let part_2_res = run_part_2(input);
    println!("part 2: {part_2_res}");
}

fn run_part_1(input: &str) -> usize {
    let competition: Competition = input.parse().unwrap();

    competition
        .races
        .iter()
        .map(|r| r.num_ways_to_beat())
        .product()
}

fn run_part_2(input: &str) -> usize {
    let race: Race = input.parse().unwrap();

    race.num_ways_to_beat()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn _distance_traveled(&self, time_held: usize) -> usize {
        if time_held >= self.time {
            return 0;
        }
        time_held * (self.time - time_held)
    }

    fn num_ways_to_beat(&self) -> usize {
        // The distance traveled is just `f(x) = (t - x) * x = -x^2 - tx` where t is the race time
        // We can simply find the 0s of the function `g(x) = f(x) - r = -x^2 - tx - r` where r is the record
        // (-b +/- sqrt(b^2 - 4ac) ) / (2a)

        let diff = f64::sqrt((self.time.pow(2) - 4 * self.record) as f64);
        let lower = (-(self.time as f64) + diff) / (-2.0);
        let upper = (-(self.time as f64) - diff) / (-2.0);

        let r1 = upper - lower;
        let range = upper.floor() - lower.ceil();
        if range == r1 {
            range as usize - 1
        } else {
            range as usize + 1
        }
    }
}

impl FromStr for Race {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        if lines.len() != 2 {
            return Err(format!(
                "Expected 2 lines exactly!, found '{}'",
                lines.len()
            ));
        }

        let mut times = lines[0].split_ascii_whitespace();
        let mut records = lines[1].split_ascii_whitespace();

        if times.next() != Some("Time:") {
            return Err("Expected first line to begin with 'Time:'".to_string());
        }

        if records.next() != Some("Distance:") {
            return Err("Expected second line to begin with 'Distance:'".to_string());
        }

        let time: String = times.collect();
        let record: String = records.collect();

        Ok(Race {
            time: time.parse().map_err(|e| format!("{e}"))?,
            record: record.parse().map_err(|e| format!("{e}"))?,
        })
    }
}

struct Competition {
    races: Vec<Race>,
}

impl FromStr for Competition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        if lines.len() != 2 {
            return Err(format!(
                "Expected 2 lines exactly!, found '{}'",
                lines.len()
            ));
        }

        let mut times = lines[0].split_ascii_whitespace();
        let mut records = lines[1].split_ascii_whitespace();

        if times.next() != Some("Time:") {
            return Err("Expected first line to begin with 'Time:'".to_string());
        }

        if records.next() != Some("Distance:") {
            return Err("Expected second line to begin with 'Distance:'".to_string());
        }

        let races = times
            .zip(records)
            .map(|(time, record)| {
                Ok(Race {
                    time: time.parse().map_err(|e| format!("{e}"))?,
                    record: record.parse().map_err(|e| format!("{e}"))?,
                })
            })
            .collect::<Result<_, String>>()?;
        Ok(Competition { races })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_answer() {
        let input = r##"Time:      7  15   30
Distance:  9  40  200
"##;

        assert_eq!(run_part_1(input), 288);
    }

    #[test]
    fn part_2_known_answer() {
        let input = r##"Time:      7  15   30
Distance:  9  40  200
"##;

        assert_eq!(run_part_2(input), 71503);
    }
}
