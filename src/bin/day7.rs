#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\sbags?\.?)").unwrap();
}

fn main() {
    let input = include_str!("../../inputs/day7.txt");

    let bags: HashMap<String, HashMap<String, usize>> =
        input.lines().fold(HashMap::new(), |mut map, line| {
            let split: Vec<&str> = line.split(" bags contain ").collect();
            let bag = RE
                .replace_all(split[1], "")
                .clone()
                .split(", ")
                .into_iter()
                .filter(|x| !x.contains("no other"))
                .fold(HashMap::new(), |mut map, x| {
                    map.insert(
                        x.chars().skip(2).into_iter().collect(),
                        x.chars()
                            .next()
                            .unwrap()
                            .to_string()
                            .parse()
                            .unwrap_or_default(),
                    );
                    map
                });
            map.insert(split[0].to_owned(), bag);
            map
        });

    // The solution below has been *widely inspired* by Chevy Ray's code because I am dumb

    fn can_hold(
        bags: &HashMap<String, HashMap<String, usize>>,
        bag: &HashMap<String, usize>,
    ) -> bool {
        bag.contains_key("shiny gold") || bag.keys().any(|b| can_hold(bags, bags.get(&*b).unwrap()))
    }

    let result: usize = bags.values().filter(|bag| can_hold(&bags, bag)).count();

    println!("The solution for the first problem is {}", result);

    assert!(result == 112);

    fn count_inner(
        bags: &HashMap<String, HashMap<String, usize>>,
        outer: &HashMap<String, usize>,
    ) -> usize {
        outer
            .iter()
            .map(|(inner, n)| n + n * count_inner(bags, bags.get(inner).unwrap()))
            .sum::<usize>()
    }

    let result: usize = count_inner(&bags, bags.get("shiny gold").unwrap());

    println!("The solution for the second problem is {}", result);

    assert!(result == 6260);
}
