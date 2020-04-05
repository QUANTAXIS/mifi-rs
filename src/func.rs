use crate::mifi::{Mifi, FullData, Handler};
use std::collections::BTreeMap;

/// 从DZH中导入数据
/// 确保输入的是一个vector
/// 注意T是一个泛型， 应该是单条数据实现
pub fn from_dzh<T>(data: Vec<T>, frq: f64, tp: String, market: String) -> Mifi<T>
    where T: Handler
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
    if tp.as_str() == "real" {
        // 在实时数据的情况下直接插入第一条
        mifi.real = data
    } else if tp.as_str() == "history"{
        // 插入历史数据
        for i in data {
            mifi.history.insert(i.get_datetime(), i);
        }
    } else {
        panic!("传入了错误的数据类型")
    }
    mifi
}





