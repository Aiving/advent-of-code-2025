#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!(">>> Day 2");

    day2::solution();

    println!(">>> Day 3");

    day3::solution();

    println!(">>> Day 4");

    day4::solution();

    println!(">>> Day 5");

    day5::solution();
}
