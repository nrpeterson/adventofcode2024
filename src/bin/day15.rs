use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Index, IndexMut};
use itertools::Itertools;
use adventofcode2024::build_main;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Contents { Empty, Box, BoxLeft, BoxRight, Wall, Robot }
use crate::Contents::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction { Up, Down, Left, Right }
use Direction::*;


#[derive(Debug)]
struct Level {
    rows: usize,
    cols: usize,
    board: Vec<Vec<Contents>>,
    robot_pos: (usize, usize)
}

impl Level {
    fn get(&self, pos: (usize, usize)) -> Option<&Contents> {
        let (i, j) = pos;
        self.board.get(i).and_then(|row| row.get(j))
    }

    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut Contents> {
        let (i, j) = pos;
        self.board.get_mut(i).and_then(|row| row.get_mut(j))
    }

    fn next_pos(&self, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        match (direction, pos) {
            (Up, (i, _)) if i == 0 => None,
            (Up, (i, j)) => Some((i - 1, j)),
            (Down, (i, _)) if i == self.rows - 1 => None,
            (Down, (i, j)) => Some((i + 1, j)),
            (Left, (_, j)) if j == 0 => None,
            (Left, (i, j)) => Some((i, j - 1)),
            (Right, (_, j)) if j == self.cols - 1 => None,
            (Right, (i, j)) => Some((i, j + 1))
        }
    }
}

impl Index<(usize, usize)> for Level {
    type Output = Contents;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Level {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Level {
    fn expand(self) -> Level {
        let board: Vec<Vec<Contents>> = self.board.into_iter()
            .map(|row| {
                row.into_iter().flat_map(|contents| {
                    match contents {
                        Empty => vec![Empty, Empty],
                        Box => vec![BoxLeft, BoxRight],
                        Wall => vec![Wall, Wall],
                        Robot => vec![Robot, Empty],
                        _ => panic!("Can't expand this")
                    }
                }).collect()
            })
            .collect();

        let robot_pos = (self.robot_pos.0, 2 * self.robot_pos.1);
        let rows = self.rows;
        let cols = 2 * self.cols;

        Level { rows, cols, board, robot_pos }
    }

    fn apply_move(&mut self, direction: Direction) -> Option<(usize, usize)> {
        let mut updates = HashMap::new();
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        let robot_new_space = self.next_pos(self.robot_pos, direction)?;
        queue.push_back(self.robot_pos);
        seen.insert(self.robot_pos);

        while let Some(cur_pos) = queue.pop_front() {
            let cur_type = self[cur_pos];

            if cur_type == Wall {
                return None
            }

            if cur_type == Empty {
                continue
            }

            let new_pos = self.next_pos(cur_pos, direction)?;
            let mut neighbors = vec![new_pos];

            if cur_type == BoxLeft && (direction == Up || direction == Down) {
                neighbors.push(self.next_pos(cur_pos, Right)?);
            }
            else if cur_type == BoxRight && (direction == Up || direction == Down) {
                neighbors.push(self.next_pos(cur_pos, Left)?);
            }

            neighbors.into_iter().for_each(|n| {
                if !seen.contains(&n) {
                    seen.insert(n);
                    queue.push_back(n);
                }
            });

            updates.insert(new_pos, cur_type);
            updates.entry(cur_pos).or_insert(Empty);
        }

        updates.into_iter().for_each(|(k, v)| {
            self[k] = v;
        });
        self.robot_pos = robot_new_space;

        Some(robot_new_space)
    }
}

mod parse {
    use nom::branch::alt;
    use nom::character::complete::{char, multispace0, newline};
    use nom::combinator::{map, opt, value};
    use nom::IResult;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{preceded, separated_pair};
    use crate::{Contents, Direction, Level};

    pub fn parse_input(input: &str) -> IResult<&str, (Level, Vec<Direction>)> {
        let cellp = alt((
            value(Contents::Box, char('O')),
            value(Contents::Empty, char('.')),
            value(Contents::Wall, char('#')),
            value(Contents::Robot, char('@'))
        ));

        let boardp = separated_list1(
            newline,
            many1(cellp)
        );

        let directionp = alt((
            value(Direction::Up, char('^')),
            value(Direction::Down, char('v')),
            value(Direction::Left, char('<')),
            value(Direction::Right, char('>'))
        ));

        let directionsp = many1(preceded(opt(newline), directionp));

        let mut parser = map(
            separated_pair(boardp, multispace0, directionsp),
            |(board, directions)| {
                let rows = board.len();
                let cols = board[0].len();

                let robot_pos = board.iter().enumerate()
                    .filter_map(|(i, row)| {
                        row.iter().enumerate()
                            .find(|&(_, &contents)| contents == Contents::Robot)
                            .map(|(j, _)| (i, j))
                    })
                    .next()
                    .unwrap();

                (Level { rows, cols, board, robot_pos }, directions)
            }
        );

        parser(input)
    }
}

fn part1(input: &str) -> usize {
    let (mut level, directions) = parse::parse_input(input).unwrap().1;

    for direction in directions {
        level.apply_move(direction);
    }

    let mut total = 0;
    for (i, j) in (0..level.rows).cartesian_product(0..level.cols) {
        if level[(i, j)] == Box {
            total += 100*i + j;
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let (orig_level, directions) = parse::parse_input(input).unwrap().1;

    let mut level = orig_level.expand();

    for direction in directions {
        level.apply_move(direction);
    }

    let mut total = 0;
    for (i, j) in (0..level.rows).cartesian_product(0..level.cols) {
        if level[(i, j)] == BoxLeft {
            total += 100*i + j;
        }
    }

    total
}

build_main!("day15.txt", "Part 1" => part1, "Part 2" => part2);