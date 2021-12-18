use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ptr::write;
use std::str::{Chars, FromStr};
use std::string::ParseError;
use std::thread::current;

use itertools::{fold, Itertools};
use phf::phf_map;
use tailcall::tailcall;

const E1: &str = "target area: x=20..30, y=-10..-5";
const E2: &str = "";
const E3: &str = "";
const E4: &str = "";
const INPUT: &str = "target area: x=248..285, y=-85..-56";

fn part1() {
    let (_, _, mut xs, mut ys): (&str, &str, &str, &str) = INPUT.split(" ").collect_tuple().unwrap();
    xs = xs.split("=").last().unwrap().split(",").nth(0).unwrap();
    ys = ys.split("=").last().unwrap();

    let (from_x, to_x): (i32, i32) = xs.split("..").map(|n| n.parse().unwrap()).collect_tuple().unwrap();
    let (from_y, to_y): (i32, i32) = ys.split("..").map(|n| n.parse().unwrap()).collect_tuple().unwrap();

    let mut distint_velocities = 0;

    for x_acceleration in -1000..5_000 {
        println!("x acc = {}", x_acceleration);
        for y_acceleration in -100..5_000 {
            if let Some(_) = try_jump(x_acceleration, y_acceleration, (from_x, to_x), (from_y, to_y)) {
                distint_velocities += 1;
            }
        }
    }

    println!("{}", distint_velocities);
    // too low: 152

    // 1919
}

fn try_jump(mut x_acc: i32, mut y_acc: i32, (from_x, to_x): (i32, i32), (from_y, to_y): (i32, i32)) -> Option<i32> {
    let mut max_y = 0;

    let (mut x, mut y) = (0, 0);
    loop {
        // println!("{:?}", (x, y));
        if y > max_y { max_y = y; }
        if x > to_x || y < from_y { return None; }
        if x >= from_x && x <= to_x && y >= from_y && y <= to_y { break; }

        x += x_acc;
        y += y_acc;

        if x_acc > 0 { x_acc -= 1; } else if x_acc < 0 { x_acc += 1; }
        y_acc -= 1;
    }

    Some(max_y)
}

fn part2() {}

fn main() {
    part1();
}