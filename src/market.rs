use serde::{Deserialize, Serialize};

use crate::base::Handler;

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

    fn get_code(&self) -> String {
        unimplemented!()
    }

    fn get_date(&self) -> String {
        unimplemented!()
    }

    fn get_open(&self) -> f64 {
        unimplemented!()
    }

    fn get_close(&self) -> f64 {
        unimplemented!()
    }

    fn get_high(&self) -> f64 {
        unimplemented!()
    }

    fn get_low(&self) -> f64 {
        unimplemented!()
    }

    fn get_vol(&self) -> f64 {
        unimplemented!()
    }

    fn get_amount(&self) -> f64 {
        unimplemented!()
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
    pub ask_price_1: f64,
    pub ask_price_2: f64,
    pub ask_price_3: f64,
    pub ask_price_4: f64,
    pub ask_price_5: f64,
    pub ask_volume_1: f64,
    pub ask_volume_2: f64,
    pub ask_volume_3: f64,
    pub ask_volume_4: f64,
    pub ask_volume_5: f64,
    pub average_price: f64,
    pub bid_price_1: f64,
    pub bid_price_2: f64,
    pub bid_price_3: f64,
    pub bid_price_4: f64,
    pub bid_price_5: f64,
    pub bid_volume_1: f64,
    pub bid_volume_2: f64,
    pub bid_volume_3: f64,
    pub bid_volume_4: f64,
    pub bid_volume_5: f64,
    pub datetime: String,
    pub exchange: String,
    pub gateway_name: String,
    pub high_price: f64,
    pub last_price: f64,
    pub last_volume: f64,
    pub limit_down: f64,
    pub limit_up: f64,
    pub local_symbol: String,
    pub low_price: f64,
    pub name: String,
    pub open_interest: f64,
    pub open_price: f64,
    pub preSettlementPrice: f64,
    pub pre_close: f64,
    pub symbol: String,
    pub volume: f64,
}

impl Handler for CtpPro {
    fn get_datetime(&self) -> String {
        return self.datetime.clone();
    }

    fn get_code(&self) -> String {
        self.symbol.clone().to_string()
    }

    fn get_date(&self) -> String {
        self.datetime[0..9].parse().unwrap()
    }

    fn get_open(&self) -> f64 {
        unimplemented!()
    }

    fn get_close(&self) -> f64 {
        unimplemented!()
    }

    fn get_high(&self) -> f64 {
        unimplemented!()
    }

    fn get_low(&self) -> f64 {
        unimplemented!()
    }

    fn get_vol(&self) -> f64 {
        unimplemented!()
    }

    fn get_amount(&self) -> f64 {
        unimplemented!()
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


#[derive(Serialize, Deserialize, Debug)]
pub struct StockDay {
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
pub struct StockMin {
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
pub struct FutureDay {
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
pub struct FutureMin {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde(rename = "trade")]
    pub volume: f64,
    pub date: String,
    pub datetime: String,
    pub code: String,
    #[serde(rename = "type")]
    pub frequence: String,
    pub position: f64,
    pub amount: f64,
    pub tradetime: String,
}

impl Handler for FutureMin {
    fn get_datetime(&self) -> String {
        self.datetime.clone()
    }

    fn get_code(&self) -> String {
        self.code.clone()
    }

    fn get_date(&self) -> String {
        self.date.clone()
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
        self.volume.clone()
    }

    fn get_amount(&self) -> f64 {
        self.amount.clone()
    }
}