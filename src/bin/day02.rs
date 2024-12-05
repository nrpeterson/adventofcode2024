use adventofcode2024::build_main;
use itertools::Itertools;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let result: IResult<&str, Vec<Vec<usize>>> = separated_list1(
        newline,
        separated_list1(space1, map_res(digit1, |d: &str| d.parse::<usize>())),
    )(input);
    result.unwrap().1
}

fn no_dir_change(a: usize, b: usize, c: usize) -> bool {
    (a < b && b < c) || (a > b && b > c)
}

fn is_gradual(a: usize, b: usize) -> bool {
    a != b && a.abs_diff(b) <= 3
}

fn is_safe(vec: &[usize]) -> bool {
    Hazards::of(vec).is_empty()
}

#[derive(Debug)]
struct Hazards {
    gaps: Vec<(usize, usize)>,
    flats: Vec<(usize, usize)>,
    direction_changes: Vec<(usize, usize, usize)>,
}

impl Hazards {
    fn of(v: &[usize]) -> Hazards {
        let mut gaps = Vec::new();
        let mut flats = Vec::new();
        let mut direction_changes = Vec::new();

        for (i, (&a, &b)) in v.iter().tuple_windows().enumerate() {
            if a.abs_diff(b) > 3 {
                gaps.push((i, i + 1));
            };
            if a == b {
                flats.push((i, i + 1));
            };
        }

        for (i, (&a, &b, &c)) in v.iter().tuple_windows().enumerate() {
            if a != b && b != c && ((b > a && b > c) || (b < a && b < c)) {
                direction_changes.push((i, i + 1, i + 2));
            };
        }

        Hazards {
            gaps,
            flats,
            direction_changes,
        }
    }

    fn is_empty(&self) -> bool {
        self.gaps.is_empty() && self.flats.is_empty() && self.direction_changes.is_empty()
    }

    fn removing_fixes_flats_gaps(&self, i: usize) -> bool {
        self.gaps.iter().all(|&(a, b)| a == i || b == i) &&
            self.flats.iter().all(|&(a, b)| a == b)
    }
}

fn is_almost_safe(v: &Vec<usize>) -> bool {
    //! Check whether v is either safe, OR can be made safe by the removal of a single level.
    //!
    //! We do this by computing all the 'hazards' (flats, gaps, strict direction changes), and
    //! considering them:
    //! - If there are no direction changes, the only things you can fix are either a flat on the
    //!     interior of the list or a flat or gap at the beginning or end.
    //! - If there is exactly one direction change, it can only be fixed if it is at the start or
    //!     end (because you must remove an entire segment going the wrong direction, thus that
    //!     segment must have length 1).
    //! - If there are exactly two direction changes and they are adjacent, you can try to fix it
    //!     by removing either of the two 'middle' elements (e.g. 1 3 2 4 -- try removing 3 or 2)
    //! - If there are two non-adjacent direction changes, or three or more total, then you can't
    //!     fix it.
    let hazards = Hazards::of(v);

    // No hazards, no problems
    if hazards.is_empty() {
        true
    }
    // We can only resolve one gap or flat by a removal; so, if we have two or more, no dice.
    else if hazards.flats.len() + hazards.gaps.len() > 1 {
        false
    }
    // We can only resolve two direction changes by a removal; so, if we have three or more, no dice
    else if hazards.direction_changes.len() > 2 {
        false
    }
    else if hazards.direction_changes.len() == 2 {
        let (_, b, c) = hazards.direction_changes[0];
        let (d, e, _) = hazards.direction_changes[1];
        if b == d && c == e {
            (hazards.removing_fixes_flats_gaps(c)
                && is_gradual(v[c-1], v[c+1])
                && (c+2 == v.len() || no_dir_change(v[c-1], v[c+1], v[c+2]))
                && no_dir_change(v[c-2], v[c-1], v[c+1])
            ) ||
                (hazards.removing_fixes_flats_gaps(b)
                    && is_gradual(v[b-1], v[b+1])
                    && (b <= 1 || no_dir_change(v[b-2], v[b-1], v[b+1]))
                    && no_dir_change(v[b-1], v[b+1], v[b+2])
                )
        }
        else {
            false
        }
    }
    else if hazards.direction_changes.len() == 1 {
        let (a, _, c) = hazards.direction_changes[0];
        if a == 0 {
            (hazards.removing_fixes_flats_gaps(1) && is_gradual(v[0], v[2]))
            || hazards.removing_fixes_flats_gaps(0)
        }
        else if c == v.len() - 1 {
            (hazards.removing_fixes_flats_gaps(c - 1) && is_gradual(v[c-2], v[c]))
            || hazards.removing_fixes_flats_gaps(c)
        }
        else {
            false
        }
    }
    else {
        // We now know there are no direction changes, and at most one flat or gap.
        if hazards.flats.is_empty() && hazards.gaps.is_empty() {
            true
        }
        else if hazards.gaps.is_empty() {
            let (a, b) = hazards.flats[0];
            if a == 0 || b == v.len() - 1 {
                true
            }
            else {
                let x = v[a-1];
                let y = v[a];
                let z = v[a+2];
                no_dir_change(x, y, z)
            }
        }
        else {
            let (a, b) = hazards.gaps[0];
            a == 0 || b == v.len() - 1
        }
    }
}

fn part1(input: &str) -> usize {
    parse_input(input).iter().filter(|&v| is_safe(v)).count()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .iter().filter(|&v| is_almost_safe(v))
        .count()
}

build_main!("day02.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn test_part_one() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
