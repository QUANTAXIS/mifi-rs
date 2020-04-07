use serde::{Deserialize, Serialize};

use crate::front::HqTrendSlice;

pub trait Handler {
    fn get_datetime(&self) -> String;

    fn to_json(&self) -> String
        where
            Self: Serialize,
    {
        serde_json::to_string(&self).unwrap()
    }

    fn get_code(&self) -> String;
    fn get_date(&self) -> String;
    fn get_open(&self) -> f64;
    fn get_close(&self) -> f64;
    fn get_high(&self) -> f64;
    fn get_low(&self) -> f64;
    fn get_vol(&self) -> f64;
    fn get_amount(&self) -> f64;
    fn to_hqchart_trend_slice(&self) -> HqTrendSlice {
        HqTrendSlice {
            price: self.get_close(),
            open: self.get_open(),
            high: self.get_high(),
            low: self.get_low(),
            vol: self.get_vol(),
            amount: self.get_amount(),
            time: self.get_datetime(),
            avprice: self.get_amount() / self.get_vol(),
            increase: 0.0,
            risefall: 0.0,
            code: self.get_code(),
            close: self.get_close(),
        }
    }
}


