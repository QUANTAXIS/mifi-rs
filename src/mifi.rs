use std::clone::Clone;
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::base::*;
use crate::front::{HqTrend, HqTrendSlice};

/// 此行情协议主要是为沟通多个行情之间的数据然后进行交换切割， 注意T是某一类数据， 比如k线数据，
/// topic： 主题
/// frq: 周期
/// market:市场
/// history: 历史数据,注意是datetime作为key, 数据作为value，逆序
/// portfolio: 组合信息,比如指数
/// format: 格式成指定数据源
/// real: 实时行情
/// zip:是否压缩发送
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Mifi<T> {
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
    where
        T: Handler + Serialize,
{
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn hqchart_trend(&self) -> HqTrend {
        let slice: Vec<HqTrendSlice> = self.history.values().into_iter().map(|x| {
            x.to_hqchart_trend_slice()
        }).collect();
        let code = slice[0].code.clone();
        let time = slice[0].get_datetime();

        HqTrend {
            name: "".to_string(),
            symbol: code,
            time: slice.last().unwrap().get_datetime(),
            date: time,
            price: 0.0,
            yclose: slice[0].open,
            open: slice.last().unwrap().open,
            high: slice.last().unwrap().high,
            low: slice.last().unwrap().low,
            vol: slice.last().unwrap().vol,
            amount: slice.last().unwrap().amount,
            minutecount: slice.len().clone() as f64,
            minute: slice,

        }
    }
}
