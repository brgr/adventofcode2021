use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ptr::write;
use std::str::{Chars, FromStr};
use std::string::ParseError;
use tailcall::tailcall;

use itertools::Itertools;
use phf::phf_map;

const E1: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";

const E2: &str = "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
const INPUT: &str = "KF-sr\nOO-vy\nstart-FP\nFP-end\nvy-mi\nvy-KF\nvy-na\nstart-sr\nFP-lh\nsr-FP\nna-FP\nend-KF\nna-mi\nlh-KF\nend-lh\nna-start\nwp-KF\nmi-KF\nvy-sr\nvy-lh\nsr-mi";

#[derive(Debug)]
struct Cave<'a> {
    paths: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Cave<'a> {
    fn new(s: &'a str) -> Self {
        let mut cave = Cave { paths: HashMap::new() };

        for ss in s.lines() {
            let (start, end): (&'a str, &'a str) = ss.split("-").collect_tuple().unwrap();
            cave.add_path(start, end);

            if start != "start" && end != "end" {
                cave.add_path(end, start);
            }
        }

        cave
    }

    fn add_path(&mut self, start: &'a str, end: &'a str) {
        let path = self.paths.entry(start).or_insert(Vec::new());
        path.push(end);
    }

}

#[tailcall]
fn find_all_paths<'a>(cave: &'a Cave, finished_paths: &'a mut Vec<FullPath<'a>>, started_paths: &mut Vec<FullPath<'a>>) -> &'a Vec<FullPath<'a>> {
    if started_paths.is_empty() {
        return finished_paths;
    }

    let first_full_path = started_paths.remove(0);
    let last_point = first_full_path.0.last().unwrap();
    let reachable = cave.paths.get(last_point).unwrap_or(&Vec::new()).clone();

    for reachable_point in reachable {
        if reachable_point == "end" {
            let mut end_path = first_full_path.clone();
            end_path.0.push(reachable_point);
            finished_paths.push(end_path);
        } else {
            if reachable_point.to_ascii_uppercase().as_str() == reachable_point {
                let mut new_path = first_full_path.clone();
                new_path.0.push(reachable_point);
                started_paths.push(new_path);
            } else { // is lowercase
                if (!first_full_path.0.contains(&reachable_point)
                    || !first_full_path.1)
                    && reachable_point != "start"
                {
                    let mut new_path = first_full_path.clone();
                    if first_full_path.0.contains(&reachable_point) {
                        new_path.1 = true;
                    }
                    new_path.0.push(reachable_point);
                    started_paths.push(new_path);
                }
            }
        }
    }

    find_all_paths(cave, finished_paths, started_paths)
}


// type FullPath<'a> = Vec<&'a str>;

#[derive(Clone, Debug)]
struct FullPath<'a>(Vec<&'a str>, bool);

fn part1() {
    let cave = Cave::new(INPUT);
    println!("{:?}", cave);

    let mut from_start = cave.paths.get("start").unwrap().clone();
    let mut finished_paths = Vec::new();
    let mut started_paths = from_start.into_iter()
        .map(|p| FullPath(vec!["start", p], false))
        .collect();

    let vec1 = find_all_paths(&cave, &mut finished_paths, &mut started_paths);

    println!("{:?}", vec1);
    println!("{}", vec1.len());
}

fn part2() {}

// too high:
// 119716

fn main() {
    part1();
}