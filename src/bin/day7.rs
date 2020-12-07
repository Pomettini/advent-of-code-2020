#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\sbags?\.?)").unwrap();
}

fn main() {
    let input = include_str!("../../inputs/day7.txt");

    let bags: HashMap<&str, HashMap<String, usize>> =
        input.lines().fold(HashMap::new(), |mut map, line| {
            let split: Vec<&str> = line.split(" bags contain ").collect();
            let bag = RE
                .replace_all(split[1], "")
                .clone()
                .split(", ")
                .into_iter()
                .fold(HashMap::new(), |mut a, b| {
                    a.insert(
                        b.chars().skip(2).into_iter().collect(),
                        b.chars()
                            .next()
                            .unwrap()
                            .to_string()
                            .parse()
                            .unwrap_or_default(),
                    );
                    a
                });
            map.insert(split[0], bag);
            map
        });

    dbg!(bags);
}
