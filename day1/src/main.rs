use color_eyre::{self, owo_colors::OwoColorize};
use std::time::Instant;

// my original solution but it happens to be a better one
fn _main2() -> color_eyre::Result<()> {
    // just for testing execution time
    let start = Instant::now();

    // start of program
    let sum: u32 = include_str!("input.txt")
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|text_input| -> u32 {
            // lets iterate on the str as bytes
            // I could collect the number into a vector
            let mut numbers: Vec<u32> = vec![];

            for c in text_input.chars() {
                match c.is_numeric() {
                    true => numbers.push(c.to_digit(10).unwrap()),
                    false => {}
                }
            }
            format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum();
    println!("sum is: {}", sum);

    // end of program
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    // just for testing execution time
    let start = Instant::now();

    // start of program
    //
    let sum: u32 = include_str!("input.txt")
        .lines()
        .filter_map(|line| {
            let mut nums = line.chars().filter_map(|ch| ch.to_digit(10));
            let first = nums.next();
            let last = nums.next_back().or(first);
            Some((first.unwrap_or(0) * 10) + last.unwrap_or(0))
        })
        .sum();
    println!("sum is: {}", sum);
    // end of program
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);

    Ok(())
}
