use itertools::Itertools;
use adventofcode2024::build_main;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

const DIRECTIONS: [Direction; 8] = [Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight];
use Direction::*;

fn is_xmas(puz: &Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> bool {
    let rows = puz.len() as isize;
    let cols = puz[0].len() as isize;
    let i = i as isize;
    let j = j as isize;

    let (di, dj) = match dir {
        Up => (-1, 0),
        Down => (1, 0),
        Left => (0, -1),
        Right => (0, 1),
        UpLeft => (-1, -1),
        UpRight => (-1, 1),
        DownLeft => (1, -1),
        DownRight => (1, 1)
    };

    if (di < 0 && i < 3) || (dj < 0 && j < 3) || (di > 0 && i + 4 > rows) || (dj > 0 && j > cols - 4) {
        false
    }
    else {
        let is = [
            (i as usize, j as usize),
            ((i + di) as usize, (j + dj) as usize),
            ((i + 2*di) as usize, (j + 2*dj) as usize),
            ((i + 3*di) as usize, (j + 3*dj) as usize)
        ];

        is.iter().zip(['X', 'M', 'A', 'S'])
            .all(|(&(a, b), c)| puz[a][b] == c)
    }
}


fn part1(input: &str) -> usize {
    let puzzle: Vec<Vec<char>> = parse_input(input);
    let rows = puzzle.len();
    let cols = puzzle[0].len();

    let mut result = 0;

    for i in 0..rows {
        for j in 0..cols {
            if puzzle[i][j] == 'X' {
                for dir in DIRECTIONS {
                    if is_xmas(&puzzle, i, j, dir) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn get_x(puzzle: &Vec<Vec<char>>, i: usize, j: usize) -> [char; 5] {
    //! For the following:
    //! A . B
    //! . C .
    //! D . E
    //!
    //! returns `[A, B, C, D, E]`
    [(i-1, j-1), (i-1, j+1), (i, j), (i+1, j-1), (i+1, j+1)].map(|(i, j)| puzzle[i][j])
}

const GOOD_XS: [[char; 5]; 4] = [
    ['M', 'M', 'A', 'S', 'S'],
    ['M', 'S', 'A', 'M', 'S'],
    ['S', 'M', 'A', 'S', 'M'],
    ['S', 'S', 'A', 'M', 'M']
];

fn part2(input: &str) -> usize {
    let puzzle: Vec<Vec<char>> = parse_input(input);
    let rows = puzzle.len();
    let cols = puzzle[0].len();

    (1..rows-1).cartesian_product(1..cols-1)
        .filter(|&(i, j)| puzzle[i][j] == 'A')
        .map(|(i, j)| get_x(&puzzle, i, j))
        .filter(|x| GOOD_XS.contains(x))
        .count()
}

build_main!("day04.txt", "Part 1" => part1, "Part 2" => part2);