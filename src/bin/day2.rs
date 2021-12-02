#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::any::TypeId;

#[macro_export]
macro_rules! lazy_capture {
    ($capture:expr, $name:expr, $type:ty) => {
        $capture
            .name($name)
            .expect(&format!("Cannot find label with name: {}", $name)[..])
            .as_str()
            .parse::<$type>()
            .expect(&format!("Cannot cast label to type: {:?}", TypeId::of::<$type>())[..])
    };
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<min>[0-9]+)-(?P<max>[0-9]+)\s(?P<letter>[a-z]):\s(?P<password>.*)")
            .expect("Invalid regex");
}

struct Password {
    min: usize,
    max: usize,
    letter: char,
    buffer: String,
}

impl Password {
    fn new(re: &Regex, password: &str) -> Self {
        let capture = re.captures(password).expect("Invalid regex");

        Self {
            min: lazy_capture!(capture, "min", usize),
            max: lazy_capture!(capture, "max", usize),
            letter: lazy_capture!(capture, "letter", char),
            buffer: lazy_capture!(capture, "password", String),
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/day2.txt");

    let lines: Vec<&str> = input.lines().collect();

    let mut total_occurrences = 0;

    for line in &lines {
        let password = Password::new(&RE, line);

        let occurrences = password.buffer.matches(password.letter).count();

        if occurrences >= password.min && occurrences <= password.max {
            total_occurrences += 1;
        }
    }

    println!("The solution for the first part is {}", total_occurrences);

    assert!(total_occurrences == 622);

    total_occurrences = 0;

    for line in &lines {
        let password = Password::new(&RE, line);

        let first_letter = password
            .buffer
            .chars()
            .nth(password.min - 1)
            .expect("Index is out of bounds");
        let second_letter = password
            .buffer
            .chars()
            .nth(password.max - 1)
            .expect("Index is out of bounds");

        if (first_letter == password.letter) ^ (second_letter == password.letter) {
            total_occurrences += 1;
        }
    }

    println!("The solution for the second part is {}", total_occurrences);

    assert!(total_occurrences == 263);
}
