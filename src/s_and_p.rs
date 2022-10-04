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

struct WorstCaseSAndP(SAndPHistoricalDaily);
impl Stock for WorstCaseSAndP{
    fn get_price(&self)->ordered_float::NotNaN<f32> {
        let x = NotNaN::new(self.0.High).expect("NaN supplied to Stock");
    }
    fn get_time(&self)->chrono::Duration {
        let s = & self.0.Date;
        let ans  = DateTime::from_str("%m/%d/%Y");
        // s.parse::<Duration>("%Y-%m-%d")

    }
}
