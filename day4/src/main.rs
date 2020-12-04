#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref ALL: Regex = Regex::new(r"byr|iyr|eyr|hgt|hcl|ecl|pid|cid").unwrap();
    static ref BYR: Regex = Regex::new(r"byr:(19[2-9][0-9]|200[0-2])\b").unwrap();
    static ref IYR: Regex = Regex::new(r"iyr:(201[0-9]|2020)\b").unwrap();
    static ref EYR: Regex = Regex::new(r"eyr:(202[0-9]|2030)\b").unwrap();
    static ref HGT: Regex =
        Regex::new(r"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\b").unwrap();
    static ref HCL: Regex = Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    static ref ECL: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    static ref PID: Regex = Regex::new(r"pid:[0-9]{9}\b").unwrap();
}

fn main() {
    let input = include_str!("input.txt");

    let lines: Vec<String> = input
        .split("\n\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let matches = lines
        .clone()
        .into_iter()
        .filter(|line| {
            let count = ALL.find_iter(line).count();
            count == 8 || (count == 7 && !line.contains("cid"))
        })
        .count();

    println!("The solution for the first problem is {}", matches);

    let matches = lines
        .into_iter()
        .filter(|line| {
            BYR.is_match(line)
                && IYR.is_match(line)
                && EYR.is_match(line)
                && HGT.is_match(line)
                && HCL.is_match(line)
                && ECL.is_match(line)
                && PID.is_match(line)
        })
        .count();

    println!("The solution for the second problem is {}", matches);
}
