use ordered_float::NotNaN;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use chrono::DateTime;
use crate::stock_action::StockAction::{Stock, StockInvestment};

#[derive(Debug, Deserialize)]
    pub  struct SAndPHistoricalDailyRaw {
    Date: String,
    Open: f32,
    High: f32,
    Low: f32,
    Close: f32,
}

pub  struct SAndPHistoricalDaily{
    date: String,
    open: NotNaN<f32>,
    high: NotNaN<f32>,
    low: NotNaN<f32>,
    close: NotNaN<f32>,
}

pub enum SandPConversionError{
    ParseError,
    IsNaNError,
}

fn ToNotNaN(f: f32)->Result<NotNaN<f32>, SandPConversionError>{
    return match NotNaN::new(f){
        Ok(f) => Ok(f),
        Err(e) => Err(SandPConversionError::IsNaNError)
    }
}

// TODO: make a derive macro for sandphistoricaldaily?
impl TryInto<SAndPHistoricalDaily> for SAndPHistoricalDailyRaw{
    type Error = SAndPHistoricalDaily;
    fn try_into(self) -> Result<SAndPHistoricalDaily, Self::Error> {
        let date = match DateTime::parse_from_str(&self.Date, "%m/%d/%Y"){
            Ok(s)=>s,
            Err(e) => {eprintln!("failed parse: {:?}", e); return  SandPConversionError::ParseError}
        };
        return Ok(SAndPHistoricalDaily{
            close: ToNotNaN(self.Close)?,
            date,
            high: ToNotNaN(self.High)?,
            low: ToNotNaN(self.Low)?,
            open: ToNotNaN(self.Open)?
        })


    }
}




struct WorstCaseSAndP(SAndPHistoricalDaily);
impl Stock for WorstCaseSAndP{
    fn get_price(&self)->ordered_float::NotNaN<f32> {
        let x = NotNaN::new(self.0.High).expect("NaN supplied to Stock");
    }
    fn get_time(&self)->chrono::Duration {
        let s = & self.0.Date;
        let ans  = DateTime::parse_from_str(s, "%m/%d/%Y");
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
        let ans  = DateTime::parse_from_str(s, "%m/%d/%Y").expect("datetime parsing failed!");
        // s.parse::<Duration>("%Y-%m-%d")
        return ans;
    }
}






