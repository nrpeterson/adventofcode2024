use adventofcode2024::build_main;

#[derive(Debug)]
struct Button { x: isize, y: isize }

#[derive(Debug)]
struct Machine { button_a: Button, button_b: Button, prize: (isize, isize) }

mod parse {
    use nom::IResult;
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, tuple};
    use super::{Button, Machine};

    fn number(input: &str) -> IResult<&str, isize> {
        map_res(digit1, str::parse::<isize>)(input)
    }

    fn machine(input: &str) -> IResult<&str, Machine> {
        let a_button = map(
            tuple(
                (
                    preceded(tag("Button A: X+"), number),
                    preceded(tag(", Y+"), number)
                )
            ),
            |(x, y)| Button { x, y }
        );

        let b_button = map(
            tuple(
                (
                    preceded(tag("Button B: X+"), number),
                    preceded(tag(", Y+"), number)
                )
            ),
            |(x, y)| Button { x, y }
        );

        let prize = tuple(
            (
                preceded(tag("Prize: X="), number),
                preceded(tag(", Y="), number)
            )
        );

        map(
            tuple(
                (
                    a_button,
                    preceded(newline, b_button),
                    preceded(newline, prize)
                )
            ),
            |(button_a, button_b, prize)| Machine { button_a, button_b, prize }
        )(input)
    }

    pub(crate) fn parse_input(input: &str) -> Vec<Machine> {
        separated_list1(tuple((newline, newline)), machine)(input).unwrap().1
    }

}

#[derive(Debug)]
struct ExtendedEuclidean { gcd: isize, bezout_coeffs: (isize, isize) }
fn extended_euclidean(a: isize, b: isize) -> ExtendedEuclidean {
    let mut r_prev = a;
    let mut r_cur = b;
    let mut s_prev = 1;
    let mut s_cur = 0;
    let mut t_prev = 0;
    let mut t_cur = 1;

    while r_cur != 0 {
        let q = r_prev / r_cur;
        (r_prev, r_cur) = (r_cur, r_prev - q * r_cur);
        (s_prev, s_cur) = (s_cur, s_prev - q * s_cur);
        (t_prev, t_cur) = (t_cur, t_prev - q * t_cur);
    }

    ExtendedEuclidean { gcd: r_prev, bezout_coeffs: (s_prev, t_prev) }
}

/// Solutions to a linear Diophantine equation in two variables ax+by=c.
///
/// They take the form `(x, y) = (x0 + kv, y0 - ku)` where:
/// - `(x0, y0)` is any solution (found e.g. by the extended Euclidean algorithm)
/// - `u = a/d` and `v=b/d`, where `d:=gcd(a, b)`
/// - `k` is any integer
///
/// We'll normalize so that u >= 0.
struct DiophantineSols {
    problem: (isize, isize, isize),
    x0: isize,
    y0: isize,
    u: isize,
    v: isize
}

impl DiophantineSols {
    /// Find solutions to ax+by=c
    fn new(a: isize, b: isize, c: isize) -> Option<DiophantineSols> {
        let ee = extended_euclidean(a, b);

        if c % ee.gcd != 0 {
            return None
        }

        let multiplier = c / ee.gcd;
        let (bezout_m, bezout_n) = ee.bezout_coeffs;
        let x0 = bezout_m * multiplier;
        let y0 = bezout_n * multiplier;

        let u0 = a / ee.gcd;
        let v0 = b / ee.gcd;

        let (u, v) = if u0 < 0 { (-u0, -v0) } else { (u0, v0) };

        Some(DiophantineSols { problem: (a, b, c), x0, y0, u, v })
    }

    fn nonneg_min_x(&self) -> Option<(isize, isize)> {
        // We'll only handle the case relevant to this problem, where we solve ax+by=c and
        // a, b, c > 0.  This means that u and v will have the same signs (and we've normalized
        // to u > 0).
        assert!(self.u > 0 && self.v > 0);

        if self.x0 >= 0 {
            // How many times can we subtract v without becoming negative?
            let k = self.x0 / self.v;
            let (x, y) = (self.x0 - k * self.v, self.y0 + k * self.u);

            let (a, b, c) = self.problem;
            assert_eq!(a*x + b*y, c);

            if y >= 0 { Some((x, y)) } else { None }
        }
        else {
            // How many times must we add v to become nonnegative?
            let k = if self.x0 % self.v == 0 {
                self.x0.abs() / self.v
            } else {
                self.x0.abs() / self.v + 1
            };

            let (x, y) = (self.x0 + k * self.v, self.y0 - k * self.u);

            let (a, b, c) = self.problem;
            assert_eq!(a*x + b*y, c);

            if y >= 0 { Some((x, y)) } else { None }
        }
    }

