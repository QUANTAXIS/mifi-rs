use serde::{Deserialize, Serialize};

use crate::base::Handler;

#[derive(Deserialize, Serialize, Debug)]
pub struct HqTrendSlice {
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub vol: f64,
    pub amount: f64,
    pub time: String,
    pub avprice: f64,
    pub increase: f64,
    pub risefall: f64,
    pub code: String,
    pub close: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HqTrend {
    pub name: String,
    pub symbol: String,
    pub time: String,
    pub date: String,
    pub price: f64,
    pub open: f64,
    pub yclose: f64,
    pub high: f64,
    pub low: f64,
    pub vol: f64,
    pub amount: f64,
    pub minute: Vec<HqTrendSlice>,
}


impl Handler for HqTrendSlice {
    fn get_datetime(&self) -> String {
        self.time.clone().to_string()
    }

    fn get_code(&self) -> String {
        self.code.to_string()
    }

    fn get_date(&self) -> String {
        unimplemented!()
    }

    fn get_open(&self) -> f64 {
        self.open.clone()
    }

    fn get_close(&self) -> f64 {
        self.close.clone()
    }

    fn get_high(&self) -> f64 {
        self.high.clone()
    }

    fn get_low(&self) -> f64 {
        self.low.clone()
    }

    fn get_vol(&self) -> f64 {
        self.vol.clone()
    }

    fn get_amount(&self) -> f64 {
        self.amount.clone()
    }
}

