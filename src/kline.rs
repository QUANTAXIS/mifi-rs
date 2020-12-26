use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct stock_day {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde(rename = "vol")]
    pub volume: f64,
    pub amount: f64,
    pub date: String,
    pub code: String,
//    date_stamp : f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct stock_min {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde(rename = "vol")]
    pub volume: f64,
    pub amount: f64,
    pub date: String,
    pub datetime: String,
    pub code: String,
    //    date_stamp : f64,
//    time_stamp : f64,
    #[serde(rename = "type")]
    pub frequence: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct future_day {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde(rename = "trade")]
    pub volume: f64,
    pub date: String,
    pub code: String,
//    date_stamp : f64
}


#[derive(Serialize, Deserialize, Debug)]
pub struct future_min {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde(rename = "trade")]
    pub volume: f64,
    pub date: String,
    pub datetime: String,
    pub code: String,
    //    date_stamp : f64,
//    time_stamp : f64,
    #[serde(rename = "type")]
    pub frequence: String,
}