use itertools::Itertools;
use std::fs::File;
use std::iter::Skip;
use std::str;
use std::io::{prelude::*, BufReader};
use itertools::Chunk;

use crate::until_err;


pub fn solve(reader: BufReader<File>) -> color_eyre::Result<()> {


    // Read each line
    let mut err = Ok(());
    let (item_priority, badge_priority) = reader
        // iterate over each line in the file
        .lines() 
        // catch and panic if an error occurs while iterating
        .scan(&mut err, until_err)
        // Find the outlier
        .array_chunks::<3>()
        .map(|trio| find_duplicates(&trio))
        // determine if the outlier is uppercase or lowercase
        // if uppercase, convert to u8 and subtract __ to get it's priority
        // if lowercase, convert to u8 and subtract __ to get it's priority
        .map(|item| get_priority(item))
        // sum up all priorities of each line
        .fold((0, 0), |(a1,a2), (x1, x2)| (a1+x1, a2+x2));
    err?;
    println!("Item Priority: {:?}", item_priority);
    println!("Badge Priority: {:?}", badge_priority);

    Ok(())
}


fn find_duplicates(s: &[String; 3]) -> (char, char, char, char) {

    // find item duplicates
    let mut item_chars: Vec<char> = vec![];
    for i in s {
        let item_x: Vec<_>= i
            .as_bytes()
            .chunks(i.len() / 2)
            .map(str::from_utf8)
            .zip(i
                .as_bytes()
                .chunks(i.len() / 2)
                .map(str::from_utf8)
                .skip(1))
            .collect();
        
        let item_y = item_x[0].0.unwrap().chars().find(|c| item_x[0].1.unwrap().chars().contains(c) == true);
    
        item_chars.push(item_y.unwrap());


    }
    let badge_dup = s[0].chars().find(|c| (s[1].chars().contains(c) == true) && (s[2].chars().contains(c) == true));
    //let badge_x: Vec<_> = s
            
    //println!("({:?}, {:?}, {:?})", item_chars[0], item_chars[1], item_chars[2]);
    (item_chars[0], item_chars[1], item_chars[2], badge_dup.unwrap())
    //(item_y.unwrap(), badge_y.unwrap())
}

fn get_priority((i1, i2, i3, b): (char, char, char, char)) -> (u32, u32) {
    // if uppercase, convert to u8 and subtract __ to get it's priority
    // if lowercase, convert to u8 and subtract __ to get it's priority
    
    let mut item_sum: Vec<u32> = vec![];

    for c in [i1, i2, i3] {
        //println!("{:?}",c);
        if c.is_ascii_uppercase() {
            //println!("{:?}", ((c as u8) as u32) - 38);
            item_sum.push(((c as u8) as u32) - 38);
        } else if  c.is_ascii_lowercase() {
            //println!("{:?}", ((c as u8) as u32) - 96);
            item_sum.push(((c as u8) as u32) - 96);
        } else {
            panic!("not a valid ASCII character")
        }
    }

    let mut b_sum;
    //println!("{:?}",b);
    if b.is_ascii_uppercase() {
        //println!("{:?}", ((b as u8) as u32) - 37);
        b_sum = ((b as u8) as u32) - 38;
    } else if  b.is_ascii_lowercase() {
        //println!("{:?}", ((b as u8) as u32) - 96);
        b_sum = ((b as u8) as u32) - 96;
    } else {
        panic!("not a valid ASCII character")
    }

    (item_sum.into_iter().sum(), b_sum)
}