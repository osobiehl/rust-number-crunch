use crate::dynamic_simulation::PercentageMaxGenerator;
use crate::simulation::StockSimulation;
use crate::stock_action::StockAction::StockInvestment;
use chrono::Duration;
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
mod dynamic_simulation;
use s_and_p::{BestCaseSAndP, SAndPHistoricalDaily, SAndPHistoricalDailyRaw, WorstCaseSAndP};
use simulation::{DollarCostAveragingLinear, InvestmentStrategy, Simulation};
use stock_action::StockAction::StockAction;
use dynamic_simulation::{DCAWithTrailingStopLoss, Generator, TimeVariantPercentage};
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
        .filter_map(|s| s.ok()?.try_into().ok())
        .collect();

    let s_and_p_worst_case: Vec<_> = elements.clone().into_iter().rev().map(WorstCaseSAndP).collect();

    println!("{:^10}|{:^10}|{:^10}|{:^10}", "leverage", "investment", "return", "fraction");
    const MAX_LEVERAGE: usize = 20;

    for leverage in 1..=MAX_LEVERAGE{
        let dollar_cost_average = DollarCostAveragingLinear::new(1000.0, chrono::Duration::days(30), leverage as f32);
        let mut simulation: Simulation<WorstCaseSAndP, StockInvestment, DollarCostAveragingLinear> =
            Simulation::new(dollar_cost_average);
        let total = simulation.run(&s_and_p_worst_case);
        let invested = simulation.total_invested();
        let fraction: f32 = total/invested;
        println!("{leverage:^10}|{invested:^10}|{total:^10}|{fraction:^10}");
    }


    // let s_and_p_best_case: Vec<_> = elements.clone().into_iter().rev().map(BestCaseSAndP).collect();
    // println!("=======================BEST CASE==================");
    // for leverage in 1..=MAX_LEVERAGE{
    //     let dollar_cost_average = DollarCostAveragingLinear::new(1000.0, chrono::Duration::days(30), leverage as f32);
    //     let mut simulation: Simulation<BestCaseSAndP, StockInvestment, DollarCostAveragingLinear> =
    //         Simulation::new(dollar_cost_average);
    //     let total = simulation.run(&s_and_p_best_case);
    //     let invested = simulation.total_invested();
    //     let fraction: f32 = total/invested;
    //     println!("{leverage:^10}|{invested:^10}|{total:^10}|{fraction:^10}");
    // }
    const TWENTY_PERCENT: f32 = 20.0;
    const MINUS_FIFTY_PERCENT: f32 = -50.0;

    println!("=======================WORST CASE 20%==================");
    for leverage in 1..=MAX_LEVERAGE{
        let dollar_cost_average = DollarCostAveragingLinear::new(1000.0, chrono::Duration::days(30), leverage as f32);
        
        let generator = PercentageMaxGenerator::try_new(TWENTY_PERCENT).unwrap();
        let dyn_dca = DCAWithTrailingStopLoss::new(dollar_cost_average, generator);

        let mut simulation: Simulation<WorstCaseSAndP, StockInvestment, DCAWithTrailingStopLoss<PercentageMaxGenerator>> =
            Simulation::new(dyn_dca);
        let total = simulation.run(&s_and_p_worst_case);
        let invested = simulation.total_invested();
        let fraction: f32 = total/invested;
        println!("{leverage:^10}|{invested:^10}|{total:^10}|{fraction:^10}");
    }

    println!("=======================DYNAMIC 20%==================");
    for leverage in 1..=MAX_LEVERAGE{
        let dollar_cost_average = DollarCostAveragingLinear::new(1000.0, chrono::Duration::days(30), leverage as f32);
        let generator = TimeVariantPercentage::new(MINUS_FIFTY_PERCENT, TWENTY_PERCENT, Duration::days(180));
        let dyn_dca = DCAWithTrailingStopLoss::new(dollar_cost_average, generator);

        let mut simulation: Simulation<WorstCaseSAndP, StockInvestment, DCAWithTrailingStopLoss<TimeVariantPercentage>> =
            Simulation::new(dyn_dca);
        let total = simulation.run(&s_and_p_worst_case);
        let invested = simulation.total_invested();
        let fraction: f32 = total/invested;
        println!("{leverage:^10}|{invested:^10}|{total:^10}|{fraction:^10}");
    }
    // dbg!(elements);
}
