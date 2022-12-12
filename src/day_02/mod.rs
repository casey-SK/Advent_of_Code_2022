use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::until_err;

#[derive(PartialEq)]
enum Hand {
    Rock, Paper, Scissors
}

#[derive(PartialEq)]
enum Outcome {
    Win, Lose, Draw
}

pub fn solve(reader: BufReader<File>) -> color_eyre::Result<()> {

    // create total_score mutable variable
    //let mut score: u32 = 0;

    // read line and get two variables: (i) opponents hand and (ii) your hand
    // get overall score
    let mut err = Ok(());
    let score = reader
        .lines() // iterate over each line in the file
        .scan(&mut err, until_err)
        .map(|line| {get_score(line.split_whitespace().collect())})
        .sum::<u32>();
    
    println!("Score: {score:?}");

    Ok(())
}

fn get_score(s: Vec<&str>) -> u32 {
    
    let opponent_hand = get_hand(s[0]);
    let your_outcome = get_outcome(s[1]);

    // A: determine your hand score (1 for rock, 2 for paper, 3 for scissors)
    // B: determine your round score (0 for losing, 3 for draw, 6 for winning)
    let hand_score: u32;
    let round_score: u32;
    
    match opponent_hand {
        Hand::Rock => if your_outcome == Outcome::Win {round_score = 6; hand_score = 2}
                      else if your_outcome == Outcome::Draw {round_score = 3; hand_score = 1}
                      else {round_score = 0; hand_score = 3},
        Hand::Paper => if your_outcome == Outcome::Win {round_score = 6; hand_score = 3}
                       else if your_outcome == Outcome::Draw {round_score = 3; hand_score = 2}
                       else {round_score = 0; hand_score = 1},
        Hand::Scissors => if your_outcome == Outcome::Win {round_score = 6; hand_score = 1}
                          else if your_outcome == Outcome::Draw {round_score = 3; hand_score = 3}
                          else {round_score = 0; hand_score = 2},
    }

    // add A + B to the overall score
    hand_score + round_score
}

fn get_hand(s: &str) -> Hand {
    if s == "A" {
        Hand::Rock
    } else if s == "B" {
        Hand::Paper
    } else if s == "C" {
        Hand::Scissors
    } else {
        panic!("Invalid opponent hand")
    }
}

fn get_outcome(s: &str) -> Outcome {
    if s == "X" {
        Outcome::Lose
    } else if s == "Y" {
        Outcome::Draw
    } else if s == "Z"{
        Outcome::Win
    } else {
        panic!("invalid outcome");
    }
}