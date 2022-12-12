use itertools::Itertools;
use std::fs::File;
use std::str;
use std::io::{prelude::*, BufReader};

use crate::until_err;

pub fn solve(reader: BufReader<File>) -> color_eyre::Result<()> {


    // Read each line
    let mut err = Ok(());
    let priority = reader
        // iterate over each line in the file
        .lines() 
        // catch and panic if an error occurs while iterating
        .scan(&mut err, until_err)
        // Find the outlier
        .map(|line| find_duplicate(&line))
        // determine if the outlier is uppercase or lowercase
        // if uppercase, convert to u8 and subtract __ to get it's priority
        // if lowercase, convert to u8 and subtract __ to get it's priority
        .map(|item| get_priority(item))
        // sum up all priorities of each line
        .sum::<u32>();
    err?;
    println!("Score: {:?}", priority);

    Ok(())
}


fn find_duplicate(s: &str) -> char {
    let x: Vec<_>= s
        .as_bytes()
        .chunks(s.len() / 2)
        .map(str::from_utf8)
        .zip(s
            .as_bytes()
            .chunks(s.len() / 2)
            .map(str::from_utf8)
            .skip(1))
        .collect();
    
    
    let y = x[0].0.unwrap().chars().find(|c| x[0].1.unwrap().chars().contains(c) == true);

    y.unwrap() 
}

fn get_priority(c: char) -> u32 {
    // if uppercase, convert to u8 and subtract __ to get it's priority
    // if lowercase, convert to u8 and subtract __ to get it's priority
    println!("{:?}",c);
    if c.is_ascii_uppercase() {
        println!("{:?}", ((c as u8) as u32) - 37);
        ((c as u8) as u32) - 38
    } else if  c.is_ascii_lowercase() {
        println!("{:?}", ((c as u8) as u32) - 96);
        ((c as u8) as u32) - 96
    } else {
        panic!("not a valid ASCII character")
    }
}