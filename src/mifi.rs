use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

pub trait Handler {
    fn get_datetime(&self) -> String;

    fn to_json(&self) -> String
        where Self: Serialize {
        serde_json::to_string(&self).unwrap()
    }
}

/// 此行情协议主要是为沟通多个行情之间的数据然后进行交换切割， 注意T是某一类数据， 比如k线数据，
/// topic： 主题
/// frq: 周期
/// market:市场
/// history: 历史数据,注意是datetime作为key, 数据作为value，逆序
/// portfolio: 组合信息,比如指数
/// format: 格式成指定数据源
/// real: 实时行情
/// zip:是否压缩发送
#[derive(Serialize, Deserialize, Debug)]
pub struct Mifi<T>
{
    pub topic: String,
    pub frq: f64,
    pub market: String,
    pub history: BTreeMap<String, T>,
    pub portfolio: HashMap<String, T>,
    pub format: String,
    pub real: T,
    pub zip: bool,
}

impl<T> Mifi<T>
    where T: Handler + Serialize
{
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FullData {
    pub amount: f64,
    pub close: f64,
    pub code: String,
    pub high: f64,
    pub low: f64,
    pub market: String,
    pub open: f64,
    pub productid: f64,
    pub tickcount: f64,
    pub time: String,
    pub vol: f64,
    pub BuyPrices: Vec<f64>,
    pub BuyVols: Vec<f64>,
    pub SellPrices: Vec<f64>,
    pub SellVols: Vec<f64>,
}

impl Handler for FullData {
    fn get_datetime(&self) -> String {
        return self.time.clone();
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Full {
    pub MarketFullName: String,
    pub products: Vec<FullData>,
}

/// ctpx提供的数据源
#[derive(Serialize, Deserialize, Debug)]
pub struct Ctpx {}


