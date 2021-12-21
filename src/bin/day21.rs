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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct GameState {
    p1score: u8,
    p2score: u8,
    p1position: u8,
    p2position: u8,
}

fn part2() {
    // TODO: I think the idea/concept is generally good and correct, but for some reason I'm getting
    //  results that are too small.
    //  One thing that is definitely still wrong and that I need to correct, is that I don't save
    //  the turn in the GameState, i.e whether at a game point it is p1's turn or p2's.
    //  I should start by implementing that, and then see if I'm getting better results maybe.
    let mut state_occurrences = HashMap::new();
    state_occurrences.insert(GameState {
        p1position: 4,
        p2position: 8,
        p1score: 0,
        p2score: 0,
    }, 1);

    // 444356092776315
    // 6036098815409

    let mut p1turn = true;
    loop {
        let mut next_occurrences = HashMap::new();

        let initial_len = state_occurrences.len();
        for so in state_occurrences.iter() {

            let so_occurrences = next_occurrences.entry(*so.0).or_insert(0);
            *so_occurrences += *so.1;

            // if p1turn {
                let gs = *so.0;
                if gs.p1score > 20 || gs.p2score > 20 {
                    continue;
                }

                for i in 3..=9 {
                    let new_position = (((&gs.p1position + i) - 1) % 10) + 1;
                    let new_score = &gs.p1score + new_position;

                    let next_gamestate = GameState { p1position: new_position, p1score: new_score, ..gs };
                    let gs_occurences = next_occurrences.entry(next_gamestate).or_insert(0);

                    let increase_by = match i {
                        3 => 1,
                        4 => 3,
                        5 => 6,
                        6 => 7,
                        7 => 6,
                        8 => 3,
                        9 => 1,
                        _ => panic!(),
                    };

                    *gs_occurences += (*so.1 * increase_by);
                }
            // } else { // todo: without this if-else, it is as if p2 starts as well as first
                let gs = *so.0;
                if gs.p1score > 20 || gs.p2score > 20 {
                    continue;
                }

                for i in 3..=9 {
                    let new_position = (((&gs.p2position + i) -1 ) % 10) + 1;
                    let new_score = &gs.p2score + new_position;

                    let next_gamestate = GameState { p2position: new_position, p2score: new_score, ..gs };
                    let gs_occurences = next_occurrences.entry(next_gamestate).or_insert(0);

                    let increase_by = match i {
                        3 => 1,
                        4 => 3,
                        5 => 6,
                        6 => 7,
                        7 => 6,
                        8 => 3,
                        9 => 1,
                        _ => panic!(),
                    };


                    *gs_occurences += (*so.1 * increase_by);
                }

            // }

        }

        if initial_len == next_occurrences.len() { break; }

        state_occurrences = next_occurrences;
        p1turn = !p1turn;

        println!("{}", state_occurrences.len());


    }

    // calculate all where p1 has score of >= 21

    let mut games_won_by_p1: u64 = 0;

    let cloned = state_occurrences.clone();
    for x in cloned {
        if x.0.p1score >= 21 {
            games_won_by_p1 += x.1;
        }
    }

    println!("won games: {}", games_won_by_p1);

}

fn main() {
    part2();
}