use chrono::Duration;

use crate::stock_action::StockAction::StockAction;

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
        funds: f32,
        strategy: Strat,
        positions: BinaryHeap<T>,
    }
    impl<T: Ord + StockAction, Strat: InvestmentStrategy> simulation<T, Strat> {
        fn new(strategy: Strat) -> Self {
            return Self {
                strategy: strategy,
                positions: BinaryHeap::new(),
            };
        }
    }
    
    impl<Stock, Iter, Strat> StockSimulation<Stock, Iter> for simulation<Stock, Strat>
    where
        Stock: Ord + StockAction,
        Iter: IntoIterator<Item = Stock>,
        Strat: InvestmentStrategy
    {
        fn run(&self, collection: Iter) {
            collection.into_iter().map(
                |item| {
                    self.funds  += self.strategy.get_funds()
                }
            )
        }
    }
    
    trait StockSimulation<V: StockAction, T: IntoIterator<Item = V>> {
        fn run(&self, collection: T) -> f32;
    }
    
    struct SimulationConfig {
        stop_loss: NotNaN<f32>,
        leverage: u32,
    }
