use std::collections::BTreeMap;

use crate::base::Handler;
use crate::market::FullData;
use crate::mifi::Mifi;

/// 从DZH中导入数据
/// 确保输入的是一个vector
/// 注意T是一个泛型， 应该是单条数据实现
pub fn from_dzh<T>(data: Vec<T>, frq: f64, market: String) -> Mifi<T>
    where T: Handler + Default
{
    let mut mifi = Mifi {
        topic: "".to_string(),
        frq: frq,
        market: market,
        history: Default::default(),
        portfolio: Default::default(),
        format: "".to_string(),
        real: Default::default(),
        zip: false,
    };
    for i in data {
        mifi.history.insert(i.get_datetime(), i);
    }
    mifi
}

pub fn from_dzh_real<T>(data: T, frq: f64, market: String) -> Mifi<T>
    where T: Handler
{
    Mifi {
        topic: "".to_string(),
        frq: frq,
        market: market,
        history: Default::default(),
        portfolio: Default::default(),
        format: "".to_string(),
        real: data,
        zip: false,
    }
}








