use serde::{Deserialize, Serialize};
use crate::base::Handler;

#[derive(Deserialize, Clone, Serialize, Debug)]
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

#[derive(Deserialize, Clone, Serialize, Debug)]
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
    pub minutecount: f64,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Kline {
    pub code: String,
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub vol: f64,
}

impl Clone for Kline {
    fn clone(&self) -> Self {
        Kline {
            code: self.code.clone(),
            datetime: self.datetime.clone(),
            open: self.open.clone(),
            high: self.high.clone(),
            low: self.low.clone(),
            close: self.close.clone(),
            vol: self.vol.clone(),
        }
    }
}

impl Default for Kline {
    fn default() -> Self {
        Kline {
            code: "".to_string(),
            datetime: "1900-01-01 00:00:00.1".to_string(),
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            vol: 0.0,
        }
    }
}

impl Handler for Kline {
    fn get_datetime(&self) -> String {
        self.datetime.clone()
    }

    fn get_code(&self) -> String {
        self.code.clone()
    }

    fn get_date(&self) -> String {
        self.datetime.clone()
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
        0 as f64
    }

    fn set_datetime(&mut self, datetime: String) {
        self.datetime = datetime;
    }

    fn set_code(&mut self, code: String) {
        self.code = code;
    }

    fn set_open(&mut self, open: f64) {
        self.open = open;
    }

    fn set_close(&mut self, close: f64) {
        self.close = close;
    }

    fn set_high(&mut self, high: f64) {
        self.high = high;
    }

    fn set_low(&mut self, low: f64) {
        self.low = low
    }

    fn set_vol(&mut self, vol: f64) {
        self.vol = vol
    }
}

