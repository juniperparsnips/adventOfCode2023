use core::str::FromStr;

fn main() {
    let input = include_str!("../assets/day10Input.txt");
    let part_1 = run_part_1(input);
    println!("part 1: {part_1}");

    // let part_2 = run_part_2(input);
    // println!("part 2: {part_2}");
}

fn run_part_1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    map.farthest()
}

fn run_part_2(input: &str) -> usize {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Tile {
    /// Returns the indices of the next tile, and the next "from" direction
    fn next(&self, from: Direction, row: usize, col: usize) -> (usize, usize, Direction) {
        match self {
            Tile::Vertical => match from {
                Direction::North => (row + 1, col, Direction::North),
                Direction::South => (row - 1, col, Direction::South),
                _ => panic!("invalid!"),
            },
            Tile::Horizontal => match from {
                Direction::East => (row, col - 1, Direction::East),
                Direction::West => (row, col + 1, Direction::West),
                _ => panic!("invalid!"),
            },
            Tile::NorthToEast => match from {
                Direction::North => (row, col + 1, Direction::West),
                Direction::East => (row - 1, col, Direction::South),
                _ => panic!("invalid!"),
            },
            Tile::NorthToWest => match from {
                Direction::North => (row, col - 1, Direction::East),
                Direction::West => (row - 1, col, Direction::South),
                _ => panic!("invalid!"),
            },
            Tile::SouthToWest => match from {
                Direction::South => (row, col - 1, Direction::East),
                Direction::West => (row + 1, col, Direction::North),
                _ => panic!("invalid!"),
            },
            Tile::SouthToEast => match from {
                Direction::South => (row, col + 1, Direction::West),
                Direction::East => (row + 1, col, Direction::North),
                _ => panic!("invalid!"),
            },
            _ => panic!("invalid!"),
        }
    }

    fn connects(&self, towards: Direction) -> bool {
        match self {
            Tile::Vertical => match towards {
                Direction::North | Direction::South => true,
                _ => false,
            },
            Tile::Horizontal => match towards {
                Direction::East | Direction::West => true,
                _ => false,
            },
            Tile::NorthToEast => match towards {
                Direction::East | Direction::North => true,
                _ => false,
            },
            Tile::NorthToWest => match towards {
                Direction::West | Direction::North => true,
                _ => false,
            },
            Tile::SouthToEast => match towards {
                Direction::East | Direction::South => true,
                _ => false,
            },
            Tile::SouthToWest => match towards {
                Direction::West | Direction::South => true,
                _ => false,
            },
            Tile::Ground => false,
            Tile::Start => true,
        }
    }
}

impl TryFrom<Vec<Direction>> for Tile {
    type Error = String;
    fn try_from(mut value: Vec<Direction>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!(
                "Expected exactly 2 directions to create a tile, found '{}'",
                value.len()
            ));
        }

        value.sort();

        let d1 = value[0];
        let d2 = value[1];

        match (d1, d2) {
            (Direction::North, Direction::South) => Ok(Tile::Vertical),
            (Direction::North, Direction::East) => Ok(Tile::NorthToEast),
            (Direction::North, Direction::West) => Ok(Tile::NorthToWest),
            (Direction::South, Direction::East) => Ok(Tile::SouthToEast),
            (Direction::South, Direction::West) => Ok(Tile::SouthToWest),
            (Direction::East, Direction::West) => Ok(Tile::Horizontal),
            o => Err(format!("Invalid directions to make tile '{:?}'", o)),
        }
    }
}

impl TryFrom<Tile> for Direction {
    type Error = String;
    fn try_from(value: Tile) -> Result<Self, Self::Error> {
        match value {
            Tile::Vertical | Tile::NorthToEast | Tile::NorthToWest => Ok(Direction::North),
            Tile::SouthToWest | Tile::SouthToEast => Ok(Direction::South),
            Tile::Horizontal => Ok(Direction::East),
            o => Err(format!("Can not convert {o:?} into default direction")),
        }
    }
}

impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::Vertical),
            "-" => Ok(Tile::Horizontal),
            "L" => Ok(Tile::NorthToEast),
            "J" => Ok(Tile::NorthToWest),
            "7" => Ok(Tile::SouthToWest),
            "F" => Ok(Tile::SouthToEast),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            o => Err(format!("Unknown tile '{o}'")),
        }
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn find_start(&self) -> Option<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                let c = r
                    .iter()
                    .enumerate()
                    .find(|(_, t)| **t == Tile::Start)
                    .unzip()
                    .0;
                Some(i).zip(c)
            })
            .next()
    }

    fn translate_start(&self, row: usize, col: usize) -> Tile {
        let up = if row == 0 {
            None
        } else {
            Some(self.0[row - 1][col])
        };
        let down = self.0.get(row + 1).map(|r| r.get(col)).flatten();
        let left = if col == 0 {
            None
        } else {
            Some(self.0[row][col - 1])
        };
        let right = self.0.get(row).map(|r| r.get(col + 1)).flatten();

        let mut dirs = Vec::with_capacity(2);
        if up.is_some_and(|u| u.connects(Direction::South)) {
            dirs.push(Direction::North)
        }
        if down.is_some_and(|d| d.connects(Direction::North)) {
            dirs.push(Direction::South)
        }
        if left.is_some_and(|l| l.connects(Direction::East)) {
            dirs.push(Direction::West)
        }
        if right.is_some_and(|r| r.connects(Direction::West)) {
            dirs.push(Direction::East)
        }

        dirs.try_into().unwrap()
    }

    fn farthest(&self) -> usize {
        let (start_row, start_col) = self.find_start().unwrap();
        let mut current = self.translate_start(start_row, start_col);
        let mut direction: Direction = current.try_into().unwrap();

        let (mut row, mut col) = (start_row, start_col);
        let mut steps = 0;
        loop {
            (row, col, direction) = current.next(direction, row, col);
            current = self.0[row][col];
            steps += 1;
            if row == start_row && col == start_col {
                return steps / 2;
            }
        }
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<Vec<_>, String>>()
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Map(map))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_known_input() {
        let input = r##".....
.S-7.
.|.|.
.L-J.
.....
"##;

        assert_eq!(run_part_1(input), 4);
    }

    #[test]
    fn part_1_known_input_2() {
        let input = r##"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"##;

        assert_eq!(run_part_1(input), 8);
    }

    // #[test]
    fn part_2_known_input() {
        let input = r##"
"##;

        assert_eq!(run_part_2(input), 0);
    }
}
