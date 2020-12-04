#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"byr|iyr|eyr|hgt|hcl|ecl|pid|cid").expect("Invalid regex");
}

fn main() {
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.lines().collect();

    let mut filtered_lines: Vec<String> = Vec::new();
    let mut temp = String::new();

    for line in lines {
        if !line.is_empty() {
            temp.push_str(line);
        } else {
            &mut filtered_lines.push(temp.clone());
            temp.clear();
        }
    }

    filtered_lines.push(temp);

    let matches = filtered_lines
        .into_iter()
        .filter(|line| {
            let count = RE.find_iter(line).count();
            count == 8 || (count == 7 && !line.contains("cid"))
        })
        .count();

    println!("The solution for the first problem is {}", matches);
}
