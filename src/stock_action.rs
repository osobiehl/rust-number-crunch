pub mod StockAction {
    use chrono::Duration;
    use ordered_float::NotNaN;
    pub enum StockActionFailure {
        UnderWipeout,
        Other,
    }

    // at some point: expand to use spread
    pub trait Stock {
        fn get_time(&self) -> Duration;
        fn get_price(&self) -> NotNaN<f32>;
    }
    #[non_exhaustive]
    pub enum StopLossFailure {
        UnderWipeout,
        WillImmediatelyCashOut,
    }
    pub trait StockAction<T: BuyableSecurity> {
        fn from(stock: & T, currency_invested: NotNaN<f32>) -> Self;
        fn will_wipeout(&self, current_stock: &T) -> bool;
        fn will_cashout(&self, current_stock: &T) -> bool;
        fn cashout(self, current_stock: &T) -> NotNaN<f32>;
        fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StopLossFailure>;
    }
    pub trait BuyableSecurity {
        fn get_price(&self) -> NotNaN<f32>;
    }
    impl<T: Stock> BuyableSecurity for T {
        fn get_price(&self) -> NotNaN<f32> {
            return self::Stock::get_price(&self);
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct StockInvestment {
        stop_loss: NotNaN<f32>,
        unit: NotNaN<f32>,
        invested: NotNaN<f32>,
        invested_at: u64,
        wipeout: NotNaN<f32>,
        leverage: NotNaN<f32>,
    }
    impl StockInvestment{
        fn will_wipeout_(&self, price: NotNaN<f32>)->bool{
             return self.wipeout > price;
        }
    }

    impl<T: BuyableSecurity> StockAction<T> for StockInvestment
    {
        fn from(
            stock: T,
            currency_invested: NotNaN<f32>,
            leverage: NotNaN<f32>,
        ) -> Self {
            assert!(leverage > 0.0, "0 leverage given!");
            return Self {
                stop_loss: currency_invested - (currency_invested / leverage),
                unit: currency_invested / stock.get_price(),
                invested: currency_invested,
                invested_at: stock.get_time(),
                wipeout: currency_invested - (currency_invested / leverage),
                leverage,
            };
        }
        fn will_wipeout(&self, current_stock: &T) -> bool {
            return self.will_wipeout_(current_stock.get_price())
        }
        fn will_cashout(&self, current_stock: &T) -> bool{
            self.will_wipeout(current_stock) || self.stop_loss < current_stock.get_price()
        }
        fn cashout(&self, current_stock: &T) -> NotNaN<f32> {
            return self.invested
                + (current_stock.get_price() - self.invested) * NotNaN::from(self.leverage as f32);
        }
        fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StopLossFailure> {
            if self.will_wipeout_(amount) {
                return Err(StopLossFailure::UnderWipeout);
            } else {
                self.stop_loss = amount;
            }

            return Ok(());
        }
    }
}