    fn nonneg_min_y(&self) -> Option<(isize, isize)> {
        // We'll only handle the case relevant to this problem, where we solve ax+by=c and
        // a, b, c > 0.  This means that u and v will have the same signs (and we've normalized
        // to u > 0).
        assert!(self.u > 0 && self.v > 0);

        if self.y0 >= 0 {
            // How many times can we subtract u without becoming negative?
            let k = self.y0 / self.u;
            let (x, y) = (self.x0 + k * self.v, self.y0 - k * self.u);

            let (a, b, c) = self.problem;
            assert_eq!(a*x + b*y, c);

            if y >= 0 { Some((x, y)) } else { None }
        }
        else {
            // How many times must we add u to become nonnegative?
            let k = if self.y0 % self.u == 0 {
                self.y0.abs() / self.u
            } else {
                self.y0.abs() / self.u + 1
            };

            let (x, y) = (self.x0 + k * self.v, self.y0 - k * self.u);

            let (a, b, c) = self.problem;
            assert_eq!(a*x + b*y, c);

            if y >= 0 { Some((x, y)) } else { None }
        }
    }
}

fn min_solution_cost(machine: &Machine) -> Option<isize> {
    let Button { x: a_x, y: a_y } = machine.button_a;
    let Button { x: b_x, y: b_y } = machine.button_b;
    let (p_x, p_y) = machine.prize;

    let det = a_x * b_y - a_y * b_x;

    if det != 0 {
        // Only one possible solution, given by X=A^{-1}B where
        // A^{-1} = (1/det) * [[b_y, -b_x], [-a_y, a_x]].
        // Check to see if this solution gives non-negative integers.
        let m_det = b_y * p_x - b_x * p_y;
        let n_det = -a_y * p_x + a_x * p_y;

        if m_det % det == 0 && n_det % det == 0 {
            let m = m_det / det;
            let n = n_det / det;

            if m >= 0 && n >= 0 { Some(3 * m + n) } else { None }
        }
        else {
            None
        }
    }
    else {
        // The x's and y's are proportionate. So there are no solutions if the prize location
        // doesn't match these proportions, and infinitely many if it does

        if a_x * p_y - a_y * p_x != 0 {
            None
        }
        else {
            // Just solve for the x's now.  Need a_x * m + b_x * n = p_x.
            // Note that we can write n = (p_x - a_x*m) / b_x.
            // Then our cost is 3m+n; taking n as a function of m, this has derivative
            // 3-a_x/b_x. So, if a_x/b_x<3, we want to choose the solution with minimal m, while
            // if a_x/b_x>3, we want to choose the solution with maximal m.
            // If a_x/b_x=3, then any solution gives the same score.

            let dio = DiophantineSols::new(a_x, b_x, p_x)?;
            let (m, n) = if a_x >= 3 * b_x {
                dio.nonneg_min_y()?
            }
            else {
                dio.nonneg_min_x()?
            };

            Some(3 * m + n)
        }
    }

}

fn part1(input: &str) -> isize {
    let machines = parse::parse_input(input);

    machines.iter().filter_map(min_solution_cost).sum()
}

fn part2(input: &str) -> isize {
    let mut machines = parse::parse_input(input);

    machines.iter_mut().for_each(|m| {
        m.prize.0 += 10000000000000;
        m.prize.1 += 10000000000000;
    });

    machines.iter().filter_map(min_solution_cost).sum()
}

build_main!("day13.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::part1;

    // For this input, Xs/Ys are proportionate (so det 0). Looking at Xs, we need 22 copies of 11
    // to get to 242, and A gives us 2 while B gives us 3.  Since A is 3x the cost, we want to use
    // as many B's as possible; so, we'll use 6 B's and 2 A's for total cost of 12.
    //
    // This is a weak test of the diophantine solution code... which isn't exercised at all by
    // the actual problem input. :-(
    const DIOPHANTINE_TEST_INPUT: &str =
"Button A: X+22, Y+4
Button B: X+33, Y+6
Prize: X=242, Y=44";

    #[test]
    fn test_diophantine() {
        assert_eq!(part1(DIOPHANTINE_TEST_INPUT), 12);
    }

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 480);
    }
}