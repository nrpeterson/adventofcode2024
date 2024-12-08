use adventofcode2024::build_main;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction { Up, Down, Left, Right }
use Direction::*;

/// Tokens representing the semantics of the input characters.
#[derive(Copy, Clone)]
enum Token { Empty, Obstruction, Guard(Direction) }

/// State of the guard (either present at a position on the board or gone)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GuardState { Gone, Present(usize, usize, Direction) }

/// A path through the board, represented by the "corners" of the path.
///
/// If the path is a loop, the last element of corners will be the first position/direction
/// recognized as a loop.
///
/// If it is not a loop, the last element of corners will be the last position touched before
/// exiting the board.
struct Path {
    corners: Vec<(usize, usize, Direction)>,
    is_loop: bool
}

impl Path {
    /// All the spaces (and corresponding directions) touched by this path.
    fn all_spaces_and_dirs(&self) -> Vec<(usize, usize, Direction)> {
        self.corners.iter().cloned()
            .tuple_windows::<((usize, usize, Direction), (usize, usize, Direction))>()
            .flat_map(|((i1, j1, d1), (i2, j2, _))| {
                let segment: Vec<(usize, usize, Direction)> = match d1 {
                    Up => (i2..=i1).rev().map(|i| (i, j1, d1)).collect(),
                    Down => (i1..=i2).map(|i| (i, j1, d1)).collect(),
                    Left => (j2..=j1).rev().map(|j| (i1, j, d1)).collect(),
                    Right => (j1..=j2).map(|j| (i1, j, d1)).collect()
                };
                segment
            }).collect()
    }

    /// HashSet of all spaces this path ever touches.
    fn visited_spaces(&self) -> HashSet<(usize, usize)> {
        self.all_spaces_and_dirs().iter().map(|&(i, j, _)| (i, j)).collect()
    }
}

/// Trait representing the board, including the ability to find the next corner from a given state.
trait Board {
    fn next_state(&self, cur_state: GuardState) -> GuardState;
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;

    /// Compute the full path followed from the given initial state.
    fn path_from(&self, start: GuardState) -> Path {
        let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
        let mut corners = Vec::new();
        let mut guard = start;

        while let GuardState::Present(i, j, dir) = guard {
            if seen.contains(&(i, j, dir)) {
                break
            }
            seen.insert((i, j, dir));
            corners.push((i, j, dir));
            guard = self.next_state(guard);
        }
        let is_loop = match guard {
            GuardState::Gone => {
                if let Some(&(i, j, dir)) = corners.last() {
                    let last = match dir {
                        Up => (0, j, Up),
                        Down => (self.num_rows() - 1, j, Down),
                        Left => (i, 0, Left),
                        Right => (i, self.num_cols() - 1, Right)
                    };
                    corners.push(last);
                }
                false
            },
            _ => true
        };

        Path { corners, is_loop }
    }

}

/// Representation of the original board (as directly parsed from the input).
///
/// The up/down/right/left vector arrays contain the next "corner" from each position on the board
/// for the given direction.
#[derive(Debug)]
struct OriginalBoard {
    rows: usize,
    cols: usize,
    up: Vec<Vec<GuardState>>,
    down: Vec<Vec<GuardState>>,
    left: Vec<Vec<GuardState>>,
    right: Vec<Vec<GuardState>>
}

impl OriginalBoard {
    fn from_tokens(tokens: &Vec<Vec<Token>>) -> OriginalBoard {
        let rows = tokens.len();
        let cols = tokens[0].len();

        let mut up: Vec<Vec<GuardState>> = {
            let single_row: Vec<GuardState> = (0..cols).map(|_| GuardState::Gone).collect();
            (0..rows).map(|_| single_row.clone()).collect()
        };
        let mut down = up.clone();
        let mut left = up.clone();
        let mut right = up.clone();

        // Up
        for j in 0..cols {
            let mut cur = GuardState::Gone;
            for i in 0..rows {
                match tokens[i][j] {
                    Token::Obstruction => cur = GuardState::Present(i + 1, j, Right),
                    _ => up[i][j] = cur
                }
            }
        }

        // Down
        for j in 0..cols {
            let mut cur = GuardState::Gone;
            for i in (0..rows).rev() {
                match tokens[i][j] {
                    Token::Obstruction => {
                        cur = if i > 0 {
                            GuardState::Present(i - 1, j, Left)
                        } else { GuardState::Gone };
                    },
                    _ => down[i][j] = cur
                }
            }
        }

        // Left
        for i in 0..rows {
            let mut cur = GuardState::Gone;
            for j in 0..cols {
                match tokens[i][j] {
                    Token::Obstruction => cur = GuardState::Present(i, j + 1, Up),
                    _ => left[i][j] = cur

                }
            }
        }

        // Right
        for i in 0..rows {
            let mut cur = GuardState::Gone;
            for j in (0..cols).rev() {
                match tokens[i][j] {
                    Token::Obstruction => {
                        cur = if j > 0 { GuardState::Present(i, j - 1, Down) }
                            else { GuardState::Gone };
                    },
                    _ => right[i][j] = cur
                }
            }
        }

        OriginalBoard { rows, cols, up, down, left, right }
    }
}

