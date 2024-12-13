use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use adventofcode2024::build_main;

struct Region {
    plots: HashSet<(usize, usize)>,
    perimeter: usize,
    area: usize,
    corners: usize
}

impl Region {
    fn new() -> Region {
        Region {
            plots: HashSet::new(),
            perimeter: 0,
            area: 0,
            corners: 0
        }
    }
}

struct Square {
    plot_type: char,
    up: Option<((usize, usize), char)>,
    up_right: Option<((usize, usize), char)>,
    right: Option<((usize, usize), char)>,
    down_right: Option<((usize, usize), char)>,
    down: Option<((usize, usize), char)>,
    down_left: Option<((usize, usize), char)>,
    left: Option<((usize, usize), char)>,
    up_left: Option<((usize, usize), char)>
}

fn join_with<A, B, C, F>(a_opt: Option<A>, b_opt: Option<B>, f: F) -> Option<C> where
    F: Fn(A, B) -> C {
    let a = a_opt?;
    let b = b_opt?;
    Some(f(a, b))
}

impl Square {
    fn of(plot: (usize, usize), garden: &Vec<Vec<char>>) -> Square {
        let (s, t) = plot;
        let plot_type = garden[s][t];

        let i = Some(s);
        let j = Some(t);
        let i_up = if s > 0 { Some(s - 1) } else { None };
        let i_down = if s < garden.len() - 1 { Some(s + 1) } else { None };
        let j_left = if t > 0 { Some(t - 1) } else { None };
        let j_right = if t < garden[0].len() - 1 { Some(t + 1) } else { None };

        let f = |x, y| -> ((usize, usize), char) { ((x, y), garden[x][y]) };

        Square {
            plot_type,
            up: join_with(i_up, j, f),
            up_right: join_with(i_up, j_right, f),
            right: join_with(i, j_right, f),
            down_right: join_with(i_down, j_right, f),
            down: join_with(i_down, j, f),
            down_left: join_with(i_down, j_left, f),
            left: join_with(i, j_left, f),
            up_left: join_with(i_up, j_left, f)
        }
    }

    fn neighbors(&self) -> Vec<((usize, usize), char)> {
        vec![self.up, self.right, self.down, self.left].into_iter()
            .filter_map(|x| x)
            .collect()
    }

    fn matching_neighbors(&self) -> Vec<(usize, usize)> {
        self.neighbors().into_iter()
            .filter(|&(_, c)| c == self.plot_type)
            .map(|(x, _)| x)
            .collect()
    }

    fn corner_triples(&self) -> [[bool; 3]; 4] {
        let opts = [
            self.up, self.up_right, self.right, self.down_right,
            self.down, self.down_left, self.left, self.up_left];

        let bools = opts.map(|opt| {
            opt.filter(|&(_, c)| c == self.plot_type).is_some()
        });

        [
            [bools[0], bools[1], bools[2]],
            [bools[2], bools[3], bools[4]],
            [bools[4], bools[5], bools[6]],
            [bools[6], bools[7], bools[0]]
        ]
    }

    fn num_corners(&self) -> usize {
        //! What can corners look like? Let's use top left corner as example. Possible configurations
        //! (where X matches and O doesn't) are:
        //!
        //! XX   XX   XO   XO   OX   OX   OO   OO
        //! X*   O*   X*   O*   X*   O*   X*   O*
        //! No   No   No   Yes  Yes  No   No   Yes
        //!
        //! So if in clockwise order those spaces matching/non-matching are bools `[a, b, c]`, then
        //! we get corners precisely when a == c and at least one element of the triple is false.
        self.corner_triples().iter()
            .filter(|&&[a, b, c]| a == c && !(a && b && c))
            .count()
    }
}

fn regions(garden: &Vec<Vec<char>>) -> Vec<Region> {
    let mut seen = HashSet::new();
    let rows = garden.len();
    let cols = garden[0].len();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    for (i, j) in (0..rows).cartesian_product(0..cols) {
        if seen.contains(&(i, j)) {
            continue;
        }

        let mut region = Region::new();

        queue.push_back((i, j));
        seen.insert((i, j));

        while let Some(plot) = queue.pop_front() {
            region.plots.insert(plot);
            region.area += 1;

            let square = Square::of(plot, garden);
            let neighbors = square.matching_neighbors();

            neighbors.iter().for_each(|&neighbor| {
                if !seen.contains(&neighbor) {
                    seen.insert(neighbor);
                    queue.push_back(neighbor);
                }
            });

            region.perimeter += 4 - neighbors.len();
            region.corners += square.num_corners();

        }

        result.push(region);
    }

    result
}


fn part1(input: &str) -> usize {
    let garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    regions(&garden).iter().map(|r| r.area * r.perimeter).sum()
}

fn part2(input: &str) -> usize {
    let garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    regions(&garden).iter().map(|r| r.area * r.corners).sum()
}

build_main!("day12.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1206);
    }
}