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

impl Default for FullData {
    fn default() -> Self {
        FullData {
            amount: 0.0,
            close: 0.0,
            code: "".to_string(),
            high: 0.0,
            low: 0.0,
            market: "".to_string(),
            open: 0.0,
            productid: 0.0,
            tickcount: 0.0,
            time: "".to_string(),
            vol: 0.0,
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Full {
    pub MarketFullName: String,
    pub products: Vec<FullData>,
}

/// ctpx提供的数据源
#[derive(Serialize, Deserialize, Debug)]
pub struct CtpPro {
    ask_price_1: f64,
    ask_price_2: f64,
    ask_price_3: f64,
    ask_price_4: f64,
    ask_price_5: f64,
    ask_volume_1: f64,
    ask_volume_2: f64,
    ask_volume_3: f64,
    ask_volume_4: f64,
    ask_volume_5: f64,
    average_price: f64,
    bid_price_1: f64,
    bid_price_2: f64,
    bid_price_3: f64,
    bid_price_4: f64,
    bid_price_5: f64,
    bid_volume_1: f64,
    bid_volume_2: f64,
    bid_volume_3: f64,
    bid_volume_4: f64,
    bid_volume_5: f64,
    datetime: String,
    exchange: String,
    gateway_name: String,
    high_price: f64,
    last_price: f64,
    last_volume: f64,
    limit_down: f64,
    limit_up: f64,
    local_symbol: String,
    low_price: f64,
    name: String,
    open_interest: f64,
    open_price: f64,
    preSettlementPrice: f64,
    pre_close: f64,
    symbol: String,
    volume: f64,
}

impl Handler for CtpPro {
    fn get_datetime(&self) -> String {
        return self.datetime.clone();
    }
}

impl Default for CtpPro {
    fn default() -> Self {
        CtpPro {
            ask_price_1: 0.0,
            ask_price_2: 0.0,
            ask_price_3: 0.0,
            ask_price_4: 0.0,
            ask_price_5: 0.0,
            ask_volume_1: 0.0,
            ask_volume_2: 0.0,
            ask_volume_3: 0.0,
            ask_volume_4: 0.0,
            ask_volume_5: 0.0,
            average_price: 0.0,
            bid_price_1: 0.0,
            bid_price_2: 0.0,
            bid_price_3: 0.0,
            bid_price_4: 0.0,
            bid_price_5: 0.0,
            bid_volume_1: 0.0,
            bid_volume_2: 0.0,
            bid_volume_3: 0.0,
            bid_volume_4: 0.0,
            bid_volume_5: 0.0,
            datetime: "".to_string(),
            exchange: "".to_string(),
            gateway_name: "".to_string(),
            high_price: 0.0,
            last_price: 0.0,
            last_volume: 0.0,
            limit_down: 0.0,
            limit_up: 0.0,
            local_symbol: "".to_string(),
            low_price: 0.0,
            name: "".to_string(),
            open_interest: 0.0,
            open_price: 0.0,
            preSettlementPrice: 0.0,
            pre_close: 0.0,
            symbol: "".to_string(),
            volume: 0.0,
        }
    }
}



