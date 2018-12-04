mod day1;
mod day2;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

fn solve_file<T, F: Fn(&str) -> T>(file_name: &str, f: F) -> T {
    let mut s = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut s).unwrap();
    f(&s)
}

fn main() {
    let matches = App::new("Advent Of Code 2018")
        .arg(
            Arg::with_name("2")
                .short("2")
                .multiple(false)
                .help("Solve part2"),
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

    let file_name = matches.value_of("file").unwrap();
    let part2 = matches.is_present("2");
    let day = matches.value_of("day").unwrap().parse::<usize>().expect("Not a number");
    match (day, part2) {
        (1, false) => println!("{}", solve_file(file_name, day1::solve1)),
        (1, true) => println!("{}", solve_file(file_name, day1::solve2)),
        (2, false) => println!("{}", solve_file(file_name, day2::solve1)),
        (2, true) => println!("{}", solve_file(file_name, day2::solve2)),
        _ => panic!("Unhandled")
    }
}

