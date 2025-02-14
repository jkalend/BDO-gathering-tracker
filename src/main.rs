mod session;
mod tshira;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use num_format::{Locale, ToFormattedString};
use std::fs;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter \"SHOW\" to show total data for a spot\n\"SHOW ALL\" to show all the individual sessions in a spot\n\"ADD\" to add a new session");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim_end() {
        "SHOW" => show_data(false),
        "SHOW ALL" => show_data(true),
        "ADD" => add_data(),
        _ => println!("Invalid input"),
    }
    // tshira::tshira_parse();
}

fn show_data(all: bool) {
    let mut input = String::new();
    let gathering_spots = Vec::from(["Tshira Ruins", "Behr"]);
    for (index, spot) in gathering_spots.iter().enumerate() {
        println!("Press {} for {}", index, spot);
    }
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let spot = gathering_spots[input.trim_end().parse::<usize>().unwrap()];
    println!("Showing data for {}", spot);
    match spot {
        "Tshira Ruins" => tshira::tshira_parse(all),
        _ => println!("Invalid input"),
    }
}

fn add_data() {
    let mut input = String::new();
    let gathering_spots = Vec::from(["Tshira Ruins", "Behr"]);
    for (index, spot) in gathering_spots.iter().enumerate() {
        println!("Press {} for {}", index, spot);
    }
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let spot = gathering_spots[input.trim_end().parse::<usize>().unwrap()];
    println!("Adding data for {}", spot);
    match spot {
        "Tshira Ruins" => tshira::tshira_add(),
        _ => println!("Invalid input"),
    }
}
