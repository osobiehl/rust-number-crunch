use std::ops::Not;

use chrono::{Duration, Utc, DateTime};
use ordered_float::NotNaN;

use crate::simulation::{InvestmentStrategy, DollarCostAveragingLinear, StockSimulation};
use crate::stock_action::StockAction::{StockAction, Stock};

pub struct PercentageMaxGenerator{
     percentage: NotNaN<f32>,
}
impl PercentageMaxGenerator{
    pub fn try_new(val: f32) -> Option<Self>{
        if 0.0 < val && val <= 100.0{
            return Some(PercentageMaxGenerator{ percentage: NotNaN::new(val).ok()? });
        }
        None
    }
}

pub struct TimeVariantPercentage{
    end_percentage: NotNaN<f32>,
    start_percentage: NotNaN<f32>,
    ramp_up: Duration,
    start_date: Option<DateTime<Utc>>,
}

impl TimeVariantPercentage{
    pub fn new(start: f32, end: f32, ramp_up: Duration)->Self{
        Self { end_percentage: NotNaN::from(end), start_percentage: NotNaN::from(start), ramp_up, start_date: None}
    }
    fn is_in_ramp_duration<S: Stock>(&mut self, stock: &S) -> bool{
            if let None = self.start_date{
                self.start_date = Some(stock.get_time());
            }
            return self.start_date.unwrap() + self.ramp_up > stock.get_time();
    }
    fn ramp_generate_percentage<S: Stock>(&self, stock: &S) -> NotNaN<f32>{
        let current_duration = stock.get_time() - self.start_date.unwrap();
        let duration_ratio = NotNaN::from(current_duration.num_milliseconds() as f32 / self.ramp_up.num_milliseconds() as f32);
        let to_generate = self.start_percentage + duration_ratio * (self.start_percentage - self.end_percentage) ;
        return to_generate;
    }

}
impl Generator for TimeVariantPercentage{
    fn generate<S,T>(&mut self, current_stock: &S, action: &T) ->NotNaN<f32> where
        S: Stock,
        T: StockAction<S> {
            if self.is_in_ramp_duration(current_stock){
                return self.ramp_generate_percentage(current_stock) * action.get_original_price();
            }
            else {
                return self.end_percentage * current_stock.get_sell_price() / NotNaN::from(100.0) + action.get_original_price();
            }
        
    }
}

pub trait Generator{
    fn generate<S,T>(&mut self, current_stock: &S, action: &T) ->NotNaN<f32> where
    S: Stock,
    T: StockAction<S>;
}

impl Generator for PercentageMaxGenerator{
    fn generate<S,T>(&mut self, current_stock: &S, action: &T) ->NotNaN<f32> where
        S: Stock,
        T: StockAction<S> {
            let investment_original = action.get_original_price();
            let current_price = current_stock.get_sell_price();
            let delta = current_price - investment_original;
            let stop_loss_gain = delta * self.percentage / NotNaN::from(100.0);
            return action.get_original_price() + stop_loss_gain;

    }
}

pub struct DCAWithTrailingStopLoss<G : Generator>{
    pub internal_strat_: DollarCostAveragingLinear,
    pub generator_: G,
    returned_funds: f32,
}
    
impl<G: Generator> DCAWithTrailingStopLoss<G>{
    pub fn new(internal_strat_: DollarCostAveragingLinear, generator_: G)->Self{
        Self { internal_strat_, generator_, returned_funds: 0.0 }
    }
}
    
impl<S,T, G> InvestmentStrategy<S,T> for DCAWithTrailingStopLoss<G> where
S: Stock,
T: StockAction<S> + Ord,
G: Generator
{
    fn get_funds(&mut self, stock: &S) -> f32 {
        InvestmentStrategy::<S,T>::get_funds(&mut self.internal_strat_, stock)

    }
    fn get_leverage(&self) -> ordered_float::NotNaN<f32> {

        InvestmentStrategy::<S,T>::get_leverage(& self.internal_strat_)
    }
    fn inspect_positions(
            &mut self,
            positions: &mut std::collections::BinaryHeap<T>,
            current_stock: &S,
        ) -> () {
            let v: Vec<T> = positions.drain().map( |mut e| {
                if e.eval(current_stock){
                    let new_sl = self.generator_.generate(current_stock, &e);
                    e.set_stop_loss(new_sl).ok();
                }
                return e
            }).collect();
            for elem in v{
                positions.push(elem);
            }
        

    }
    fn return_funds(&mut self, amount: f32) {
        self.returned_funds += amount;
    }
    fn total_invested(&self) -> f32 {
        InvestmentStrategy::<S,T>::total_invested(& self.internal_strat_) + self.returned_funds
    }
}