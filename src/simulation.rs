use chrono::{Duration, DateTime, format::Fixed, Utc, Date};
use std::{collections::BinaryHeap, marker::PhantomData};
use ordered_float::NotNaN;
use std::cmp::Reverse;
use crate::stock_action::StockAction::{StockAction, Stock, BuyableSecurity};

    pub struct DollarCostAveragingLinear{
        invest_amount: f32,
        invest_frequency: Duration,
        last_payout: DateTime<Utc>,
    }

    impl DollarCostAveragingLinear{
        pub fn new(invest_amount: f32, invest_frequency: Duration)->Self{
            return DollarCostAveragingLinear { invest_amount, invest_frequency, last_payout: DateTime::<Utc>::MIN_UTC }
        }

    }

    impl<S,T> InvestmentStrategy<S,T> for DollarCostAveragingLinear where
    S: Stock,
    T: StockAction<S> 
    {
        fn get_funds(&mut self, stock: &S)->f32 {
            let current_time = stock.get_time();
            if self.last_payout + self.invest_frequency < current_time{
                self.last_payout = current_time;
                return self.invest_amount
            }
            0.0
        }
        fn inspect_positions(&mut self, positions: &mut BinaryHeap<Reverse<T>>, current_stock: &S ) -> () {
            // do nothing since this is just DCA
        }
        fn get_leverage(&self) -> NotNaN<f32> {
            return NotNaN::from(1.0);
        }
    }
    
    pub trait InvestmentStrategy<S, T> where
    S: Stock,
    T: StockAction<S>
    {
        fn get_funds( &mut self, stock: &S)->f32;
        fn inspect_positions(&mut self, positions: &mut BinaryHeap<Reverse<T>>, current_stock: &S) -> ();
        fn get_leverage(&self) -> NotNaN<f32>;
    }
    
    
    pub struct Simulation<S, T, Strat> where
    S: Stock,
    T: Ord + StockAction<S>,
    Strat: InvestmentStrategy<S,T>
    {
        s: PhantomData<S>,
        funds: f32,
        strategy: Strat,
        positions: BinaryHeap<Reverse<T>>,
    }
    impl<S: Stock, T: Ord + StockAction<S>, Strat: InvestmentStrategy<S,T>> Simulation<S, T, Strat> {
        pub fn new(strategy: Strat) -> Self {
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
                    dbg!("funds after wipeout value: {:?}", self.funds);
                }
            }
        }
        fn remove_stop_losses(&mut self, current_stock: &S)->Option<()>{
            loop {
                let current_elem = self.positions.peek()?;
                if current_elem.0.will_cashout(current_stock){
                    drop(current_elem);
                    self.funds += self.positions.pop()?.0.cashout(current_stock).into_inner();
                    dbg!("funds after stop loss value: {:?}", self.funds);
                }
            }
        }
        fn adjust_positions(&mut self, current_stock: &S){
            self.strategy.inspect_positions(&mut self.positions, current_stock);
        }
    }
    
    impl<S, Action, Strat> StockSimulation<S> for Simulation<S, Action, Strat>
    where
        S: Stock,
        Action: Ord + StockAction<S>,
        Strat: InvestmentStrategy<S, Action>
    {
        fn run(&mut self, collection: &[S]) -> f32{
            collection.into_iter().map(
                |item| {
                    self.funds  += self.strategy.get_funds(item);
                    // see if we have funds
                    if self.funds > 0.0 {
                        let n = NotNaN::new(self.funds).unwrap();
                        self.positions.push( Reverse(Action::from(&item, n, self.strategy.get_leverage())) );
                        self.funds = 0.0;
                    }
                    self.remove_stop_losses(item);
                    self.remove_wipeouts(item);
                    self.adjust_positions(item);
                    // remove positions under amount
                    return ();
                }
            );
            return 0.0;
        }
    }
    
    pub trait StockSimulation<T: Stock> {
        fn run(&mut self, collection: &[T]) -> f32;
    }
    
