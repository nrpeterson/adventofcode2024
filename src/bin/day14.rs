use std::cmp::Ordering;
use std::ops::{Add, Mul, Rem, Sub};
use itertools::Itertools;
use adventofcode2024::build_main;
use crate::parse::parse_input;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vector(isize, isize);

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Vector> for usize {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let k = self as isize;
        Vector(k * rhs.0, k * rhs.1)
    }
}

impl Rem for Vector {
    type Output = Vector;

    fn rem(self, rhs: Self) -> Self::Output {
        Vector(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}

#[derive(Eq, PartialEq, Hash)]
enum Quadrant { NE, SE, SW, NW }

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Vector,
    velocity: Vector,
    board: Vector
}

impl Robot {
    fn updated(&self, frames: usize) -> Robot {
        let position = (self.position + frames * self.velocity) % self.board;
        Robot { position, ..*self }
    }

    fn quadrant(&self) -> Option<Quadrant> {
        assert_eq!(self.board.0 % 2, 1);
        assert_eq!(self.board.1 % 2, 1);
        let x_mid = (self.board.0 - 1) / 2;
        let y_mid = (self.board.1 - 1) / 2;

        match (self.position.0.cmp(&x_mid), self.position.1.cmp(&y_mid)) {
            (Ordering::Greater, Ordering::Greater) => Some(Quadrant::SE),
            (Ordering::Greater, Ordering::Less) => Some(Quadrant::SW),
            (Ordering::Less, Ordering::Greater) => Some(Quadrant::NE),
            (Ordering::Less, Ordering::Less) => Some(Quadrant::NW),
            _ => None
        }
    }
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, newline, space1};
    use nom::combinator::{map, map_res, opt};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, tuple};

    use super::{Robot, Vector};

    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            tuple((opt(char('-')), digit1)),
            |(sign, digits)| {
                str::parse::<isize>(digits).map(|num| if sign.is_some() { -num } else { num })
            }
        )(input)
    }

    fn vector(input: &str) -> IResult<&str, Vector> {
        let pair = separated_pair(number, char(','), number);
        map(pair, |(x, y)| Vector(x, y))(input)
    }

    pub fn parse_input(input: &str, board: Vector) -> Vec<Robot> {
        let p = preceded(tag("p="), vector);
        let v = preceded(tag("v="), vector);
        let robot = map(
            separated_pair(p, space1, v),
            |(pos, vel)| Robot { position: pos, velocity: vel, board }
        );

        separated_list1(newline, robot)(input).unwrap().1
    }
}

fn part1(input: &str) -> usize {
    let board = Vector(101, 103);
    let (ne, se, sw, nw) = parse_input(input, board).iter()
        .filter_map(|robot| robot.updated(100).quadrant())
        .fold((0, 0, 0, 0), |(ne, se, sw, nw), q| {
            match q {
                Quadrant::NE => (ne + 1, se, sw, nw),
                Quadrant::SE => (ne, se + 1, sw, nw),
                Quadrant::SW => (ne, se, sw + 1, nw),
                Quadrant::NW => (ne, se, sw, nw + 1)
            }
        });

    ne * se * sw * nw
}


fn to_map(robots: &Vec<Robot>) -> Vec<Vec<bool>> {
    let board = robots[0].board;

    let mut map: Vec<Vec<bool>> = (0..board.1).map(|_| vec![false; board.0 as usize]).collect();
    robots.iter().for_each(|robot| {
        assert_eq!(robot.board, board);
        let x = robot.position.0 as usize;
        let y = robot.position.1 as usize;
        map[y][x] = true;
    });

    map
}

fn map_to_string(map: &Vec<Vec<bool>>) -> String {
    let mut result = String::new();

    map.iter().for_each(|row| {
        row.iter().for_each(|&present| {
            result.push(if present { '*' } else { ' ' });
        });
        result.push('\n');
    });

    result
}

fn neighbor_score_at(map: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    if !map[i][j] {
        return 0
    }

    let mut score = -1;
    (i as isize - 1..i as isize +1)
        .cartesian_product(j as isize - 1..j as isize +1)
        .filter(|&(i, j)| i >= 0 && i < map.len() as isize && j >= 0 && j < map[0].len() as isize)
        .map(|(i, j)| (i as usize, j as usize))
        .for_each(|(i, j)| if map[i][j] { score += 1 });

    score as usize
}

/// Score to try to look for the tree...
///
/// If the robots are going to form a picture, they're going to need to be close to each other.
///
/// This score is added up pixel by pixel; the score for a pixel is 0 if it is off; if it is on,
/// then the score is the number of pixels the 3x3 grid centered at this pixel that are on.
///
/// This will tend to favor images that have lots of structure to them as opposed to random single
/// pixels.
fn neighbor_score(map: &Vec<Vec<bool>>) -> usize {
    (0..map.len()).cartesian_product(0..map[0].len())
        .map(|(i, j)| neighbor_score_at(map, i, j))
        .sum()
}

fn part2(input: &str) -> usize {
    let robots = parse_input(input, Vector(101, 103));

    let mut best_i = 0;
    let mut best_map = "".to_owned();
    let mut best_score = 0;

    (0..101*103).for_each(|i| {
        let updated: Vec<Robot> = robots.iter().map(|r| r.updated(i)).collect();
        let map = to_map(&updated);
        let score = neighbor_score(&map);

        if score > best_score {
            best_i = i;
            best_score = score;
            best_map = map_to_string(&map);
        }
    });

    println!("{best_map}");
    best_i
}

build_main!("day14.txt", "Part 1" => part1, "Part 2" => part2);