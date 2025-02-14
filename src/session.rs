use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use num_format::{Locale, ToFormattedString};
use std::fs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Session {
    date: String,
    duration: Number,
    items: Vec<Map<String, Value>>,
}

impl Session {
    pub fn new(date: String, duration: Number, items: HashMap<&str, f64>) -> Session {
        let mut items_vec: Vec<Map<String, Value>> = Vec::new();
        items_vec.push(Map::from_iter(items.iter().map(|(k, v)| (k.to_string(), Value::Number(Number::from_f64(*v).unwrap())))));
        Session {
            date,
            duration,
            items: items_vec,
        }
    }
}

pub fn session_parse(session: Session, prices: &Value) -> (f64, HashMap<String, f64>) {
    let mut total: f64 = 0.0;
    let mut items: HashMap<String, f64> = HashMap::new();
    for item in &session.items {
        for j in item {
            let price = get_price(j.0.as_str(), prices) * j.1.as_f64().unwrap();
            total += price;
            items.insert(j.0.to_string(), price);
            // println!("Item: {}, Price: {}", j.0, (price as i64).to_formatted_string(&Locale::en));
        }
    }
    (total, items)
}

fn get_price(item: &str, prices: &Value) -> f64 {
    match item {
        "Ancient Spirit Dust" => spirit_dust_calc(prices),
        "Fairy Breath" => fairy_breath_calc(prices),
        _ => prices[item].as_f64().unwrap(),
    }
    // let price = prices[item].as_f64().unwrap();
    // price
}

fn spirit_dust_calc(prices: &Value) -> f64 {
    let price = prices["Caphras Stone"].as_f64().unwrap() / (5f64) - prices["Black Stone"].as_f64().unwrap();
    price
}

fn fairy_breath_calc(prices: &Value) -> f64 {
    let price = prices["Sharp Black Crystal Shard"].as_f64().unwrap() * 2f64 / 10f64;
    price
}
