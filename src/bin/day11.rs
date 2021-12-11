use std::collections::{HashMap, HashSet};
use std::str::{Chars, FromStr};
use std::string::ParseError;

use itertools::Itertools;
use phf::phf_map;

const E1: &str = "11111\n19991\n19191\n19991\n11111";
const E2: &str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
const INPUT: &str = "2138862165\n2726378448\n3235172758\n6281242643\n4256223158\n1112268142\n1162836182\n1543525861\n1882656326\n8844263151";


fn increase_all(maze: &mut Vec<Vec<u8>>) {
    for row in maze {
        for o in row {
            *o += 1;
        }
    }
}

fn print_maze(maze: &Vec<Vec<u8>>) {
    for row in maze {
        for o in row {
            print!("{}", o);
        }
        println!();
    }
    println!();
}

fn flash(maze: &mut Vec<Vec<u8>>) -> u32 {
    let mut count_flash = 0;

    // for y in 0..maze.len() {
    //     for x in 0..maze[y].len() {
    //         if maze[y][x] == 10 {
    //             let mut neighbors = neighbors(maze, x, y);
    //             maze[y][x] += 1;
    //             count_flash += flash_from(maze, (x, y), &mut neighbors, count_flash + 1);
    //         }
    //     }
    // }

    let mut will_flash = all_that_will_flash(maze);
    if !will_flash.is_empty() {
        let first = will_flash.remove(0);
        count_flash = flash_from(maze, first, &mut will_flash, count_flash);
    }

    count_flash
}

fn all_that_will_flash(maze: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut will_flash = Vec::new();
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if maze[y][x] == 10 {
                will_flash.push((x, y));
            }
        }
    }

    will_flash
}

fn flash_from(maze: &mut Vec<Vec<u8>>, (x, y): (usize, usize), yet_to_look_at: &mut Vec<(usize, usize)>, count_flashes: u32) -> u32 {
    return if maze[y][x] == 11 {
        if yet_to_look_at.is_empty() {
            count_flashes
        } else {
            let first = yet_to_look_at.remove(0);
            flash_from(maze, first, yet_to_look_at, count_flashes)
        }
    } else if maze[y][x] == 10 {
        maze[y][x] += 1;
        let neighbors = neighbors(maze, x, y);
        yet_to_look_at.extend(neighbors.iter());
        let first = yet_to_look_at.remove(0);
        flash_from(maze, first, yet_to_look_at, count_flashes + 1)
    } else { // maze[y][x] <= 9
        maze[y][x] += 1;
        if maze[y][x] == 10 {
            let vec = neighbors(maze, x, y);
            yet_to_look_at.extend(vec);
            maze[y][x] += 1;
            let first = yet_to_look_at.remove(0);
            flash_from(maze, first, yet_to_look_at, count_flashes + 1)
        } else if yet_to_look_at.is_empty() {
            count_flashes
        } else {
            let first = yet_to_look_at.remove(0);
            flash_from(maze, first, yet_to_look_at, count_flashes)
        }
    };
}

fn neighbors(maze: &mut Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let from_x = if x > 0 { x - 1 } else { x };
    let to_x = if x < maze[0].len() - 1 { x + 1 } else { maze[0].len() - 1 };
    let from_y = if y > 0 { y - 1 } else { y };
    let to_y = if y < maze.len() - 1 { y + 1 } else { maze.len() - 1 };

    for xi in from_x..=to_x {
        for yi in from_y..=to_y {
            if (xi, yi) != (x, y) {
                neighbors.push((xi, yi));
            }
        }
    }

    neighbors
}

fn reset11(maze: &mut Vec<Vec<u8>>) {
    for row in maze {
        for o in row {
            if *o == 11 {
                *o = 0;
            }
        }
    }
}


fn part1() {
    let mut maze: Vec<Vec<u8>> = INPUT.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect())
        .collect();

    print_maze(&maze);

    let mut total_count_flashes = 0;
    for i in 0..100 {
        println!("Step {}:", i + 1);
        total_count_flashes += step(&mut maze);
    }

    println!("Total Count: {}", total_count_flashes);
}

fn step(mut maze: &mut Vec<Vec<u8>>) -> u32 {
    increase_all(&mut maze);
    let count_flashes = flash(&mut maze);
    reset11(&mut maze);


    print_maze(&maze);
    println!("--> Count Flashes: {}", count_flashes);
    count_flashes
}


fn part2() {
    let mut maze: Vec<Vec<u8>> = INPUT.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect())
        .collect();

    let mut nth_step = 0;
    loop {
        nth_step += 1;
        step(&mut maze);

        if is_all_zero(&maze) { break; }
    }

    println!("At step: {}", nth_step);
}

fn is_all_zero(maze: &Vec<Vec<u8>>) -> bool {
    for row in maze {
        for x in row {
            if *x != 0 { return false; }
        }
    }

    return true;
}

fn main() {
    part2();
}