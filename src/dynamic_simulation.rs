use std::ops::Not;

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

pub trait Generator{
    fn generate<S,T>(&mut self, current_stock: &S, action: &T) ->NotNaN<f32> where
    S: Stock,
    T: StockAction<S>;
}

impl Generator for PercentageMaxGenerator{
    fn generate<S,T>(&mut self, _current_stock: &S, action: &T) ->NotNaN<f32> where
        S: Stock,
        T: StockAction<S> {
        return self.percentage * action.get_max().price / NotNaN::from(100.0);
    }
}

pub struct DCAWithTrailingStopLoss<G : Generator>{
    pub internal_strat_: DollarCostAveragingLinear,
    pub generator_: G
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
        if amount != 0.0{
            eprintln!("amount returned: {amount}");
        }
    }
    fn total_invested(&self) -> f32 {
        InvestmentStrategy::<S,T>::total_invested(& self.internal_strat_)
    }
}