use crate::stock_action::StockAction::{BuyableSecurity, Stock, StockAction};
use chrono::{format::Fixed, Date, DateTime, Duration, Utc};
use ordered_float::NotNaN;
use std::cmp::Reverse;
use std::{collections::BinaryHeap, fmt::Debug, marker::PhantomData, ops::Not};

pub struct DollarCostAveragingLinear {
    invest_amount: f32,
    invest_frequency: Duration,
    last_payout: DateTime<Utc>,
    total_invested: f32,
    leverage: f32,
}

impl DollarCostAveragingLinear {
    pub fn new(invest_amount: f32, invest_frequency: Duration, leverage: f32) -> Self {
        return DollarCostAveragingLinear {
            invest_amount,
            invest_frequency,
            last_payout: DateTime::<Utc>::MIN_UTC,
            total_invested: 0.0,
            leverage,
        };
    }
}

impl<S, T> InvestmentStrategy<S, T> for DollarCostAveragingLinear
where
    S: Stock,
    T: StockAction<S>,
{
    fn get_funds(&mut self, stock: &S) -> f32 {
        let current_time = stock.get_time();
        if self.last_payout + self.invest_frequency < current_time {
            self.last_payout = current_time;
            self.total_invested += self.invest_amount;
            return self.invest_amount;
        }
        0.0
    }
    fn inspect_positions(
        &mut self,
        positions: &mut BinaryHeap<T>,
        current_stock: &S,
    ) -> () {
        // do nothing since this is just DCA
    }
    fn get_leverage(&self) -> NotNaN<f32> {
        return NotNaN::from(self.leverage);
    }
    fn total_invested(&self) -> f32 {
        self.total_invested
    }
}

pub trait InvestmentStrategy<S, T>
where
    S: Stock,
    T: StockAction<S>,
{
    fn get_funds(&mut self, stock: &S) -> f32;
    fn inspect_positions(
        &mut self,
        positions: &mut BinaryHeap<T>,
        current_stock: &S,
    ) -> ();
    fn get_leverage(&self) -> NotNaN<f32>;
    fn total_invested(&self) -> f32;
}

pub struct Simulation<S, T, Strat>
where
    S: Stock,
    T: Ord + StockAction<S>,
    Strat: InvestmentStrategy<S, T>,
{
    s: PhantomData<S>,
    funds: f32,
    strategy: Strat,
    positions: BinaryHeap<T>,
}
impl<S: Stock, T: Ord + StockAction<S>, Strat: InvestmentStrategy<S, T>> Simulation<S, T, Strat> {
    pub fn new(strategy: Strat) -> Self {
        return Self {
            s: Default::default(),
            strategy: strategy,
            positions: BinaryHeap::new(),
            funds: 0.0,
        };
    }
    fn remove_stop_losses(&mut self, current_stock: &S) -> Option<()> {
        loop {
            let current_elem = self.positions.peek()?;
            if current_elem.will_cashout(current_stock) {
                drop(current_elem);
                let to_remove = self.positions.pop()?;
                self.funds += to_remove.cashout(current_stock).into_inner();
                
                println!("funds after stop loss value: {:?}", to_remove);
            } else {
                drop(current_elem);

                return Some(());
            }
        }
    }
    fn adjust_positions(&mut self, current_stock: &S) {
        self.strategy
            .inspect_positions(&mut self.positions, current_stock);
    }
    fn get_total_funds(&self, current_stock: &S) -> NotNaN<f32> {
        return self
            .positions
            .iter()
            .fold(NotNaN::from(0.0), |accum, elem| {
                accum + elem.cashout(current_stock)
            })
            + NotNaN::new(self.funds).unwrap();
    }
}

impl<S, Action, Strat> StockSimulation<S> for Simulation<S, Action, Strat>
where
    S: Stock,
    Action: Ord + StockAction<S>,
    Strat: InvestmentStrategy<S, Action>,
{
    fn run(&mut self, collection: &[S]) -> f32 {
        collection.into_iter().for_each(|item| {
            self.funds += self.strategy.get_funds(item);
            // see if we have funds
            if self.funds > 0.0 {
                let n = NotNaN::new(self.funds).unwrap();
                self.positions.push(Action::from(
                    &item,
                    n,
                    self.strategy.get_leverage(),
                ));
                self.funds = 0.0;
            }
            self.remove_stop_losses(item);

            self.adjust_positions(item);
            // remove positions under amount
        });
        let final_elem = collection.last();

        return match final_elem {
            None => {
                eprintln!("err: final elem is None");
                0.0
            }
            Some(stock) => self.get_total_funds(stock).into_inner(),
        };
    }
    fn total_invested(&self) -> f32 {
        return self.strategy.total_invested();
    }
}

pub trait StockSimulation<T: Stock> {
    fn run(&mut self, collection: &[T]) -> f32;
    fn total_invested(&self) -> f32;
}
