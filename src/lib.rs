use mifi::Mifi;

use crate::base::Handler;
use crate::front::{HqTrend, HqTrendSlice};
use crate::func::{from_history, from_real};
use crate::market::{CtpPro, Full, FullData};

/// 描述了相关方法函数
pub mod func;

/// 描述了核心需求
pub mod mifi;

/// 描述了基础方法
pub mod base;

/// 描述了行情信息
pub mod market;

/// 描述了前端格式
pub mod front;

#[cfg(test)]
mod tests {
    use crate::base::Handler;
    use crate::func::from_history;
    use crate::market::FullData;

    #[test]
    fn test_fulldata() {
        let strs = r#"[{ "BuyPrices" : [
        2.35,
        2.34,
        2.33,
        2.32,
        2.31
    ],
    "BuyVols" : [
        7101,
        17489,
        13565,
        9465,
        8739
    ],
    "SellPrices" : [
        2.36,
        2.37,
        2.38,
        2.39,
        2.4
    ],
    "SellVols" : [
        13411,
        6679,
        6200,
        3948,
        6194
    ],
    "amount" : 40592791,
    "close" : 2.36,
    "code" : "002470",
    "high" : 2.36,
    "low" : 2.35,
    "market" : "SZ",
    "open" : 2.31,
    "productid" : 981,
    "tickcount" : 4210,
    "time" : "2020/2/5 10:17:57",
    "vol" : 173782,
    "datetime" : "2020-02-05 10:17:57"}]"#;
        let val: Vec<FullData> = serde_json::from_str(strs).unwrap();
        println!("{}", val[0].to_json());
        let v = from_history(val, 1 as f64, "stock".to_string());
        println!("v: {:?}", v.to_json());
        println!("v: {:?}", v.clone())
    }
}

