#![feature(uniform_paths)]
#[macro_use]
extern crate nom;

#[macro_export]
macro_rules! err {
    () => (
        concat!(file!(), ":", line!(), ",", column!())
    );
    ($msg:expr) => (
        concat!($msg, ", at ", file!(), ":", line!(), ",", column!())
    )
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
