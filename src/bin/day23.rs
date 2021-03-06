use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::ptr::write;
use std::str::FromStr;
use std::thread::current;

use itertools;
use itertools::Itertools;
use parse_display::Display;

use crate::Amphipod::{A, B, C, D};

#[derive(Debug, Copy, Clone, Display, Eq, PartialEq, Hash)]
enum Amphipod { A, B, C, D }

impl Amphipod {
    fn get_energy(&self, energy: u32) -> u32 {
        match self {
            A => energy,
            B => energy * 10,
            C => energy * 100,
            D => energy * 1000
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Burrow {
    hallway: [Option<Amphipod>; 11],
    sideway_a: [Option<Amphipod>; 4],
    sideway_b: [Option<Amphipod>; 4],
    sideway_c: [Option<Amphipod>; 4],
    sideway_d: [Option<Amphipod>; 4],
}

/// Represents a burrow.
/// As for the sideways, `sideway[0]` is always the upper/outer position, and `sideway[1]` is
/// always lower/inner position. With part 2 now, `sideway[2]` and `sideway[3]` will be further
/// into the sideway/cave, i.e. `sideway[3]` is the last and lowest element in the cave.
impl Burrow {
    fn init() -> Self {
        let hallway = [None, None, None, None, None, None, None, None, None, None, None];
        // My input:
        let sideway_a = [Some(C), Some(D), Some(D),  Some(C)];
        let sideway_b = [Some(A), Some(C), Some(B),  Some(A)];
        let sideway_c = [Some(B), Some(B), Some(A),  Some(D)];
        let sideway_d = [Some(D), Some(A), Some(C),  Some(B)];

        // Example input:
        // let sideway_a = [Some(B), Some(D), Some(D), Some(A)];
        // let sideway_b = [Some(C), Some(C), Some(B), Some(D)];
        // let sideway_c = [Some(B), Some(B), Some(A), Some(C)];
        // let sideway_d = [Some(D), Some(A), Some(C), Some(A)];
        Burrow { hallway, sideway_a, sideway_b, sideway_c, sideway_d }
    }

    /// Moves an amphipod from the hallway into its destined sideway.
    /// Returns an error if:
    ///  - Some other amphipod is in the way
    ///  - The sideway is not yet ready for insert - a sideway is ready for insert if it's empty
    ///    or it only contains the other same amphipod
    fn move_into_sideway(&mut self, position: usize) -> Result<u32, Error> {
        let current_amphipod = self.hallway[position]
            .unwrap_or_else(|| panic!());
        self.hallway[position] = None;

        let sideway_position = Burrow::get_sideway_position(&current_amphipod);
        if self.is_no_one_in_the_way(position, sideway_position) {
            if self.is_sideway_ready_for_insert(&current_amphipod) {
                let sideway_energy = self.insert_amphipod_in_sideway(&current_amphipod, current_amphipod);
                let total_energy = sideway_energy + (sideway_position as i8 - position as i8).abs() as u32;
                Ok(current_amphipod.get_energy(total_energy))
            } else {
                Err(Error)
            }
        } else {
            Err(Error)
        }
    }

    /// Checks if the sideway for the given amphipod is ready for the given amphipod to be inserted.
    /// This is the case if either the sideway is empty, or it contains only the other same
    /// amphipod.
    fn is_sideway_ready_for_insert(&self, amphipod: &Amphipod) -> bool {
        let sideway = self.get_sideway(amphipod);

        // sideway[0].is_none() && (sideway[1].is_none() || sideway[1].unwrap() == *amphipod)

        for i in 0..4 {
            if sideway[i].is_some() && sideway[i].unwrap() != *amphipod {
                return false;
            }
        }

        true
    }

    /// Moves an amphipod out of a sideway and into the given position.
    /// Returns an error if:
    ///  - the position is a sideway position or greater than 10
    ///  - the sideway was empty, i.e. it contained no amphipod
    ///  - there was another amphipod in the way in the hallway
    /// If no error occurred, the used energy is returned.
    fn move_away_from_sideway(&mut self, sideway: &Amphipod, position: usize) -> Result<u32, Error> {
        if Burrow::is_sideways_position(position) || position > 10 { return Err(Error); }

        let removed_amphipod: Option<(Amphipod, u32)> = self.remove_next_up_in_sideway(sideway);

        if let Some((next_up, sideway_energy)) = removed_amphipod {
            let sideway_position: usize = Burrow::get_sideway_position(sideway);

            if self.is_no_one_in_the_way(sideway_position, position) {
                self.hallway[position] = Some(next_up);
                let total_energy = sideway_energy + (sideway_position as i8 - position as i8).abs() as u32;
                Ok(next_up.get_energy(total_energy))
            } else {
                self.insert_amphipod_in_sideway(sideway, next_up);
                Err(Error)
            }
        } else {
            Err(Error)
        }
    }

    /// Inserts the given amphipod into the given sideway.
    /// This panics if one of the following happens:
    ///  - Both entries in the sideway are already full
    ///  - The first entry (`sideway[0]`) is already full, but the second isn't - which is actually
    ///    an invalid state!
    fn insert_amphipod_in_sideway(&mut self, sideway: &Amphipod, amphipod: Amphipod) -> u32 {
        let sideway = self.get_sideway_mut(sideway);

        for i in 0..4 {
            if sideway[3 - i].is_none() {
                sideway[3 -i] = Some(amphipod);
                return 4 - i as u32;
            }
        }

        panic!()

        // if sideway[1].is_none() && sideway[0].is_none() {
        //     sideway[1] = Some(amphipod);
        //     2
        // } else if sideway[0].is_none() {
        //     sideway[0] = Some(amphipod);
        //     1
        // } else {
        //     panic!();
        // }
    }

    /// Checks if between start and goal no amphipod is in the way and returns true if that
    /// is the case.
    fn is_no_one_in_the_way(&self, start: usize, goal: usize) -> bool {
        if start == goal { return true; }

        let from = if start < goal { start + 1 } else { goal + 1 };
        let to = if start < goal { goal - 1 } else { start - 1 };

        for i in from..=to {
            if self.hallway[i].is_some() { return false; }
        }

        true
    }

    /// Removes the first amphipod in the sideway. If there is none in the sideway, *None*
    /// is returned.
    /// If instead one was removed, it is returned together with the energy used to move it out
    /// of the sideway, i.e. 1 if it was the upper amphipod, and 2 if it was the lower one.
    fn remove_next_up_in_sideway(&mut self, sideway: &Amphipod) -> Option<(Amphipod, u32)> {
        let sideway: &mut [Option<Amphipod>; 4] = self.get_sideway_mut(sideway);

        for i in 0..4 {
            if let Some(amphipod) = sideway[i] {
                sideway[i] = None;
                return Some((amphipod, i as u32 + 1));
            }
        }

        None
    }

    fn get_sideway_mut(&mut self, sideway: &Amphipod) -> &mut [Option<Amphipod>; 4] {
        match sideway {
            A => &mut self.sideway_a,
            B => &mut self.sideway_b,
            C => &mut self.sideway_c,
            D => &mut self.sideway_d
        }
    }

    fn get_sideway(&self, sideway: &Amphipod) -> &[Option<Amphipod>; 4] {
        match sideway {
            A => &self.sideway_a,
            B => &self.sideway_b,
            C => &self.sideway_c,
            D => &self.sideway_d
        }
    }

    fn get_sideway_position(sideway: &Amphipod) -> usize {
        match sideway {
            A => 2,
            B => 4,
            C => 6,
            D => 8
        }
    }

    fn is_sideways_position(position: usize) -> bool {
        position == 2 || position == 4 || position == 6 || position == 8
    }

    fn get_sideway_entry_as_str(&self, sideway: &Amphipod, pos: usize) -> String {
        let s = self.get_sideway(sideway);
        if let Some(a) = s[pos] {
            a.to_string()
        } else {
            ".".to_string()
        }
    }

    fn is_done(&self) -> bool {
        self.get_sideway(&A) == &[Some(A), Some(A), Some(A), Some(A)] &&
            self.get_sideway(&B) == &[Some(B), Some(B), Some(B), Some(B)] &&
            self.get_sideway(&C) == &[Some(C), Some(C), Some(C), Some(C)] &&
            self.get_sideway(&D) == &[Some(D), Some(D), Some(D), Some(D)]
    }

    fn find_all_next(&self, current_score: u32) -> Vec<(Burrow, u32)> {
        let mut all_next = Vec::new();

        // first all from the hallway
        // (note that hallway first or sideways first shouldn't matter)

        for i in 0..self.hallway.len() {
            if self.hallway[i].is_some() {
                let mut new_burrow = self.clone();
                let result = new_burrow.move_into_sideway(i);
                if let Ok(score) = result {
                    all_next.push((new_burrow, current_score + score));
                }
            }
        }

        // then all from the sideways

        for sideway in vec![A, B, C, D] {
            if self.sideway_contains_wrong_amphipod(&sideway) {
                // move it out into all possible positions (all that don't give an error are possible)
                for position in 0..=10 {
                    let mut new_burrow = self.clone();
                    let result = new_burrow.move_away_from_sideway(&sideway, position);
                    if let Ok(score) = result {
                        all_next.push((new_burrow, current_score + score));
                    }
                }
            }
        }

        all_next
    }

    fn sideway_contains_wrong_amphipod(&self, amphipod: &Amphipod) -> bool {
        let sideway = self.get_sideway(amphipod);

        for i in 0..4 {
            if sideway[i].is_some() {
                if sideway[i].unwrap() != *amphipod {
                    return true;
                }
            }
        }

        false

        // (sideway[0].is_some() && sideway[0].unwrap() != *amphipod) ||
        //     (sideway[1].is_some() && sideway[1].unwrap() != *amphipod)
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..13 { write!(f, "#")?; }
        write!(f, "\n#")?;

        for i in 0..self.hallway.len() {
            if let Some(a) = self.hallway[i] {
                write!(f, "{:?}", a)?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "#\n")?;

        // sideways
        write!(f, "###{}#{}#{}#{}###\n", self.get_sideway_entry_as_str(&A, 0),
            self.get_sideway_entry_as_str(&B, 0), self.get_sideway_entry_as_str(&C, 0),
            self.get_sideway_entry_as_str(&D, 0))?;
        write!(f, "  #{}#{}#{}#{}#  \n", self.get_sideway_entry_as_str(&A, 1),
            self.get_sideway_entry_as_str(&B, 1), self.get_sideway_entry_as_str(&C, 1),
            self.get_sideway_entry_as_str(&D, 1))?;
        write!(f, "  #{}#{}#{}#{}#  \n", self.get_sideway_entry_as_str(&A, 2),
               self.get_sideway_entry_as_str(&B, 2), self.get_sideway_entry_as_str(&C, 2),
               self.get_sideway_entry_as_str(&D, 2))?;
        write!(f, "  #{}#{}#{}#{}#  \n", self.get_sideway_entry_as_str(&A, 3),
               self.get_sideway_entry_as_str(&B, 3), self.get_sideway_entry_as_str(&C, 3),
               self.get_sideway_entry_as_str(&D, 3))?;

        write!(f, "  #########  \n")?;



        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let mut burrow = Burrow::init();
        let amphipod = burrow.remove_next_up_in_sideway(&A);
        println!("{}\n{:?}", burrow, amphipod);
    }

    #[test]
    fn test_move_out_of_sideway() {
        let mut burrow = Burrow::init();
        println!("{}", burrow);

        let energy = burrow.move_away_from_sideway(&A, 10);
        println!("{}", burrow);
        assert_eq!(Ok(90), energy);

        burrow.move_away_from_sideway(&A, 0);
        println!("{}", burrow);

        // move out of empty sideway gives an error
        let result = burrow.move_away_from_sideway(&A, 1);
        assert_eq!(Err(Error), result);
        println!("{}", burrow);

        // move in front of sideway gives an error
        let result1 = burrow.move_away_from_sideway(&C, 6);
        assert_eq!(Err(Error), result1);
        println!("{}", burrow);

        // move from C sideway to position 3, like in the AOC example
        let result2 = burrow.move_away_from_sideway(&C, 3);
        println!("{}", burrow);
        assert_eq!(Ok(40), result2);

        let result3 = burrow.move_away_from_sideway(&C, 9);
        println!("{}", burrow);
        assert_eq!(Ok(500), result3);

        // let's check that nothing is moved if an amphipod is in the way
        let result4 = burrow.move_away_from_sideway(&D, 1);
        println!("{}", burrow);
        assert_eq!(Err(Error), result4);
    }

    // #[test]
    // fn test_example_part1_manually() {
    //     // Test the given example manually
    //     let mut burrow = Burrow::init();
    //     println!("{}", burrow);
    //
    //     let mut total_energy = 0;
    //
    //     total_energy += burrow.move_away_from_sideway(&C, 3).unwrap();
    //     println!("{}", burrow);
    //
    //     total_energy += burrow.move_away_from_sideway(&B, 5).unwrap();
    //     total_energy += burrow.move_into_sideway(5).unwrap();
    //     println!("{}", burrow);
    //
    //     total_energy += burrow.move_away_from_sideway(&B, 5).unwrap();
    //     total_energy += burrow.move_into_sideway(3).unwrap();
    //     println!("{}", burrow);
    //
    //     total_energy += burrow.move_away_from_sideway(&A, 3).unwrap();
    //     total_energy += burrow.move_into_sideway(3).unwrap();
    //     println!("{}", burrow);
    //
    //     total_energy += burrow.move_away_from_sideway(&D, 7).unwrap();
    //     total_energy += burrow.move_away_from_sideway(&D, 9).unwrap();
    //     println!("{}", burrow);
    //
    //     total_energy += burrow.move_into_sideway(5).unwrap();
    //     total_energy += burrow.move_into_sideway(7).unwrap();
    //     println!("{}", burrow);
    //
    //     total_energy += burrow.move_into_sideway(9).unwrap();
    //     println!("{}", burrow);
    //
    //     println!("{}", total_energy);
    // }

    #[test]
    fn test_example_part2_manually() {
        let mut burrow = Burrow::init();
        let mut score = 0;

        println!("{}", burrow);

        score += burrow.move_away_from_sideway(&D, 10).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&D, 0).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&C, 9).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&C, 7).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&C, 1).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&B, 5).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(5).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&B, 5).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(5).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&B, 5).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&B, 3).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(5).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(7).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(9).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&D, 7).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(7).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&D, 9).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(3).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&A, 3).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(3).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&A, 7).unwrap();
        println!("{}", burrow);
        score += burrow.move_away_from_sideway(&A, 5).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(7).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(5).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(1).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(0).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(9).unwrap();
        println!("{}", burrow);
        score += burrow.move_into_sideway(10).unwrap();
        println!("{}", burrow);

        println!("{}", score);
        println!("Is done: {}", burrow.is_done());



    }

    // #[test]
    // fn test_binary_heap_with_game_state_with_score() {
    //     let mut heap = BinaryHeap::new();
    //
    //     let burrow = Burrow::init();
    //
    //     heap.push(GameStateWithScore::new(burrow, 3000));
    //     heap.push(GameStateWithScore::new(burrow, 2000));
    //     heap.push(GameStateWithScore::new(burrow, 2000));
    //     heap.push(GameStateWithScore::new(burrow, 10));
    //
    //     let option = heap.pop();
    //     println!("{:?}", option);
    // }
}

