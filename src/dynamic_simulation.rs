use crate::simulation::{InvestmentStrategy, DollarCostAveragingLinear};
use crate::stock_action::StockAction::{StockAction, Stock};

pub struct DCAWithTrailingStopLoss{
    internal_strat_: DollarCostAveragingLinear,
}
    
    
impl<S,T> InvestmentStrategy<S,T> for DCAWithTrailingStopLoss where
S: Stock,
T: StockAction<S>
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
        
        
            
    }
    fn return_funds(&mut self, amount: f32) {
        
    }
    fn total_invested(&self) -> f32 {
        InvestmentStrategy::<S,T>::total_invested(& self.internal_strat_)
    }
}