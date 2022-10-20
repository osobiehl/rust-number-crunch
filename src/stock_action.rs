
pub mod StockAction {
    use chrono::Duration;
    use ordered_float::NotNaN;
    pub enum StockActionFailure {
        UnderWipeout,
        Other,
    }

    // at some point: expand to use spread
    pub trait Stock{
        fn get_time(&self)->Duration;
        fn get_price(&self)->NotNaN<f32>;
    }
    #[non_exhaustive]
    pub enum StopLossFailure{
        UnderWipeout,
        WillImmediatelyCashOut,
    }
    pub trait StockAction {
        type UnderlyingAsset: Stock;
        fn from(stock: & Self::UnderlyingAsset, currency_invested: NotNaN<f32>) -> Self;
        fn will_wipeout(&self, current_price: NotNaN<f32>) -> bool;
        fn will_cashout(&self, current_price: NotNaN<f32>) -> bool;
        fn cashout(&self, current_price: NotNaN<f32>) -> NotNaN<f32>;
        fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StopLossFailure>;

    }
    pub trait BuyableSecurity {
        fn get_price(&self)->NotNaN<f32>;
    }
    impl BuyableSecurity for Stock {
        fn get_price(&self)->NotNaN<f32> {
            return self::Stock::get_price(&self);
        }
    }
    

    

    
}