#[derive(Debug, Clone)]
struct GameStateWithScore {
    burrow: Burrow,
    score: u32
}

impl <'a> GameStateWithScore {
    fn new(burrow: Burrow, score: u32) -> Self {
        GameStateWithScore {
            burrow, score
        }
    }
}

impl Eq for GameStateWithScore {}

impl PartialEq<Self> for GameStateWithScore {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl PartialOrd<Self> for GameStateWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // self.score.partial_cmp(&other.score)
        other.score.partial_cmp(&self.score) // we use it like this for min-heap
    }
}

impl Ord for GameStateWithScore {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.score.cmp(&other.score)
        other.score.cmp(&self.score) // we use it like this for min-heap
    }
}


fn part1() {
    // let mut burrows_yet_to_try = vec![(Burrow::init(), 0)];
    let mut burrows_yet_to_try = BinaryHeap::new();
    burrows_yet_to_try.push(GameStateWithScore::new(Burrow::init(), 0));
    let mut burrows_already_tried = HashMap::new();

    let mut burrow_at_min_score = None;
    let mut min_score_found_so_far = None;

    while !burrows_yet_to_try.is_empty() {
        println!("{}", burrows_yet_to_try.len());

        let gsws = burrows_yet_to_try.pop().unwrap();
        let this_burrow = gsws.burrow;
        let score = gsws.score;

        if min_score_found_so_far.map(|min_score| min_score <= score).unwrap_or(false) {
            continue;
        }

        if burrows_already_tried.contains_key(&this_burrow) &&
            burrows_already_tried.get(&this_burrow).unwrap() <= &score {
            continue;
        }

        if this_burrow.is_done() {
            if min_score_found_so_far == None || score < min_score_found_so_far.unwrap() {
                burrow_at_min_score = Some(gsws.burrow.clone());
                min_score_found_so_far = Some(score);
            }
            continue;
        }

        burrows_yet_to_try.extend(this_burrow.find_all_next(score).iter()
            .map(|bs| GameStateWithScore::new(bs.0, bs.1)));

        burrows_already_tried.insert(this_burrow, score);
    }

    println!("Min Score: {:?}", min_score_found_so_far);
    // println!("{}", burrow_at_min_score.unwrap());

    // for x in burrow_at_min_score.unwrap() {
    //     println!("burrow:\n{}\n", x);
    // }
    println!("{}", burrow_at_min_score.unwrap());
}


fn main() {
    // TODO: I got a SIGKILL during execution for the actual input!?
    //  I should try to remove keeping *all* burrows up to the last, and just save the last again
    part1();
}