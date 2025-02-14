use crate::session::{Session, session_parse};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use num_format::{Locale, ToFormattedString};
use serde_json::{Number, Value};
use log::debug;
use chrono::Local;

pub fn tshira_parse(all: bool) {
    let prices = fs::read_to_string("prices.json").expect("Unable to read file");
    let prices_parsed: Value = serde_json::from_str(&prices).expect("Unable to parse json");

    let tshira = fs::read_to_string("tshira.json").expect("Unable to read file");
    let v: Value = serde_json::from_str(&tshira).expect("Unable to parse json");

    let mut overall_map: HashMap<String, f64> = HashMap::new();
    let mut overall_total: f64 = 0.0;
    for (i, session) in v["sessions"].as_array().unwrap().iter().enumerate() {
        let session: Session = serde_json::from_value(session.to_owned()).expect("Unable to parse session");
        let (total, map) = session_parse(session, &prices_parsed);
        for item in &map {
            let entry = overall_map.entry(item.0.to_string()).or_insert(0.0);
            *entry += item.1;
        }
        overall_total += total;
        if all {
            println!("---------------------------------");
            println!("Session {}", i);
            for item in map {
                println!("Item: {}, Price: {}", item.0, (item.1 as i64).to_formatted_string(&Locale::en));
            }
            println!("---------------------------------");
            println!("Total for session {}: {}", i, (total as i64).to_formatted_string(&Locale::en));
        }
    }
    println!("\n---------------------------------");
    for item in overall_map {
        println!("Item: {}, Price: {}", item.0, (item.1 as i64).to_formatted_string(&Locale::en));
    }
    println!("---------------------------------");
    println!("Total: {}", (overall_total as i64).to_formatted_string(&Locale::en));
}

pub fn tshira_add() {
    let prices = fs::read_to_string("prices.json").expect("Unable to read file");
    let prices_parsed: Value = serde_json::from_str(&prices).expect("Unable to parse json");

    let tshira = fs::read_to_string("tshira.json").expect("Unable to read file");
    let mut v: Value = serde_json::from_str(&tshira).expect("Unable to parse json");

    let mut input = String::new();

    let date = Local::now().format("%Y-%m-%d").to_string();
    // let date = String::from("0000-00-00");//input.trim_end().to_string();
    // input.clear();

    println!("Enter the duration of the session in minutes");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let duration = input.trim_end().parse::<f64>().unwrap();
    input.clear();

    let mut items: HashMap<&str, f64> = HashMap::new();
    for item in v["items"].as_array().unwrap() {
        // println!("Add number of {} gathered", item);
        print!("Add number of {} gathered: ", item);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim_end() == "" {
            items.insert(item.as_str().unwrap(), 0f64);
            continue;
        }
        let amount = input.trim_end().parse::<f64>().unwrap();
        items.insert(item.as_str().unwrap(), amount);
        input.clear();
    }

    let session = Session::new(date, Number::from_f64(duration).unwrap(), items);
    let mut sessions = v["sessions"].as_array().unwrap().to_owned();
    sessions.push(serde_json::to_value(session).expect("Unable to serialize session"));
    v["tries"] = serde_json::to_value(v["tries"].as_i64().unwrap() + 1).expect("Unable to serialize tries");
    v["sessions"] = serde_json::to_value(sessions).expect("Unable to serialize sessions");

    let tshira = fs::File::create("tshira.json").expect("Unable to create file");
    serde_json::to_writer(tshira, &v).expect("Unable to write to file");

    println!("Session added");
}
