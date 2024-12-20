use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::ops::Index;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use adventofcode2024::build_main_res;

#[derive(Copy, Clone, Eq, PartialEq)]
enum SpaceType { Track, Wall }
use SpaceType::*;

type Pos = (usize, usize);

struct Maze {
    start: Pos,
    end: Pos,
    rows: usize,
    cols: usize,
    spaces: Vec<Vec<SpaceType>>
}

impl Index<Pos> for Maze {
    type Output = SpaceType;
    fn index(&self, (i, j): Pos) -> &Self::Output { &self.spaces[i][j] }
}

impl Maze {
    fn adj_tracks(&self, (i, j): Pos) -> Vec<Pos> {
        let mut opts = Vec::new();
        if i > 0 { opts.push((i - 1, j)); }
        if i < self.rows - 1 { opts.push((i + 1, j)); }
        if j > 0 { opts.push((i, j - 1)); }
        if j < self.cols - 1 { opts.push((i, j + 1)); }

        opts.into_iter().filter(|&pos| self[pos] == Track).collect()
    }

    fn tracks_in_radius(&self, (i, j): Pos, r: usize) -> Vec<Pos> {
        let mut result = Vec::new();

        let s0 = if i < 20 { 0 } else { i - 20 };
        let s1 = min(self.rows - 1, i + 20);

        for s in s0..=s1 {
            let r0 = r - s.abs_diff(i);
            let t0 = if j < r0 { 0 } else { j - r0 };
            let t1 = min(self.cols - 1, j + r0);
            for t in t0..=t1 {
                if (s, t) != (i, j) && self.spaces[s][t] == Track {
                    result.push((s, t));
                }
            }
        }

        result
    }

    fn dists(&self, pos: Pos) -> Vec<Vec<usize>> {
        let mut result = vec![vec![usize::MAX; self.cols]; self.rows];
        result[pos.0][pos.1] = 0;

        let mut seen = HashSet::new();
        seen.insert(pos);
        let mut queue = VecDeque::new();
        queue.push_back((pos, 0));

        while let Some((p, dist)) = queue.pop_front() {
            result[p.0][p.1] = dist;
            self.adj_tracks(p).iter()
                .for_each(|&p| {
                    if seen.insert(p) {
                        queue.push_back((p, dist + 1));
                    }
                });
        }

        result
    }
}

fn parse_input(input: &str) -> IResult<&str, Maze> {
    let space = alt((
        value((false, false, Wall), char('#')),
        value((false, false, Track), char('.')),
        value((true, false, Track), char('S')),
        value((false, true, Track), char('E'))
    ));

    let line = map(
        many1(space),
        |spaces| {
            spaces.into_iter()
                .enumerate()
                .fold(
                    (None, None, Vec::new()),
                    |(start, end, mut spaces), (j, (is_start, is_end, space))| {
                        let this_start = if is_start { Some(j) } else { None };
                        let this_end = if is_end { Some(j) } else { None };
                        let new_start = start.or(this_start);
                        let new_end = end.or(this_end);
                        spaces.push(space);
                        (new_start, new_end, spaces)
                    }
                )
        }
    );

    map(
        separated_list1(newline, line),
        |lines| {
            let (start_opt, end_opt, spaces) = lines.into_iter()
                .enumerate()
                .fold(
                    (None, None, Vec::new()),
                    |(start, end, mut spaces), (i, (start0, end0, line))| {
                        let this_start = start0.map(|j| (i, j));
                        let this_end = end0.map(|j| (i, j));
                        let new_start = start.or(this_start);
                        let new_end = end.or(this_end);
                        spaces.push(line);
                        (new_start, new_end, spaces)
                    }
                );

            let start = start_opt.expect("Must have start");
            let end = end_opt.expect("Must have end");
            let rows = spaces.len();
            let cols = spaces[0].len();
            Maze { start, end, rows, cols, spaces }
        }
    )(input)
}

fn part1(input: &str) -> Result<usize, String> {
    let (_, maze) = parse_input(input).map_err(|_| "Failed to parse".to_owned())?;
    let to_end = maze.dists(maze.end);
    let from_start = maze.dists(maze.start);

    let honest = from_start[maze.end.0][maze.end.1];

    let result = (0..maze.rows).cartesian_product(0..maze.cols)
        .filter(|&p| maze[p] == Wall)
        .flat_map(|p| maze.adj_tracks(p).into_iter().permutations(2))
        .map(|ps| {
            let (i0, j0) = ps[0];
            let (i1, j1) = ps[1];
            from_start[i0][j0] + 1 + to_end[i1][j1]
        })
        .filter(|&new_dist| new_dist + 100 <= honest)
        .count();

    Ok(result)
}

fn part2(input: &str) -> Result<usize, String> {
    let (_, maze) = parse_input(input).map_err(|_| "Failed to parse".to_owned())?;
    let to_end = maze.dists(maze.end);
    let from_start = maze.dists(maze.start);
    let honest = from_start[maze.end.0][maze.end.1];

    let result = (0..maze.cols).cartesian_product(0..maze.rows)
        .filter(|&p| maze[p] == Track)
        .flat_map(|p| {
            maze.tracks_in_radius(p, 20).into_iter().map(move |p0| (p, p0))
        })
        .map(|((i0, j0), (i1, j1))| {
            let dist = i0.abs_diff(i1) + j0.abs_diff(j1);
            from_start[i0][j0] + dist + to_end[i1][j1] - 1
        })
        .filter(|&new_dist| new_dist + 100 <= honest)
        .count();

    Ok(result)
}

build_main_res!("day20.txt", "Part 1" => part1, "Part 2" => part2);