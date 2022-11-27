pub mod StockAction {
    use std::fmt::{Debug};
    use chrono::{prelude::*, Duration, DateTime};
    use ordered_float::NotNaN;
    pub enum StockActionFailure {
        UnderWipeout,
        Other,
    }

    // at some point: expand to use spread
    pub trait Stock: Debug {
        fn get_time(&self) -> DateTime<Utc> ;
        fn get_buy_price(&self) -> NotNaN<f32>;
        fn get_sell_price(&self) -> NotNaN<f32>;
    }
    #[non_exhaustive]
    pub enum StopLossFailure {
        UnderWipeout,
        WillImmediatelyCashOut,
    }

    #[derive(Debug, Eq, PartialEq, PartialOrd)]
    pub struct StockTimeSnapshot{
        pub price: NotNaN<f32>,
        pub time: DateTime<Utc>
    }


    pub trait StockAction<T: BuyableSecurity>: Debug {
        fn from(stock: & T, currency_invested: NotNaN<f32>, leverage: NotNaN<f32>) -> Self;
        fn eval(&mut self, stock: &T)->bool;
        fn will_wipeout(&self, current_stock: &T) -> bool;
        fn will_cashout(&self, current_stock: &T) -> bool;
        fn cashout(&self, current_stock: &T) -> NotNaN<f32>;
        fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StopLossFailure>;
        fn get_max(&self) -> &StockTimeSnapshot;
        fn get_min(&self) -> &StockTimeSnapshot;
    }
    pub trait BuyableSecurity: Debug {
        fn get_price(&self) -> NotNaN<f32>;
    }
    impl<T: Stock> BuyableSecurity for T {
        fn get_price(&self) -> NotNaN<f32> {
            return <Self as Stock>::get_sell_price(&self);
        }
    }

    #[derive(Eq, Debug)]
    pub struct StockInvestment {
        stop_loss: NotNaN<f32>,
        unit: NotNaN<f32>,
        invested: NotNaN<f32>,
        invested_at: DateTime<Utc>,
        invested_at_price: NotNaN<f32>,
        wipeout_at: NotNaN<f32>,
        leverage: NotNaN<f32>,
        max: StockTimeSnapshot,
        min: StockTimeSnapshot,
    }
    
    impl PartialEq for StockInvestment{
        fn eq(&self, other: &Self) -> bool {
            self.stop_loss == other.stop_loss
        }
    }
    impl PartialOrd for StockInvestment{
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.stop_loss.cmp(&other.stop_loss))
        }
    }
    impl Ord for StockInvestment{
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            return self.stop_loss.cmp(&other.stop_loss);
        }
    }


    impl StockInvestment{
        fn will_wipeout_(&self, price: NotNaN<f32>)->bool{
             return self.wipeout_at > price;
        }
    }

    impl<T: Stock> StockAction<T> for StockInvestment
    {
        fn from(
            stock: &T,
            currency_invested: NotNaN<f32>,
            leverage: NotNaN<f32>,
        ) -> Self {
            assert!(leverage > NotNaN::new(0.0).unwrap(), "0 leverage given!");
            let v = stock.get_buy_price();
            let price = stock.get_buy_price();
            let time = stock.get_time();
            return Self {
                stop_loss: v - (v / leverage),
                unit: (currency_invested / price) * leverage,
                invested: currency_invested,
                invested_at: time,
                wipeout_at: v - (v / leverage),
                leverage,
                invested_at_price: stock.get_buy_price(),
                max: StockTimeSnapshot { price, time},
                min: StockTimeSnapshot { price, time},
            };
        }
        fn eval(&mut self, stock: &T) -> bool {
            let price = stock.get_sell_price();
            if price < self.min.price{
                self.min.price = price;
                self.min.time = stock.get_time();
                return true
            }
            else if price > self.max.price{
                self.max.price = price;
                self.max.time = stock.get_time();
                return true
            }
            false
        }
        fn will_wipeout(&self, current_stock: &T) -> bool {
            return self.will_wipeout_(current_stock.get_sell_price())
        }
        fn will_cashout(&self, current_stock: &T) -> bool{
            self.will_wipeout(current_stock) || self.stop_loss > current_stock.get_sell_price()
        }
        fn cashout(&self, current_stock: &T) -> NotNaN<f32> {
            let total_borrowed = self.invested * (self.leverage - 1.0);
            return  (current_stock.get_sell_price() * self.unit) - total_borrowed;
        }
        fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StopLossFailure> {
            if self.will_wipeout_(amount) {
                return Err(StopLossFailure::UnderWipeout);
            } else {
                self.stop_loss = amount;
            }

            return Ok(());
        }
        fn get_max(&self) -> &StockTimeSnapshot {
            &self.max
        }
        fn get_min(&self) -> &StockTimeSnapshot {
            &self.min
        }

    }
}
