use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::ptr::write;
use std::str::FromStr;
use std::thread::current;

use itertools;
use parse_display::Display;

use crate::Amphipod::{A, B, C, D};

const E1: &str = "";
const E2: &str = "";
const E3: &str = "";
const E4: &str = "";
const INPUT: &str = "";

#[derive(Debug, Copy, Clone, Display)]
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

struct Burrow {
    hallway: [Option<Amphipod>; 11],
    sideway_a: [Option<Amphipod>; 2],
    sideway_b: [Option<Amphipod>; 2],
    sideway_c: [Option<Amphipod>; 2],
    sideway_d: [Option<Amphipod>; 2],
}

/// Represents a burrow.
/// As for the sideways, `sideway[0]` is always the upper/outer position, and `sideway[1]` is
/// always lower/inner position.
impl Burrow {
    fn init() -> Self {
        let hallway = [None, None, None, None, None, None, None, None, None, None, None];
        // initialize here according to input!
        let sideway_a = [Some(B), Some(A)];
        let sideway_b = [Some(C), Some(D)];
        let sideway_c = [Some(B), Some(C)];
        let sideway_d = [Some(D), Some(A)];
        Burrow { hallway, sideway_a, sideway_b, sideway_c, sideway_d }
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

    fn insert_amphipod_in_sideway(&mut self, sideway: &Amphipod, amphipod: Amphipod) {
        let sideway = self.get_sideway_mut(sideway);

        if sideway[0].is_some() && sideway[1].is_some() { panic!(); }

        if sideway[1].is_none() {
            sideway[1] = Some(amphipod);
        } else {
            sideway[0] = Some(amphipod);
        }
    }

    /// Checks if between start and goal no amphipod is in the way and returns true if that
    /// is the case.
    fn is_no_one_in_the_way(&self, start: usize, goal: usize) -> bool {
        if start == goal { return true; }

        let from = if start < goal { start + 1 } else { goal + 1 };
        let to = if start < goal { goal - 1 } else { start - 1 };

        for i in from..to {
            if self.hallway[i].is_some() { return false; }
        }

        true
    }

    /// Removes the first amphipod in the sideway. If there is none in the sideway, *None*
    /// is returned.
    /// If instead one was removed, it is returned together with the energy used to move it out
    /// of the sideway, i.e. 1 if it was the upper amphipod, and 2 if it was the lower one.
    fn remove_next_up_in_sideway(&mut self, sideway: &Amphipod) -> Option<(Amphipod, u32)> {
        let sideway: &mut [Option<Amphipod>; 2] = self.get_sideway_mut(sideway);

        if let Some(amphipod) = sideway[0] {
            sideway[0] = None;
            Some((amphipod, 1))
        } else if let Some(amphipod) = sideway[1] {
            sideway[1] = None;
            Some((amphipod, 2))
        } else {
            None
        }
    }

    fn get_sideway_mut(&mut self, sideway: &Amphipod) -> &mut [Option<Amphipod>; 2] {
        match sideway {
            A => &mut self.sideway_a,
            B => &mut self.sideway_b,
            C => &mut self.sideway_c,
            D => &mut self.sideway_d
        }
    }

    fn get_sideway(&self, sideway: &Amphipod) -> &[Option<Amphipod>; 2] {
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
}

fn part1() {}


fn part2() {}

fn main() {
    part1();
}