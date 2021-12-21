use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::thread::current;

use itertools;

const E1: &str = "";
const E2: &str = "";
const E3: &str = "";
const E4: &str = "";
const INPUT: &str = "";

fn part1() {
    let mut player1_position = 10;
    let mut player2_position = 7;

    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut dice = 1;
    let mut dice_rolls = 0;

    let mut player1_turn = true;
    while player1_score < 1000 && player2_score < 1000 {
        if player1_turn {
            let player_move = dice + (dice + 1) + (dice + 2);
            dice = (((dice + 3) - 1) % 100) + 1;

            player1_position = (((player1_position + player_move) - 1) % 10) + 1;
            player1_score += player1_position;
            println!("Player 1 rolls {}, moves to {}, total score of {}", player_move, player1_position, player1_score);
        } else {
            let player_move = dice + (dice + 1) + (dice + 2);
            dice = dice + 3;

            player2_position = (((player2_position + player_move) - 1) % 10) + 1;
            player2_score += player2_position;
            println!("Player 2 rolls {}, moves to {}, total score of {}", player_move, player2_position, player2_score);
        }

        dice_rolls += 3;
        player1_turn = !player1_turn;
    }

    println!("Total dice rolls: {}", dice_rolls);

    println!("Result: {}", dice_rolls * (if player1_score < player2_score { player1_score } else { player2_score }));
}

fn part2() {}

fn main() {
    part1();
}