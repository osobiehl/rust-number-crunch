use chrono::Duration;
use std::{collections::BinaryHeap, marker::PhantomData};
use ordered_float::NotNaN;
use std::cmp::Reverse;
use crate::stock_action::StockAction::{StockAction, Stock, BuyableSecurity};

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
        fn get_funds<S: Stock>( &mut self, stock: &S)->f32;
    }
    
    
    struct Simulation<S, T, Strat> where
    S: Stock,
    T: Ord + StockAction<S>,
    Strat: InvestmentStrategy
    {
        s: PhantomData<S>,
        funds: f32,
        strategy: Strat,
        positions: BinaryHeap<Reverse<T>>,
    }
    impl<S: Stock, T: Ord + StockAction<S>, Strat: InvestmentStrategy> Simulation<S, T, Strat> {
        fn new(strategy: Strat) -> Self {
            return Self {
                s: Default::default(),
                strategy: strategy,
                positions: BinaryHeap::new(),
                funds: 0.0
            };
        }
        fn remove_wipeouts(&mut self, current_stock: &S)->Option<()>{
            loop {
                let current_elem = self.positions.peek()?;
                if current_elem.0.will_wipeout(current_stock){
                    drop(current_elem);
                    self.funds += self.positions.pop()?.0.cashout(current_stock).into_inner();
                }

                
            }
        }
        fn remove_stop_losses(&mut self){

        }
    }
    
    impl<S, Action, Strat> StockSimulation<S> for Simulation<S, Action, Strat>
    where
        S: Stock,
        Action: Ord + StockAction<S>,
        Strat: InvestmentStrategy
    {
        fn run(&self, collection: &[S]) -> f32{
            collection.into_iter().map(
                |item| {
                    self.funds  += self.strategy.get_funds(item);
                    if self.funds > 0.0 {
                        let n = NotNaN::new(self.funds).unwrap();
                        self.positions.push( Reverse(Action::from(&item, n)) );
                        self.funds = 0.0;
                    }
                    unimplemented!("finish!");


                    return ();
                }
            );
            return 0.0;
        }
    }
    
    trait StockSimulation<T: Stock> {
        fn run(&self, collection: &[T]) -> f32;
    }
    
    struct SimulationConfig {
        stop_loss: NotNaN<f32>,
        leverage: u32,
    }
