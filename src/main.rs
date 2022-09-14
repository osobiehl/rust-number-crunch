use csv;
use std::error::Error;
use std::io;
use std::process;
use std::fs;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SAndPHistoricalDaily{
    Date: String,
    Open: f32,
    High: f32,
    Low: f32,
    Close: f32,

}

struct BuyAction{
    amount_stock: f32,
    cost: f32,
    leverage: u32,
}

struct SAndPSimulation{
    
}
/**
 * The value of a stock on a vertain day is determined by some random variable r,
 * r
 * stop loss: we want to find a function SL that maximizes the our profits.
 * 
 * 
 * 
 */

 fn StopLoss(starting_price: f32, current_price: f32){
                      fvm,
 }

struct Simulation_Config{
    stop_loss: f32,
    leverage: u32,
    sto
}


fn remove_shitty_csv(fname: &str)->(){
    let mut vals = fs::read_to_string(fname).unwrap();
    let mut file = fs::File::create(format!("{}_stripped", fname)).unwrap();
    let stripped_vals = vals.replace(" ", "");
    file.write_all(stripped_vals.as_bytes()).expect("could not re-write to file");

}




fn main() {
    const CSV_NAME: &str = "S&P_500_Daily.csv";
    
    // remove_shitty_csv(CSV_NAME);
    // panic!("lmao");

    let mut elements = vec![];
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(fs::File::open(CSV_NAME).unwrap()   );
    for elem in reader.deserialize::<SAndPHistoricalDaily>(){
        match elem{
            Ok(i) => elements.push(i),
            Err(e) => println!("could not convert a row! {}", e)
        }
    }
dbg!(elements);

}
