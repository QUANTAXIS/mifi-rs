/// 主要用于前端行情接口测试
use mifi_rs::front::{Kline, HqTrendSlice, HqTrend};
use mifi_rs::mifi::Mifi as Mi;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use mifi_rs::func::from_history;

/// 测试转换为hqchart的分时线
#[test]
fn hqchart_convert() {
    let x = Kline::default();
    assert_eq!(x.datetime, "1900-01-01 00:00:00.1");
    println!("{:?}", x);
    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        data: Vec<Kline>
    }
    let strings = r#"{"data":[
    { "code": "rb2010", "datetime": "2020-04-19 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 },
    { "code": "rb2010", "datetime": "2020-04-20 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 },
    { "code": "rb2010", "datetime": "2020-04-22 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 },
    { "code": "rb2010", "datetime": "2020-04-25 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 },
    { "code": "rb2010", "datetime": "2020-04-26 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 },
    { "code": "rb2010", "datetime": "2020-04-27 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 },
    { "code": "rb2010", "datetime": "2020-04-28 00:00:00.1", "open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "vol": 3300.0 }
    ]}"#;
    let c: TestStruct = serde_json::from_str(strings).unwrap();
    assert_eq!(c.data.len(), 7);
    let mifi_rs = from_history(c.data.clone(), 60.0, "future".to_string());
    println!("mifi_rs hqchart: {:?}", mifi_rs.hqchart_trend());
    assert_eq!(mifi_rs.hqchart_trend().time, "2020-04-28 00:00:00.1");
}


