use serde::{Deserialize, Serialize};
use crate::base::Handler;
use regex::Regex;
use super::market_preset::{MarketPreset};
// use std::intrinsics::forget;
// use std::fs::read_to_string;

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

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct HqKlineFormat {
    pub data: Vec<Vec<f64>>,
    pub symbol: String,
    pub name: String,
    pub end: f64,
    pub start: f64,
    pub count: f64,
    pub ticket: f64,
    pub version: String,
    pub message: String,
    pub code: f64,
    pub servertime: String,
}

impl HqTrend {
    /// 将任意初始的图转换为k线图所需要的格式
    pub fn as_kline_format(&self) -> HqKlineFormat {
        let mut data: Vec<Vec<f64>> = vec![];
        let length = self.minute.len();
        data.push(vec![self.covert_timestr_to_data_f64(self.minute[0].time.as_str()), self.yclose.clone(),
                       self.minute[0].high.clone(), self.minute[0].low.clone(), self.minute[0].close.clone(), self.minute[0].vol, self.minute[0].amount.clone()]);
        // 拿到第一个bar 直接送入最新的收盘价格
        let mut close_pre = self.minute[0].close.clone();
        for c in &self.minute[1..length] {
            let temp = vec![self.covert_timestr_to_data_f64(c.time.as_str()), close_pre.clone(), c.open.clone(), c.high.clone(), c.low.clone(), c.close.clone(), c.vol.clone(), c.amount.clone()];
            data.push(temp);
            close_pre = c.close.clone();
        };
        HqKlineFormat {
            data,
            symbol: self.symbol.clone(),
            name: self.name.clone(),
            end: 0.0,
            start: 0.0,
            count: self.minutecount.clone(),
            ticket: 0.0,
            version: "2".to_string(),
            message: "".to_string(),
            code: 0.0,
            servertime: self.time.clone(),
        }
    }
    pub fn covert_timestr_to_data_f64(&self, timestr: &str) -> f64 {
        let x: Vec<&str> = timestr.split(" ").collect::<Vec<&str>>();
        let tt = x[0].replace("-", "");
        assert_eq!(tt.len(), 8);
        tt.parse::<f64>().unwrap()
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    pub code: String,
    pub name: String,
}

impl Contract {
    pub fn to_symbol(&self) -> String {
        let re = Regex::new(r"[a-zA-z]+").unwrap();
        let mut market = MarketPreset::new();
        let mk = market.get(self.code.as_ref());
        if re.find(self.code.as_str()).is_some() {
            return format!("future_{}_{}", mk.exchange, self.code);
        } else {
            let exchange = if self.code.starts_with("6") { "SH" } else { "SZ" };
            return format!("stock_{}_{}", exchange, self.code);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = r#"{
        "code":"600000",
        "name":"上海浦東發展銀行"
        }"#;
        let n: Contract = serde_json::from_str(x).unwrap();
        println!("{}", n.to_symbol());

        let c = r#"{
        "code":"RB2010",
        "name":"螺纹钢2010"
        }"#;
        let m: Contract = serde_json::from_str(c).unwrap();
        println!("{}", m.to_symbol());
    }
}