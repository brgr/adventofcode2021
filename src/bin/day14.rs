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

const E1: &str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";
const E2: &str = "";
const INPUT: &str = "CKFFSCFSCBCKBPBCSPKP\n\nNS -> P\nKV -> B\nFV -> S\nBB -> V\nCF -> O\nCK -> N\nBC -> B\nPV -> N\nKO -> C\nCO -> O\nHP -> P\nHO -> P\nOV -> O\nVO -> C\nSP -> P\nBV -> H\nCB -> F\nSF -> H\nON -> O\nKK -> V\nHC -> N\nFH -> P\nOO -> P\nVC -> F\nVP -> N\nFO -> F\nCP -> C\nSV -> S\nPF -> O\nOF -> H\nBN -> V\nSC -> V\nSB -> O\nNC -> P\nCN -> K\nBP -> O\nPC -> H\nPS -> C\nNB -> K\nVB -> P\nHS -> V\nBO -> K\nNV -> B\nPK -> K\nSN -> H\nOB -> C\nBK -> S\nKH -> P\nBS -> S\nHV -> O\nFN -> F\nFS -> N\nFP -> F\nPO -> B\nNP -> O\nFF -> H\nPN -> K\nHF -> H\nVK -> K\nNF -> K\nPP -> H\nPH -> B\nSK -> P\nHN -> B\nVS -> V\nVN -> N\nKB -> O\nKC -> O\nKP -> C\nOS -> O\nSO -> O\nVH -> C\nOK -> B\nHH -> B\nOC -> P\nCV -> N\nSH -> O\nHK -> N\nNO -> F\nVF -> S\nNN -> O\nFK -> V\nHB -> O\nSS -> O\nFB -> B\nKS -> O\nCC -> S\nKF -> V\nVV -> S\nOP -> H\nKN -> F\nCS -> H\nCH -> P\nBF -> F\nNH -> O\nNK -> C\nOH -> C\nBH -> O\nFC -> V\nPB -> B";

fn part1() {
    let (template, reactions) = INPUT.split_once("\n\n").unwrap();

    let reactions: HashMap<(char, char), char> = reactions.lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(from, to)|
            ((from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()),
             to.chars().nth(0).unwrap()))
        .collect();

    let mut current_polymer = template.chars().collect_vec();

    for i in 0..10 {
        println!("Step: {}", i);
        let mut new_polymer = Vec::new();
        new_polymer.push(*current_polymer.first().unwrap());

        for j in 1..current_polymer.len() {
            let after = current_polymer[j];
            let before = new_polymer.last().unwrap();
            let middle = get_middle(&reactions, before, &after);

            new_polymer.push(middle);
            new_polymer.push(after);
        }

        current_polymer = new_polymer;
    }

    println!("{:?}", current_polymer);


    let mut frequencies: HashMap<char, u32> = HashMap::new();
    for x in current_polymer {
        let count = frequencies.entry(x).or_insert(0);
        *count += 1;
    }

    println!("{:?}", frequencies);

    let max = frequencies.values().max().unwrap();
    let min = frequencies.values().min().unwrap();

    println!("{}", (max - min));
}

fn get_middle(reactions: &HashMap<(char, char), char>, before: &char, after: &char) -> char {
    *reactions.get(&(*before, *after)).unwrap()
}


fn part2() {}

fn main() {
    part1();
}