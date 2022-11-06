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
        
        let date: DateTime<Utc> = match NaiveDateTime::parse_from_str(&self.Date, "%m/%d/%Y"){
            Ok(s)=>s.try_into().expect("failed naivedatetime parse"),
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




pub struct WorstCaseSAndP(pub SAndPHistoricalDaily);
impl Stock for WorstCaseSAndP{
    fn get_price(&self)->ordered_float::NotNaN<f32> {
        return self.0.high;
    }
    fn get_time(&self)->chrono::DateTime<Utc> {
        self.0.date
    }
}

pub struct BestCaseSAndP(pub SAndPHistoricalDaily);
impl Stock for BestCaseSAndP{
    fn get_price(&self)->ordered_float::NotNaN<f32> {
        return self.0.low;
    }
    fn get_time(&self)->chrono::DateTime<Utc> {
        self.0.date
    }
}






