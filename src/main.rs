use clap::{App, Arg};
use std::fs::File;
use std::fmt::Display;
use std::io::prelude::*;

use aoc_2018::*;

fn solve_and_print<T: Display, F: Fn(&str) -> T>(file_name: &str, f: F) {
    let mut s = String::new();
    let mut file = File::open(file_name).expect(err!("could not open input file"));
    file.read_to_string(&mut s).expect(err!("could not read input file"));
    println!("{}", f(&s))
}

fn main() {
    let matches = App::new("Advent Of Code 2018")
        .arg(
            Arg::with_name("2")
                .short("2")
                .multiple(false)
                .help("Solve part2 instead of part1"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .multiple(false)
                .required(true)
                .takes_value(true)
                .help("file name"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .multiple(false)
                .required(true)
                .takes_value(true)
                .help("which day"),
        )
        .get_matches();

    let file_name = matches.value_of("file").expect(err!());
    let part2 = matches.is_present("2");
    let day = matches
        .value_of("day")
        .expect(err!())
        .parse::<usize>()
        .expect("Not a number");
    match (day, part2) {
        (1, false) => solve_and_print(file_name, day1::solve1),
        (1, true) => solve_and_print(file_name, day1::solve2),
        (2, false) => solve_and_print(file_name, day2::solve1),
        (2, true) => solve_and_print(file_name, day2::solve2),
        (3, false) => solve_and_print(file_name, day3::solve1),
        (3, true) => solve_and_print(file_name, day3::solve2),
        _ => panic!("Unhandled"),
    }
}
