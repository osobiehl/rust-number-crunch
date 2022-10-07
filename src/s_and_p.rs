use ordered_float::NotNaN;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::stock_action::StockAction::{Stock, StockAction};
#[derive(Debug, Deserialize)]
    pub  struct SAndPHistoricalDaily {
    Date: String,
    Open: f32,
    High: f32,
    Low: f32,
    Close: f32,
}


trait SAndPStock{}

struct WorstCaseSAndP(SAndPHistoricalDaily);
impl Stock for WorstCaseSAndP{
    fn get_price(&self)->ordered_float::NotNaN<f32> {
        let x = NotNaN::new(self.0.High).expect("NaN supplied to Stock");
    }
    fn get_time(&self)->chrono::Duration {
        let s = & self.0.Date;
        let ans  = DateTime::from_str("%m/%d/%Y");
        // s.parse::<Duration>("%Y-%m-%d")
        return ans;
    }
}

struct BestCaseSAndP(SAndPHistoricalDaily);
impl Stock for BestCaseSAndP{
    fn get_price(&self)->ordered_float::NotNaN<f32> {
        let x = NotNaN::new(self.0.Low).expect("NaN supplied to Stock");
    }
    fn get_time(&self)->chrono::Duration {
        let s = & self.0.Date;
        let ans  = DateTime::from_str("%m/%d/%Y");
        // s.parse::<Duration>("%Y-%m-%d")
        return ans;
    }
}

impl SAndPStock for WorstCaseSAndP{}

impl<T> StockAction<T> for SAndPLeverageBuy
where T: SAndPStock{
    type UnderlyingAsset = SAndPHistoricalDaily;
    fn from(stock: Self::UnderlyingAsset)->Self{

        return StockBuy { stop_loss: (),
             unit: (),
             invested: (), 
             invested_at: (), 
             wipeout: (), 
             leverage: () }
    }
    fn will_wipeout(&self, current_price: NotNaN<f32>) -> bool {
        return self.wipeout > current_price;
    }
    fn will_cashout(&self, current_price: NotNaN<f32>) -> bool {
        self.will_wipeout(current_price) || self.stop_loss < current_price
    }
    fn cashout(&self, current_price: NotNaN<f32>) -> NotNaN<f32> {
        return self.invested
            + (current_price - self.invested) * NotNaN::from(self.leverage as f32);
    }
    fn set_stop_loss(&mut self, amount: NotNaN<f32>) -> Result<(), StockActionFailure> {
        if self.will_wipeout(amount) {
            return Err(StockActionFailure::UnderWipeout);
        } else {
            self.stop_loss = amount;
        }

        return Ok(());
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SAndPLeverageBuy {
    stop_loss: NotNaN<f32>,
    unit: NotNaN<f32>,
    invested: NotNaN<f32>,
    invested_at: u64,
    wipeout: NotNaN<f32>,
    leverage: u32,
}
