use csv;
use ordered_float::NotNaN;
use rayon::array::IntoIter;
use rayon::prelude::*;
use core::num::flt2dec::strategy;
use std::error::Error;
use std::fmt::Binary;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;
use std::time;
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SAndPHistoricalDaily {
    Date: String,
    Open: f32,
    High: f32,
    Low: f32,
    Close: f32,
}

struct BuyAction {
    amount_stock: NotNaN<f32>,
    cost: NotNaN<f32>,
    leverage: u32,
}

struct SAndPSimulation {}
/**
 * The value of a stock on a vertain day is determined by some random variable r,
 * r
 * stop loss: we want to find a function SL that maximizes the our profits.
 *
 *
 *
 */
fn set_stop_loss(stock: &StockBuy, current_price: NotNaN<f32>, current_time: u64) {}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct StockBuy {
    stop_loss: NotNaN<f32>,
    unit: NotNaN<f32>,
    invested: NotNaN<f32>,
    invested_at: u64,
    wipeout: NotNaN<f32>,
    leverage: u32,
}

enum StockActionFailure {
    UnderWipeout,
    Other,
}
trait StockAction {
    fn will_wipeout(&self, current_price: NotNaN<f32>) -> bool;
    fn will_cashout(&self, current_price: NotNaN<f32>) -> bool;
    fn cashout(&self, current_price: NotNaN<f32>) -> NotNaN<f32>;
    fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StockActionFailure>;
}
impl StockAction for StockBuy {
    fn will_wipeout(&self, current_price: NotNaN<f32>) -> bool {
        return self.wipeout > current_price;
    }
    fn will_cashout(&self, current_price: NotNaN<f32>) -> bool {
        self.will_wipeout(current_price) || self.stop_loss < current_price
    }
    fn cashout(&self, current_price: NotNaN<f32>) -> NotNaN<f32> {
        return self.invested
            + (current_price - self.invested) * NotNaN::from(self.leverage as f32);
    }
    fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StockActionFailure> {
        if self.will_wipeout(amount) {
            return Err(StockActionFailure::UnderWipeout);
        } else {
            self.stop_loss = amount;
        }

        return Ok(());
    }
}
use std::collections::BinaryHeap;

trait BuyableSecurity {
    fn from_price(price: NotNaN<f32>);
}

// struct StockSimulation<T: StockAction, InputType: Into<T>, InputFormat: ParallelIterator>{
//     actions: Vec<T>,
//     data: InputFormat,
// }

struct DollarCostAveragingLinear{
    invest_amount: f32,
    invest_frequency: Duration,
    last_payout: Duration,
}
impl InvestmentStrategy for DollarCostAveragingLinear{
    fn get_funds(&mut self, current_time: &Duration)->f32 {
        if self.last_payout + self.invest_frequency < *current_time{
            self.last_payout = *current_time;
            return self.invest_amount
        }
        0.0
    }
}

trait InvestmentStrategy{
    fn get_funds( &mut self, current_time: &Duration)->f32;
}


struct simulation<T: Ord + StockAction, Strat: InvestmentStrategy> {
    strategy: Strat,
    positions: BinaryHeap<T>,
}
impl<T: Ord + StockAction, Strat: InvestmentStrategy> simulation<T, Strat> {
    fn new(strategy: Strat) -> Self {
        return Self {
            funds: NotNaN::new(funds).unwrap(),
            positions: BinaryHeap::new(),
        };
    }
}

impl<Stock, Iter> StockSimulation<Stock, Iter> for simulation<Stock>
where
    Stock: Ord + StockAction,
    Iter: IntoIterator<Item = Stock>,
{
    fn run(collection: Iter) {}
}

trait StockSimulation<V: StockAction, T: IntoIterator<Item = V>> {
    fn run(collection: T);
}

struct SimulationConfig {
    stop_loss: NotNaN<f32>,
    leverage: u32,
}

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
    for elem in reader.deserialize::<SAndPHistoricalDaily>() {
        match elem {
            Ok(i) => elements.push(i),
            Err(e) => println!("could not convert a row! {}", e),
        }
    }
    let arr_par = elements.into_par_iter();
    // dbg!(elements);
}
