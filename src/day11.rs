use core::{
    fmt::{Display, Write},
    str::FromStr,
};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../assets/day11Input.txt");
    let res_1 = run_part_1(input);
    println!("part 1: {res_1}");
    // let res_2 = run_part_2(input);
    // println!("part 2: {res_2}");
}

fn run_part_1(input: &str) -> usize {
    let mut star_chart: StarChart = input.parse().unwrap();
    star_chart.expand();

    star_chart
        .pairs()
        .map(|(l, r)| l.taxi_cab_distance(r))
        .sum()
}

fn run_part_2(input: &str) -> usize {
    todo!()
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn taxi_cab_distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug, Clone)]
struct StarChart {
    width: usize,
    height: usize,
    galaxies: Vec<Galaxy>,
}

impl StarChart {
    fn expand(&mut self) {
        let height_set: HashSet<usize> = (0..self.height).collect();
        let occupied_rows: HashSet<usize> = self.galaxies.iter().map(|g| g.row).collect();
        let mut expand_rows: Vec<_> = height_set.difference(&occupied_rows).collect();
        expand_rows.sort();

        let width_set: HashSet<usize> = (0..self.width).collect();
        let occupied_cols: HashSet<usize> = self.galaxies.iter().map(|g| g.col).collect();
        let mut expand_cols: Vec<_> = width_set.difference(&occupied_cols).collect();
        expand_cols.sort();

        let mut rows_expanded_at = vec![0; self.height];
        for row in &expand_rows {
            let new = vec![rows_expanded_at[**row] + 1; self.height - *row];
            let _: Vec<_> = rows_expanded_at.splice(*row..&self.height, new).collect();
        }

        println!("{expand_cols:?}");
        let mut cols_expanded_at = vec![0; self.width];
        for col in &expand_cols {
            let new = vec![cols_expanded_at[**col] + 1; self.width - *col];
            let _: Vec<_> = cols_expanded_at.splice(*col..&self.width, new).collect();
        }
        println!("{cols_expanded_at:?}");

        self.height += expand_rows.len();
        self.width += expand_cols.len();

        for galaxy in &mut self.galaxies {
            galaxy.row += rows_expanded_at[galaxy.row];
            galaxy.col += cols_expanded_at[galaxy.col];
        }
    }

    fn pairs(&self) -> GalaxyPairs {
        println!("{} galaxies", self.galaxies.len());
        GalaxyPairs {
            galaxies: &self.galaxies,
            curr_left: 0,
            // The first pair given will be (0, 1), but next is called and 'curr_right' is incremented to get it
            curr_right: 0,
        }
    }
}

impl FromStr for StarChart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let mut width = None;
        let mut height = 0;
        for (row, line) in s.lines().enumerate() {
            height += 1;
            width.get_or_insert(line.len());
            galaxies.append(
                &mut line
                    .match_indices('#')
                    .map(|(i, _)| Galaxy { row, col: i })
                    .collect(),
            )
        }
        let width = width.ok_or_else(|| "Expected at least 1 line".to_string())?;
        Ok(StarChart {
            width,
            height,
            galaxies,
        })
    }
}

impl Display for StarChart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec![vec![".".to_string(); self.width]; self.height];

        for (i, galaxy) in self.galaxies.iter().enumerate() {
            chars[galaxy.row][galaxy.col] = i.to_string();
        }

        for line in chars {
            for c in line {
                let _ = f.write_str(&c)?;
            }
            let _ = f.write_char('\n');
        }
        Ok(())
    }
}

struct GalaxyPairs<'a> {
    galaxies: &'a Vec<Galaxy>,
    curr_left: usize,
    curr_right: usize,
}

impl<'a> Iterator for GalaxyPairs<'a> {
    type Item = (&'a Galaxy, &'a Galaxy);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_right + 1 < self.galaxies.len() {
            self.curr_right += 1;
            Some((
                &self.galaxies[self.curr_left],
                &self.galaxies[self.curr_right],
            ))
        } else if self.curr_left + 2 < self.galaxies.len() {
            self.curr_left += 1;
            self.curr_right = self.curr_left + 1;
            Some((
                &self.galaxies[self.curr_left],
                &self.galaxies[self.curr_right],
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_known_input() {
        let input = r##"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"##;

        assert_eq!(run_part_1(input), 374)
    }
}
