use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::thread::current;
use std::time::SystemTime;

use itertools;
use itertools::{Either, Itertools};
use parse_display::helpers::regex::internal::Inst;

use crate::Either::{Left, Right};
use crate::Instruction::{ADD, DIV, EQL, INP, MOD, MUL};
use crate::Variable::{W, X, Y, Z};

const E1: &str = "inp x\nmul x -1";
const E2: &str = "inp z\ninp x\nmul z 3\neql z x";
const E3: &str = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";
const E4: &str = "";
const INPUT: &str = "inp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 10\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 13\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 13\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 10\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 13\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 3\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -11\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 1\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 11\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 9\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -4\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 3\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 12\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 5\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 12\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 1\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 1\nadd x 15\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 0\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -2\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 13\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -5\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 7\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -11\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 15\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -13\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 12\nmul y x\nadd z y\ninp w\nmul x 0\nadd x z\nmod x 26\ndiv z 26\nadd x -10\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y 8\nmul y x\nadd z y";

fn part1() {
    let mut input_values: Vec<u8> = vec![7];

    let mut lower_bound = 0;
    let mut upper_bound = 1_000_000;
    let mut ub = [1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 1, 1];
    ub.reverse();

    let mut upper_bound = [9u8; 14].to_vec();

    // 1st way took 125.79ms for 1000 iterations

    let program = get_program(INPUT);

    // println!("{}", is_program_valid(&mut ub));

    let start_time = SystemTime::now();
    let mut count = 0;
    loop {
        println!("{:?}", upper_bound);
        let is_valid = is_program_valid(&program, &mut upper_bound);

        if count == 1_000_000 {
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time).unwrap();
            println!("{:?}", duration);
            break;
        }
        count += 1;

        if is_valid { break; }
        decrease_number(&mut upper_bound);
    }

    println!("{:?}", upper_bound);
}

fn decrease_number(number: &mut Vec<u8>) {
    let mut index_not_1 = 0;
    for i in 0..14 {
        if number[i] != 1 {
            index_not_1 = i;
            break;
        }
    }

    for i in 0..index_not_1 {
        number[i] = 9;
    }
    number[index_not_1] -= 1;
}

#[derive(Copy, Clone)]
enum Variable { W, X, Y, Z }

impl Variable {
    fn i(&self) -> usize {
        match self {
            W => 0,
            X => 1,
            Y => 2,
            Z => 3
        }
    }
}

enum Instruction {
    INP(Variable),
    ADD(Variable, Either<Variable, i16>),
    MUL(Variable, Either<Variable, i16>),
    DIV(Variable, Either<Variable, i16>),
    MOD(Variable, Either<Variable, i16>),
    EQL(Variable, Either<Variable, i16>),
}

fn get_variable(var: &str) -> Variable {
    match var {
        "w" => W,
        "x" => X,
        "y" => Y,
        "z" => Z,
        _ => panic!()
    }
}

fn get_variable_or_number(s: &str) -> Either<Variable, i16> {
    let result = s.parse::<i16>();

    if let Ok(number) = result {
        Right(number)
    } else {
        Left(get_variable(s))
    }
}

fn get_program(program: &str) -> Vec<Instruction> {
    let mut resulting_program = Vec::new();

    for line in program.lines() {
        let line: Vec<&str> = line.split(" ").collect();

        match line[0] {
            "inp" => {
                resulting_program.push(INP(get_variable(line[1])))
            }
            "add" => {
                resulting_program.push(ADD(get_variable(line[1]), get_variable_or_number(line[2])));
            }
            "div" => {
                resulting_program.push(DIV(get_variable(line[1]), get_variable_or_number(line[2])));
            }
            "mod" => {
                resulting_program.push(MOD(get_variable(line[1]), get_variable_or_number(line[2])));
            }
            "mul" => {
                resulting_program.push(MUL(get_variable(line[1]), get_variable_or_number(line[2])));
            }
            "eql" => {
                resulting_program.push(EQL(get_variable(line[1]), get_variable_or_number(line[2])));
            }
            _ => unimplemented!()
        }
    }

    resulting_program
}

fn is_program_valid(program: &Vec<Instruction>, input_values: &Vec<u8>) -> bool {
    let (_, _, _, z) = run_program(program, input_values);
    z == 0
}

fn run_program(program: &Vec<Instruction>, input_values: &Vec<u8>) -> (i64, i64, i64, i64) {
    let mut variables: [i64; 4] = [0, 0, 0, 0]; // w, x, y, z

    let mut next_input_index = input_values.len() - 1;

    for instr in program {
        match instr {
            INP(a) => {
                let input = input_values[next_input_index];
                if next_input_index != 0 { next_input_index -= 1 };
                variables[a.i()] = input as i64;
            }
            ADD(a, b) => {
                variables[a.i()] += if b.is_left() { variables[b.unwrap_left() as usize] } else { b.right().unwrap() as i64 };
            }
            MUL(a, b) => {
                variables[a.i()] *= if b.is_left() { variables[b.unwrap_left().i() as usize] } else { b.right().unwrap() as i64 };
            }
            DIV(a, b) => {
                variables[a.i()] /= if b.is_left() { variables[b.unwrap_left().i() as usize] } else { b.right().unwrap() as i64 };
            }
            MOD(a, b) => {
                variables[a.i()] %= if b.is_left() { variables[b.unwrap_left().i() as usize] } else { b.right().unwrap() as i64 };
            }
            EQL(a, b) => {
                let a_val = variables[a.i()];
                let b_val = if b.is_left() { variables[b.left().unwrap().i()] } else { b.right().unwrap() as i64 };
                variables[a.i()] = if a_val == b_val { 1 } else { 0 };
            }
        }
    }

    // for line in program.lines() {
    //     let line: Vec<&str> = line.split(" ").collect();
    //
    //     match line[0] {
    //         "inp" => {
    //             let input = input_values[next_input_index];
    //             if next_input_index != 0 { next_input_index -= 1 };
    //             *variables.get_mut(line[1]).unwrap() = input as i64;
    //         }
    //         "add" => {
    //             *variables.get_mut(line[1]).unwrap() += line[2].parse::<i64>()
    //                 .unwrap_or_else(|_| *variables.get(line[2]).unwrap());
    //         }
    //         "div" => {
    //             *variables.get_mut(line[1]).unwrap() /= line[2].parse::<i64>()
    //                 .unwrap_or_else(|_| *variables.get(line[2]).unwrap());
    //         }
    //         "mod" => {
    //             *variables.get_mut(line[1]).unwrap() %= line[2].parse::<i64>()
    //                 .unwrap_or_else(|_| *variables.get(line[2]).unwrap());
    //         }
    //         "mul" => {
    //             *variables.get_mut(line[1]).unwrap() *= line[2].parse::<i64>()
    //                 .unwrap_or_else(|_| *variables.get(line[2]).unwrap());
    //         }
    //         "eql" => {
    //             let v1 = *variables.get(line[1]).unwrap();
    //             let v2 = line[2].parse::<i64>().unwrap_or_else(|_| *variables.get(line[2]).unwrap());
    //             *variables.get_mut(line[1]).unwrap() = if v1 == v2 { 1 } else { 0 };
    //         }
    //         _ => unimplemented!()
    //     }
    //
    //     // println!("{:?}", variables);
    // }

    // (*variables.get("w").unwrap(),
    //  *variables.get("x").unwrap(),
    //  *variables.get("y").unwrap(),
    //  *variables.get("z").unwrap())
    (variables[0], variables[1], variables[2], variables[3])
}

fn part2() {}

fn main() {
    part1();
}