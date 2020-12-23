
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use serde_json::value::Value;

use crate::dataframe;
use crate::dataframe::DataCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAKlineBase {
    pub datetime: String,
    pub updatetime: String,
    pub code: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub frequence: String,
    pub pctchange: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAAskBidBase {
    pub BuyPrices: Vec<f64>,
    pub BuyVols: Vec<i32>,
    pub SellPrices: Vec<f64>,
    pub SellVols: Vec<i32>,

    pub code: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub amount: i64,
    pub productid: i32,
    pub tickcount: i32,
    pub time: String,
    pub vol: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QASeries {
    pub min1: QAKlineBase,
    pub min5: QAKlineBase,
    pub min15: QAKlineBase,
    pub min30: QAKlineBase,
    pub min60: QAKlineBase,
    pub day: QAKlineBase,
    dtmin: String,
    //dtmin是用于控制分钟线生成的
    cached: Vec<QAKlineBase>
}

impl QAAskBidBase {
    pub fn new() -> QAAskBidBase {
        QAAskBidBase {
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
            code: "".to_string(),
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            amount: 0,
            productid: 0,
            tickcount: 0,
            time: "".to_string(),
            vol: 0,
        }
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}

impl QAKlineBase {
    pub fn init() -> QAKlineBase {
        QAKlineBase {
            datetime: "".to_string(),
            updatetime: "".to_string(),
            code: "".to_string(),
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
            frequence: "1min".to_string(),
            pctchange: 0.0
        }
    }

    pub fn update(&mut self, data: Value) {
        if self.open == 0.0 {
            self.init_data(data.clone());
        }
        let new_price = data["close"].as_f64().unwrap();
        if (self.high < new_price) {
            self.high = new_price;
        }
        if (self.low > new_price) {
            self.low = new_price;
        }
        self.close = new_price;
        self.volume += data["vol"].as_f64().unwrap();
        let cur_datetime: String = data["time"].as_str().unwrap().parse().unwrap();
        self.updatetime = cur_datetime.clone();
        self.pctchange = (new_price/ self.open.clone()) - 1.0
    }

    pub fn init_data(&mut self, data: Value) {
        self.datetime = data["time"].as_str().unwrap().parse().unwrap();
        self.updatetime = data["time"].as_str().unwrap().parse().unwrap();
        self.code = data["code"].as_str().unwrap().parse().unwrap();
        self.open = data["close"].as_f64().unwrap();
        self.high = data["close"].as_f64().unwrap();
        self.low = data["close"].as_f64().unwrap();
        self.close = data["close"].as_f64().unwrap();
        self.volume = data["vol"].as_f64().unwrap();
    }

    pub fn print(&mut self) {
        println!(
            "\n\r {:?}\n {:?}-{:?}-{:?}-{:?}-{:?}-{:#?}\r",
                self.datetime.clone(),
                self.code.clone(),
                self.open.clone(),
                self.high.clone(),
                self.low.clone(),
                self.close.clone(),
                self.volume.clone()
        );
    }

    pub fn new(&mut self, data: Value, frequence: String) -> QAKlineBase {
        let data = QAKlineBase {
            datetime: data["time"].as_str().unwrap().parse().unwrap(),
            updatetime: data["time"].as_str().unwrap().parse().unwrap(),
            code: data["code"].as_str().unwrap().parse().unwrap(),
            open: data["close"].as_f64().unwrap(),
            high: data["close"].as_f64().unwrap(),
            low: data["close"].as_f64().unwrap(),
            close: data["close"].as_f64().unwrap(),
            volume: data["vol"].as_f64().unwrap(),
            frequence: frequence,
            pctchange: 0.0
        };
        data
    }
    pub fn to_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self).unwrap();
        //println!("this is json{:#?}", jdata);
        jdata
    }
}

impl QASeries {
    pub fn init() -> QASeries {
        QASeries {
            min1: QAKlineBase {
                datetime: "".to_string(),
                updatetime: "".to_string(),
                code: "".to_string(),
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
                frequence: "1min".to_string(),
                pctchange: 0.0
            },
            min5: QAKlineBase {
                datetime: "".to_string(),
                updatetime: "".to_string(),
                code: "".to_string(),
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
                frequence: "5min".to_string(),
                pctchange: 0.0
            },
            min15: QAKlineBase {
                datetime: "".to_string(),
                updatetime: "".to_string(),
                code: "".to_string(),
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
                frequence: "15min".to_string(),
                pctchange: 0.0
            },
            min30: QAKlineBase {
                datetime: "".to_string(),
                updatetime: "".to_string(),
                code: "".to_string(),
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
                frequence: "30min".to_string(),
                pctchange: 0.0
            },
            min60: QAKlineBase {
                datetime: "".to_string(),
                updatetime: "".to_string(),
                code: "".to_string(),
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
                frequence: "60min".to_string(),
                pctchange: 0.0
            },
            day: QAKlineBase {
                datetime: "".to_string(),
                updatetime: "".to_string(),
                code: "".to_string(),
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
                frequence: "day".to_string(),
                pctchange: 0.0
            },
            cached: vec![],
            dtmin: "".to_string(),
        }
    }
    pub fn on_data(&mut self, data: Value) {
        self.update(data);
    }

    pub fn update(&mut self, data: Value) {
        let cur_data = data.clone();

        //println!("{:#?}", cur_data);

        let cur_datetime: String = cur_data["time"].as_str().unwrap().parse().unwrap();
        if self.dtmin == "99".to_string() {
            self.init_data(cur_data.clone());
            self.dtmin = cur_datetime[14..16].parse().unwrap();
        }

        //切换分钟线的时候
        if &cur_datetime[14..16] != self.dtmin {
            let min_f = &cur_datetime[14..16];
            match min_f {
                "00" => {
                    self.update_1(cur_data.clone(), cur_datetime.clone());
                    self.update_5(cur_data.clone(), cur_datetime.clone());
                    self.update_15(cur_data.clone(), cur_datetime.clone());
                    self.update_30(cur_data.clone(), cur_datetime.clone());
                    self.update_stock60(cur_data.clone(), cur_datetime.clone());
                }
                "30" => {
                    self.update_1(cur_data.clone(), cur_datetime.clone());
                    self.update_5(cur_data.clone(), cur_datetime.clone());
                    self.update_15(cur_data.clone(), cur_datetime.clone());
                    self.update_30(cur_data.clone(), cur_datetime.clone());
                    //self.min60.update(data.clone());
                    self.update_stock60(cur_data.clone(), cur_datetime.clone());
                }
                "15" | "45" => {
                    self.update_1(cur_data.clone(), cur_datetime.clone());
                    self.update_5(cur_data.clone(), cur_datetime.clone());
                    self.update_15(cur_data.clone(), cur_datetime.clone());
                    self.min30.update(data.clone());
                    self.min60.update(data.clone());
                }
                "05" | "10" | "20" | "25" | "35" | "40" | "50" | "55" => {
                    self.update_1(cur_data.clone(), cur_datetime.clone());
                    self.update_5(cur_data.clone(), cur_datetime.clone());
                    self.min15.update(data.clone());
                    self.min30.update(data.clone());
                    self.min60.update(data.clone());
                }
                _ => {
                    self.update_1(cur_data.clone(), cur_datetime.clone());
                    self.min5.update(data.clone());                                                                                                 
                    self.min15.update(data.clone());
                    self.min30.update(data.clone());
                    self.min60.update(data.clone());
                }
            }
        } else {
            self.min1.update(data.clone());
            self.min5.update(data.clone());
            self.min15.update(data.clone());
            self.min30.update(data.clone());
            self.min60.update(data.clone());
        }
        self.day.update(data.clone());
        self.dtmin = cur_datetime[14..16].parse().unwrap();
        //println!("dtmin {:?}", self.dtmin);
    }

    pub fn update_1(&mut self, data: Value, cur_datetime: String) {
        self.cached.push(self.min1.clone());
        self.min1 = QAKlineBase::init().new(data.clone(), "1min".to_string());
    }

    pub fn update_5(&mut self, data: Value, cur_datetime: String) {
        self.cached.push(self.min5.clone());
        self.min5 = QAKlineBase::init().new(data.clone(), "5min".to_string());
    }
    pub fn update_15(&mut self, data: Value, cur_datetime: String) {
        self.cached.push(self.min15.clone());
        self.min15 = QAKlineBase::init().new(data.clone(), "15min".to_string());
    }
    pub fn update_30(&mut self, data: Value, cur_datetime: String) {
        self.cached.push(self.min30.clone());
        self.min30 = QAKlineBase::init().new(data.clone(), "30min".to_string());
    }
    // pub fn update_60(&mut self, data: Value, cur_datetime: String) {
    //     self.cached.push(self.min60.clone());
    //     self.min60 = QAKlineBase::init().new(data.clone(), "60min".to_string());
    // }

    pub fn update_stock60(&mut self, data: Value, cur_datetime: String) {
        let hour = &cur_datetime[11..13];
        let hour_i32 = hour.parse::<i32>().unwrap();
        let minute= &cur_datetime[14..16];

        if hour_i32<12{
            if minute == "30"{
                self.cached.push(self.min60.clone());
                self.min60 = QAKlineBase::init().new(data.clone(), "60min".to_string());
            }else{
                self.min60.update(data.clone());
            }
        }else{
            if minute == "00"{
                self.cached.push(self.min60.clone());
                self.min60 = QAKlineBase::init().new(data.clone(), "60min".to_string());
            }else{
                self.min60.update(data.clone());
            }
        }

    }
    pub fn print(&mut self) {
        print!("MIN1 \n\r{:?}\n\r", self.min1);
        print!("MIN5 \n\r{:?}\n\r", self.min5);
        print!("MIN15 \n\r{:?}\n\r", self.min15);
        print!("MIN30 \n\r{:?}\n\r", self.min30);
        print!("MIN60 \n\r{:?}\n\r", self.min60);
    }
    pub fn to_json(&mut self) -> (String, String) {
        let jdata = serde_json::to_string(&self).unwrap();
        //println!("\nthis is json{:#?}\n", jdata);

        let cache = self.clear_cache();
        (jdata, cache)
    }
    pub fn clear_cache(&mut self) -> String {
        let cached = serde_json::to_string(&self.cached).unwrap();
        self.cached = vec![];
        if cached.is_empty() {
            "".to_string()
        } else {
            cached
        }
    }
    pub fn to_1min_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self.min1).unwrap();
        jdata
    }
    pub fn to_5min_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self.min5).unwrap();
        jdata
    }
    pub fn to_15min_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self.min15).unwrap();
        jdata
    }
    pub fn to_30min_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self.min30).unwrap();
        jdata
    }
    pub fn to_60min_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self.min60).unwrap();
        jdata
    }

    pub fn to_day_json(&mut self) -> String {
        let jdata = serde_json::to_string(&self.day).unwrap();
        jdata
    }


    pub fn init_data(&mut self, data: Value) {
        self.min1 = QAKlineBase::init().new(data.clone(), "1min".to_string());
        self.min5 = QAKlineBase::init().new(data.clone(), "5min".to_string());
        self.min15 = QAKlineBase::init().new(data.clone(), "15min".to_string());
        self.min30 = QAKlineBase::init().new(data.clone(), "30min".to_string());
        self.min60 = QAKlineBase::init().new(data.clone(), "60min".to_string());
        self.day = QAKlineBase::init().new(data.clone(), "day".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_QAASKBID() {
        let askbid = QAAskBidBase::new();
        println!("{:#?}", askbid.to_json())
    }

    #[test]
    fn test_update() {
        let mut new_data = QAAskBidBase {
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
            code: "000001".to_string(),
            open: 20.0,
            high: 24.0,
            low: 19.0,
            close: 21.0,
            amount: 10000,
            productid: 110,
            tickcount: 1120,
            time: "2020-01-20 10:20:02".to_string(),
            vol: 110,
        };

        let mut qseries = QASeries::init();

        qseries.init_data(serde_json::to_value(new_data).unwrap());

        //println!("{:#?}",qseries.to_json());

        let res = "{\"min1\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"1min\",\"pctchange\":0.0},\"min5\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"5min\",\"pctchange\":0.0},\"min15\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"15min\",\"pctchange\":0.0},\"min30\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"30min\",\"pctchange\":0.0},\"min60\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"60min\",\"pctchange\":0.0},\"day\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"day\",\"pctchange\":0.0},\"dtmin\":\"\",\"cached\":[]}";

        let (res1, fix) = qseries.to_json();
        assert_eq!(res, res1);


        new_data = QAAskBidBase {
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
            code: "000001".to_string(),
            open: 22.0,
            high: 26.0,
            low: 21.0,
            close: 24.0,
            amount: 3000,
            productid: 110,
            tickcount: 1120,
            time: "2020-01-20 10:20:09".to_string(),
            vol: 130,
        };

        qseries.update(serde_json::to_value(new_data).unwrap());


        let res = "{\"min1\":{\"datetime\":\"2020-01-20 10:20:09\",\"updatetime\":\"2020-01-20 10:20:09\",\"code\":\"000001\",\"open\":24.0,\"high\":24.0,\"low\":24.0,\"close\":24.0,\"volume\":130.0,\"frequence\":\"1min\",\"pctchange\":0.0},\"min5\":{\"datetime\":\"2020-01-20 10:20:09\",\"updatetime\":\"2020-01-20 10:20:09\",\"code\":\"000001\",\"open\":24.0,\"high\":24.0,\"low\":24.0,\"close\":24.0,\"volume\":130.0,\"frequence\":\"5min\",\"pctchange\":0.0},\"min15\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:09\",\"code\":\"000001\",\"open\":21.0,\"high\":24.0,\"low\":21.0,\"close\":24.0,\"volume\":240.0,\"frequence\":\"15min\",\"pctchange\":0.1428571428571428},\"min30\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:09\",\"code\":\"000001\",\"open\":21.0,\"high\":24.0,\"low\":21.0,\"close\":24.0,\"volume\":240.0,\"frequence\":\"30min\",\"pctchange\":0.1428571428571428},\"min60\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:09\",\"code\":\"000001\",\"open\":21.0,\"high\":24.0,\"low\":21.0,\"close\":24.0,\"volume\":240.0,\"frequence\":\"60min\",\"pctchange\":0.1428571428571428},\"day\":{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:09\",\"code\":\"000001\",\"open\":21.0,\"high\":24.0,\"low\":21.0,\"close\":24.0,\"volume\":240.0,\"frequence\":\"day\",\"pctchange\":0.1428571428571428},\"dtmin\":\"20\",\"cached\":[{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"1min\",\"pctchange\":0.0},{\"datetime\":\"2020-01-20 10:20:02\",\"updatetime\":\"2020-01-20 10:20:02\",\"code\":\"000001\",\"open\":21.0,\"high\":21.0,\"low\":21.0,\"close\":21.0,\"volume\":110.0,\"frequence\":\"5min\",\"pctchange\":0.0}]}";
        let (res1, fix) = qseries.to_json();
        assert_eq!(res, res1);
    }
}