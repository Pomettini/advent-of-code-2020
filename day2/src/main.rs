#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::any::TypeId;

#[macro_export]
macro_rules! lazy_capture_name {
    ($capture:expr, $name:expr, $type:ty) => {
        $capture
            .name($name)
            .expect(&format!("Cannot find label with name: {}", $name)[..])
            .as_str()
            .parse::<$type>()
            .expect(&format!("Cannot cast label to type: {:?}", TypeId::of::<$type>())[..]);
    };
}

fn main() {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>[0-9]+)-(?P<max>[0-9]+)\s(?P<letter>[a-z]):\s(?P<password>.*)")
                .expect("Invalid regex");
    }

    let input = include_str!("input.txt");

    let passwords: Vec<&str> = input.lines().collect();

    let mut total_occurrences = 0;

    for password in &passwords {
        let capture = RE.captures(password).unwrap();

        let min = lazy_capture_name!(capture, "min", usize);
        let max = lazy_capture_name!(capture, "max", usize);
        let letter = lazy_capture_name!(capture, "letter", char);
        let password = lazy_capture_name!(capture, "password", String);

        let occurrences = password.matches(letter).count();

        if occurrences >= min && occurrences <= max {
            total_occurrences += 1;
        }
    }

    println!("The solution for the first part is {}", total_occurrences);

    total_occurrences = 0;

    for password in &passwords {
        let capture = RE.captures(password).expect(&format!("Invalid capture for {}", password));

        let min = lazy_capture_name!(capture, "min", usize);
        let max = lazy_capture_name!(capture, "max", usize);
        let letter = lazy_capture_name!(capture, "letter", char);
        let password: String = lazy_capture_name!(capture, "password", String);

        let first_letter = password.chars().nth(min - 1).expect("Index is out of bounds");
        let second_letter = password.chars().nth(max - 1).expect("Index is out of bounds");

        if (first_letter == letter) ^ (second_letter == letter)
        {
            total_occurrences += 1;
        }
    }

    println!("The solution for the second part is {}", total_occurrences);
}
