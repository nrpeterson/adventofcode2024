use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use adventofcode2024::build_main_res;

type Res<T> = Result<T, String>;

#[derive(Copy, Clone, Debug)]
enum ComboOperand {
    LiteralZero,
    LiteralOne,
    LiteralTwo,
    LiteralThree,
    RegisterA,
    RegisterB,
    RegisterC
}
use ComboOperand::*;

impl ComboOperand {
    fn from(data: usize) -> Res<ComboOperand> {
        match data {
            0 => Ok(ComboOperand::LiteralZero),
            1 => Ok(ComboOperand::LiteralOne),
            2 => Ok(ComboOperand::LiteralTwo),
            3 => Ok(ComboOperand::LiteralThree),
            4 => Ok(ComboOperand::RegisterA),
            5 => Ok(ComboOperand::RegisterB),
            6 => Ok(ComboOperand::RegisterC),
            _ => Err("Bad data for combo operand".to_owned())
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Adv(ComboOperand),
    Bxl(usize),
    Bst(ComboOperand),
    Jnz(Option<usize>),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand)
}

use Op::*;

impl Op {
    fn from(opcode: usize, operand: Option<usize>) -> Res<Op> {
        match (opcode, operand) {
            (0, Some(data)) => Ok(Adv(ComboOperand::from(data)?)),
            (1, Some(data)) => Ok(Bxl(data)),
            (2, Some(data)) => Ok(Bst(ComboOperand::from(data)?)),
            (3, data) => Ok(Jnz(data)),
            (4, Some(_)) => Ok(Bxc),
            (5, Some(data)) => Ok(Out(ComboOperand::from(data)?)),
            (6, Some(data)) => Ok(Bdv(ComboOperand::from(data)?)),
            (7, Some(data)) => Ok(Cdv(ComboOperand::from(data)?)),
            (other, _) if other <= 7 => Err(format!("Unknown opcode {other}")),
            (other, None) => Err(format!("Opcode {other} requires an operand")),
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
struct Machine {
    data: Vec<usize>,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instr_ptr: usize,
    output: Vec<usize>
}

impl Machine {
    fn new(data: Vec<usize>, register_a: usize, register_b: usize, register_c: usize) -> Machine {
        let instr_ptr = 0;
        let output = Vec::new();
        Machine { data, register_a, register_b, register_c, instr_ptr, output }
    }

    fn eval_operand(&self, operand: ComboOperand) -> Res<usize> {
        match operand {
            LiteralZero => Ok(0),
            LiteralOne => Ok(1),
            LiteralTwo => Ok(2),
            LiteralThree => Ok(3),
            RegisterA => Ok(self.register_a),
            RegisterB => Ok(self.register_b),
            RegisterC => Ok(self.register_c)
        }
    }

    fn step(&mut self) -> Res<Option<usize>> {
        let op_code = self.data[self.instr_ptr];
        let op_data = self.data.get(self.instr_ptr + 1).map(|&x| x);
        let op = Op::from(op_code, op_data)?;

        match op {
            Adv(operand) => {
                let num = self.register_a;
                let denom = 1 << self.eval_operand(operand)?;
                self.register_a = num / denom;
                self.instr_ptr += 2;
                Ok(None)
            }
            Bxl(data) => {
                self.register_b ^= data;
                self.instr_ptr += 2;
                Ok(None)
            },
            Bst(operand) => {
                self.register_b = self.eval_operand(operand)? % 8;
                self.instr_ptr += 2;
                Ok(None)
            },
            Jnz(data) => {
                if self.register_a == 0 {
                    self.instr_ptr += 2;
                } else {
                    self.instr_ptr = data.ok_or("jnz activated without data".to_owned())?;
                }
                Ok(None)
            },
            Bxc => {
                self.register_b ^= self.register_c;
                self.instr_ptr += 2;
                Ok(None)
            },
            Out(operand) => {
                let result = self.eval_operand(operand)? % 8;
                self.output.push(result);
                self.instr_ptr += 2;
                Ok(Some(result))
            },
            Bdv(operand) => {
                let num = self.register_a;
                let denom = 1 << self.eval_operand(operand)?;
                self.register_b = num / denom;
                self.instr_ptr += 2;
                Ok(None)
            },
            Cdv(operand) => {
                let num = self.register_a;
                let denom = 1 << self.eval_operand(operand)?;
                self.register_c = num / denom;
                self.instr_ptr += 2;
                Ok(None)
            }
        }
    }

    fn run(&mut self) -> Res<String> {
        while self.instr_ptr < self.data.len() {
            self.step()?;
        }

        Ok(
            self.output.iter()
                .map(|&x| x.to_string())
                .join(",")
        )
    }
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_machine(input: &str) -> Res<Machine> {
    let parse_result = map(
        tuple((
            preceded(tag("Register A: "), number),
            preceded(tuple((newline, tag("Register B: "))), number),
            preceded(tuple((newline, tag("Register C: "))), number),
            preceded(
                tuple((newline, newline, tag("Program: "))),
                separated_list1(char(','), number)
            )
        )),
        |(register_a, register_b, register_c, data)| {
            Machine::new(data, register_a, register_b, register_c)
        }
    )(input);

    match parse_result {
        Ok((_, machine)) => Ok(machine),
        Err(_) => Err("Failed to parse machine input".to_owned())
    }
}

fn part1(input: &str) -> Res<String> {
    parse_machine(input)?.run()
}

#[derive(Debug)]
struct Step {
    cur_choice: usize,
    rem_choices: Vec<usize>
}

impl Step {
    fn next(&mut self) -> bool {
        if self.rem_choices.is_empty() {
            false
        }
        else {
            self.cur_choice = self.rem_choices.pop().unwrap();
            true
        }
    }
}

impl Step {
    fn new() -> Step {
        Step { cur_choice: 0, rem_choices: (1..8).rev().collect() }
    }
}

fn part2(input: &str) -> Res<usize> {
    let mut stack = Vec::new();
    stack.push(Step::new());

    let base_machine = parse_machine(input)?;
    let target = base_machine.data.clone();

    loop {
        let cur = stack.iter().fold(0, |acc, x| 8*acc + x.cur_choice);
        let mut machine = base_machine.clone();
        machine.register_a = cur;
        machine.run()?;

        if machine.output != target[target.len() - machine.output.len()..] {
            while !stack.last_mut().unwrap().next() {
                stack.pop();
            }
        }  else {
            if target.len() == machine.output.len() {
                return Ok(cur)
            }

            stack.push(Step::new());
        }
    }
}

build_main_res!("day17.txt", "Part 1" => part1, "Part 2" => part2);