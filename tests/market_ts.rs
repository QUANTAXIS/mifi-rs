/// 测试各个级别的k线使用

/// 主要用于前端行情接口测试
use Mifi::market::FutureMin;
use Mifi::mifi::Mifi as Mi;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use Mifi::func::from_history;

/// 测试转换为hqchart kline格式数据
#[test]
fn future_covert_kline() {
    let x = FutureMin::default();
    println!("{:?}", x);
    assert_eq!(x.datetime, "1900-01-01 00:00:00");

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        data: Vec<FutureMin>
    }
    let strings = r#"{"data":[
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-15 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0},
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-16 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0},
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-17 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0},
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-18 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0},
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-19 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0},
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-20 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0},
    { "code": "rb2010","open": 3359.0, "high": 3367.0, "low": 3341.0, "close": 3361.0, "trade": 3300.0, "datetime":"2019-10-21 09:00:00", "date":"2019-10-15", "amount":4564561.0, "type":"1min", "tradetime":"2019-10-15 09:00:00", "position":1500.0}
    ]}"#;
    let c: TestStruct = serde_json::from_str(strings).unwrap();
    assert_eq!(c.data.len(), 7);
    let mifi = from_history(c.data.clone(), 60.0, "future".to_string());
    // println!("mifi: {:?}", mifi);
    let ttx = mifi.hqchart_trend().as_kline_format();
    // println!("mifi hqchart: {:?}", );
    assert_eq!(ttx.data.len(), 7);
}


