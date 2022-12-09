use color_eyre::eyre::Context;
use itertools::Itertools;


use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::until_err;

pub fn solve(reader: BufReader<File>) -> color_eyre::Result<()> {

    let mut err = Ok(());
    let mut x = reader
        .lines() // iterate over each line in the file
        .scan(&mut err, until_err) // panic if an error is found later on
        .group_by(|vi| !vi.is_empty()) // group elements together, using the empty lines as delimiter
        .into_iter()
        .filter_map(|(filt, group)| // convert text to u32 and sum each group
            if filt {
                Some(group
                    .into_iter()
                    .map(|x| x.parse::<u32>().wrap_err(format!("{:?}", x)).unwrap())
                    .sum::<u32>())
            } else { None })
        .into_iter()
        .collect::<Vec<_>>();

    err?;

    x.sort_by(|a, b| a.cmp(b).reverse()); // sort the sums
    x.truncate(3); // we only care about the top three

    println!("Top Elf: {:?}", x[0]);

    println!("Top Three Elves: {:?}", x.iter().sum::<u32>());

    Ok(())
}

