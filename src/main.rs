use crate::simulation::StockSimulation;
use crate::stock_action::StockAction::StockInvestment;
use csv;
use ordered_float::NotNaN;
use rayon::array::IntoIter;
use rayon::prelude::*;
use std::error::Error;
use std::fmt::Binary;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;
use std::time;

mod s_and_p;
mod simulation;
mod stock_action;
use stock_action::StockAction::StockAction;
use s_and_p::{SAndPHistoricalDaily, SAndPHistoricalDailyRaw, BestCaseSAndP, WorstCaseSAndP};
use simulation::{DollarCostAveragingLinear, InvestmentStrategy, Simulation};
struct SAndPSimulation {}

// struct StockSimulation<T: StockAction, InputType: Into<T>, InputFormat: ParallelIterator>{
//     actions: Vec<T>,
//     data: InputFormat,
// }

fn remove_shitty_csv(fname: &str) -> () {
    let mut vals = fs::read_to_string(fname).unwrap();
    let mut file = fs::File::create(format!("{}_stripped", fname)).unwrap();
    let stripped_vals = vals.replace(" ", "");
    file.write_all(stripped_vals.as_bytes())
        .expect("could not re-write to file");
}

fn main() {
    const CSV_NAME: &str = "S&P_500_Daily.csv";

    // remove_shitty_csv(CSV_NAME);
    // panic!("lmao");

    let mut elements: Vec<SAndPHistoricalDaily> = vec![];
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(fs::File::open(CSV_NAME).unwrap());

    let elements: Vec<SAndPHistoricalDaily> = reader
        .deserialize::<SAndPHistoricalDailyRaw>()
        .filter_map(|s| s.ok()?.try_into().ok()).collect();

    let s_and_p_worst_case: Vec<_> = elements.into_iter().map(WorstCaseSAndP).collect();
    

    let dollar_cost_average = DollarCostAveragingLinear::new(1000.0, chrono::Duration::days(30));
    let mut simulation: Simulation<WorstCaseSAndP, StockInvestment, DollarCostAveragingLinear >= Simulation::new(dollar_cost_average);
    simulation.run(&s_and_p_worst_case);
    
    // dbg!(elements);
}
