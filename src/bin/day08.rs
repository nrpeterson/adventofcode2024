use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};
use itertools::Itertools;
use adventofcode2024::build_main;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vector(isize, isize);

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
    antennas: HashMap<char, Vec<Vector>>
}

impl Board {
    fn contains(&self, v: &Vector) -> bool {
        v.0 >= 0 && v.1 >= 0 && v.0 < self.rows as isize && v.1 < self.cols as isize
    }

    fn pair_antinodes(&self) -> HashSet<Vector> {
        self.antennas.iter()
            .flat_map(|(_, vs)| {
                vs.iter().combinations(2).flat_map(|vec| {
                    let v = vec[0];
                    let u = vec[1];
                    let delta = *v - *u;
                    vec![*u - delta, *v + delta]
                })
            })
            .filter(|v| self.contains(v))
            .collect()
    }

    fn linear_antinodes(&self) -> HashSet<Vector> {
        self.antennas.iter()
            .flat_map(|(_, vs)| {
                vs.iter().combinations(2).flat_map(|vec| {
                    let v = vec[0];
                    let u = vec[1];
                    let delta = *v - *u;

                    let mut result = Vec::new();
                    let mut cur = *u;
                    while self.contains(&(cur - delta)) {
                        cur = cur - delta;
                    }
                    while self.contains(&cur) {
                        result.push(cur);
                        cur = cur + delta;
                    }

                    result
                })
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Board {
    let antennas: HashMap<char, Vec<Vector>> = input.lines().enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                if c == '.' { None } else { Some((c, Vector(i as isize, j as isize))) }
            })
        })
        .fold(HashMap::new(), |mut acc, (c, v)| {
            acc.entry(c).or_default().push(v);
            acc
        });

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    Board { rows, cols, antennas }
}

fn part1(input: &str) -> usize {
    let board = parse_input(input);
    board.pair_antinodes().len()
}

fn part2(input: &str) -> usize {
    let board = parse_input(input);
    board.linear_antinodes().len()
}

build_main!("day08.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 34);
    }
}