impl Board for OriginalBoard {
    fn next_state(&self, guard: GuardState) -> GuardState {
        match guard {
            GuardState::Gone => GuardState::Gone,
            GuardState::Present(i, j, dir) => {
                match dir {
                    Up => self.up[i][j],
                    Down => self.down[i][j],
                    Left => self.left[i][j],
                    Right => self.right[i][j]
                }
            }
        }
    }

    fn num_rows(&self) -> usize {
        self.rows
    }

    fn num_cols(&self) -> usize {
        self.cols
    }
}

/// Board that represents adding one additional obstruction to the original board.
struct AugmentedBoard<'a> {
    row: usize,
    col: usize,
    orig: &'a OriginalBoard
}

impl<'a> AugmentedBoard<'a> {
    fn from(orig: &'a OriginalBoard, row: usize, col: usize) -> AugmentedBoard<'a> {
        AugmentedBoard { row, col, orig }
    }
}

impl<'a> Board for AugmentedBoard<'a> {
    fn next_state(&self, guard: GuardState) -> GuardState {
        match guard {
            GuardState::Present(i, j, Up) if j == self.col && i > self.row => {
                match self.orig.next_state(guard) {
                    g@ GuardState::Present(x, _, _) if x > self.row => g,
                    _ => GuardState::Present(self.row + 1, j, Right)
                }
            }
            GuardState::Present(i, j, Down) if j == self.col && i < self.row => {
                match self.orig.next_state(guard) {
                    g@ GuardState::Present(x, _, _) if x < self.row => g,
                    _ => GuardState::Present(self.row - 1, j, Left)
                }
            }
            GuardState::Present(i, j, Left) if i == self.row && j > self.col => {
                match self.orig.next_state(guard) {
                    g@ GuardState::Present(_, y, _) if y > self.col => g,
                    _ => GuardState::Present(i, self.col + 1, Up)
                }
            },
            GuardState::Present(i, j, Right) if i == self.row && j < self.col => {
                match self.orig.next_state(guard) {
                    g@ GuardState::Present(_, y, _) if y < self.col => g,
                    _ => GuardState::Present(i, self.col - 1, Down)
                }
            },
            guard => self.orig.next_state(guard)
        }
    }

    fn num_rows(&self) -> usize {
        self.orig.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.orig.num_cols()
    }
}

fn parse_input(input: &str) -> (OriginalBoard, GuardState) {
    fn parser(i: &str) -> IResult<&str, Vec<Vec<Token>>> {
        separated_list1(
            newline,
            many1(
                alt(
                    (
                        value(Token::Empty, char('.')),
                        value(Token::Obstruction, char('#')),
                        value(Token::Guard(Up), char('^')),
                        value(Token::Guard(Down), char('v')),
                        value(Token::Guard(Left), char('<')),
                        value(Token::Guard(Right), char('>'))
                    )
                )
            )
        )(i)
    }

    let tokens = parser(input).unwrap().1;
    let base = OriginalBoard::from_tokens(&tokens);
    let (i, j, dir) = tokens.iter().enumerate()
        .filter_map(|(i, row)|
            row.iter().enumerate()
                .filter_map(|(j, &g)| {
                    match g {
                        Token::Guard(dir) => Some((i, j, dir)),
                        _ => None
                    }
                }).next()
        ).next().unwrap();

    (base, GuardState::Present(i, j, dir))
}

fn part1(input: &str) -> usize {
    let (base, guard) = parse_input(input);
    base.path_from(guard).visited_spaces().len()
}

fn part2(input: &str) -> usize {
    let (base, guard) = parse_input(input);

    let (row, col) = match guard {
        GuardState::Present(i, j, _) => (i, j),
        _ => panic!("This will always be present at the beginning")
    };

    base.path_from(guard).visited_spaces().into_iter()
        .filter(|&p| p != (row, col))
        .filter(|&(i, j)| AugmentedBoard::from(&base, i, j).path_from(guard).is_loop)
        .count()
}

build_main!("day06.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}