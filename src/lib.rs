#![feature(uniform_paths)]

#[macro_export]
macro_rules! err {
    () => {
        concat!(file!(), ":", line!(), ",", column!())
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
