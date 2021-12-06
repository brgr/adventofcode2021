use std::collections::HashMap;
use itertools::Itertools;

const E1: &str = "3,4,3,1,2";
const INPUT: &str = "1,3,1,5,5,1,1,1,5,1,1,1,3,1,1,4,3,1,1,2,2,4,2,1,3,3,2,4,4,4,1,3,1,1,4,3,1,5,5,1,1,3,4,2,1,5,3,4,5,5,2,5,5,1,5,5,2,1,5,1,1,2,1,1,1,4,4,1,3,3,1,5,4,4,3,4,3,3,1,1,3,4,1,5,5,2,5,2,2,4,1,2,5,2,1,2,5,4,1,1,1,1,1,4,1,1,3,1,5,2,5,1,3,1,5,3,3,2,2,1,5,1,1,1,2,1,1,2,1,1,2,1,5,3,5,2,5,2,2,2,1,1,1,5,5,2,2,1,1,3,4,1,1,3,1,3,5,1,4,1,4,1,3,1,4,1,1,1,1,2,1,4,5,4,5,5,2,1,3,1,4,2,5,1,1,3,5,2,1,2,2,5,1,2,2,4,5,2,1,1,1,1,2,2,3,1,5,5,5,3,2,4,2,4,1,5,3,1,4,4,2,4,2,2,4,4,4,4,1,3,4,3,2,1,3,5,3,1,5,5,4,1,5,1,2,4,2,5,4,1,3,3,1,4,1,3,3,3,1,3,1,1,1,1,4,1,2,3,1,3,3,5,2,3,1,1,1,5,5,4,1,2,3,1,3,1,1,4,1,3,2,2,1,1,1,3,4,3,1,3";

type FishAmount = i64;
type LanternFish = HashMap<u8, FishAmount>;

trait FishCounter {
    fn get_amount_of_fish(&self, days_until_reset: &u8) -> &FishAmount;
    fn next_day(&mut self);
    fn init(&mut self, v: Vec<u8>);
}

impl FishCounter for LanternFish {
    fn get_amount_of_fish(&self, days_until_reset: &u8) -> &FishAmount {
        self.get(days_until_reset).unwrap_or(&0)
    }

    fn next_day(&mut self) {
        let reproducing_fish = self.get_amount_of_fish(&0).clone();
        self.remove(&0);

        for i in 1..=8 {
            self.remove(&i).map(|v| self.insert(i - 1, v));
        }

        self.insert(6, self.get_amount_of_fish(&6u8) + reproducing_fish);
        self.insert(8, reproducing_fish);
    }

    fn init(&mut self, v: Vec<u8>) {
        for x in v {
            self.insert(x, self.get_amount_of_fish(&x) + 1);
        }
    }
}

fn part1() {
    let values: Vec<u8> = INPUT.split(",").map(|n| n.parse().unwrap()).collect();
    let mut fishes = LanternFish::new();
    // Note: Another way would have been: (all in one)
    // let mut fishes = INPUT.split(",").map(|n| n.parse().unwrap()).counts();
    fishes.init(values);

    for i in 0..256 {
        fishes.next_day();
        println!("{:?}", fishes);
    }

    println!("{:?}", fishes.values().sum::<i64>());
}

fn main() {
    part1();
}