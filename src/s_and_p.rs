use std::convert::TryInto;

use chrono::{Duration, DateTime, Date,NaiveDateTime};
use ordered_float::NotNaN;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::stock_action::StockAction::{Stock, StockInvestment};

#[derive(Debug, Deserialize)]
    pub  struct SAndPHistoricalDailyRaw {
    Date: String,
    Open: f32,
    High: f32,
    Low: f32,
    Close: f32,
}
#[derive(Clone, Copy, Debug)]
pub  struct SAndPHistoricalDaily{
    date: DateTime<Utc>,
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
    type Error = SandPConversionError;
    fn try_into(self) -> Result<SAndPHistoricalDaily, Self::Error> {
        let date: DateTime<Utc> = match NaiveDate::parse_from_str(&self.Date, "%m/%d/%Y"){
            Ok(s)=> Utc.from_local_datetime(&s.and_time(NaiveTime::from_hms(0, 0, 0))).unwrap(),
            Err(e) => {eprintln!("failed parse: {:?}", e.to_string()); 
            return  Err(SandPConversionError::ParseError)}
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



#[derive(Clone, Copy, Debug)]
pub struct WorstCaseSAndP(pub SAndPHistoricalDaily);
impl Stock for WorstCaseSAndP{
    fn get_sell_price(&self)->ordered_float::NotNaN<f32> {
        return self.0.low;
    }
    fn get_time(&self)->chrono::DateTime<Utc> {
        self.0.date
    }
    fn get_buy_price(&self) -> NotNaN<f32> {
        return self.0.high;
    }
}
#[derive(Clone, Copy, Debug)]
pub struct BestCaseSAndP(pub SAndPHistoricalDaily);
impl Stock for BestCaseSAndP{
    fn get_sell_price(&self)->ordered_float::NotNaN<f32> {
        return self.0.high;
    }
    fn get_buy_price(&self) -> NotNaN<f32> {
        return self.0.low;
    }
    fn get_time(&self)->chrono::DateTime<Utc> {
        self.0.date
    }
}






