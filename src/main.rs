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
use crate::stock_action::StockAction;

mod s_and_p;
mod simulation;
mod stock_action;

use s_and_p::SAndPHistoricalDaily;

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



impl StockAction for StockBuy {
    type UnderlyingAsset = SAndPHistoricalDaily;
    fn from(stock: Self::UnderlyingAsset)->Self{

        return StockBuy { stop_loss: (),
             unit: (),
             invested: (), 
             invested_at: (), 
             wipeout: (), 
             leverage: () }
    }
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
    for elem in reader.deserialize::<SAndPHistoricalDaily>() {
        match elem {
            Ok(i) => elements.push(i),
            Err(e) => println!("could not convert a row! {}", e),
        }
    }
    let arr_par = elements.into_par_iter();
    // dbg!(elements);
}
