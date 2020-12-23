use serde_json::Value;
use std::collections::HashMap;

use crate::qafastkline;
use crate::qafastkline::{QAAskBidBase, QAKlineBase, QASeries};

pub struct marketcollection {
    pub market: HashMap<String, QASeries>,
}

impl marketcollection {
    pub fn new() -> marketcollection {
        marketcollection {
            market: HashMap::new(),
        }
    }

    pub fn update(&mut self, data: &Value) -> (String, String) {
        let code: &str = data["code"].as_str().unwrap();
        let mut mk = QASeries::init();
        if self.market.contains_key(code) {
            //println!("insert!!");
            mk = (self.market.get_mut(code).unwrap()).to_owned();
        }
        mk.update(data.clone());

        let (data, fixdata) = mk.to_json();
        self.market.insert(code.parse().unwrap(), mk);

        (data, fixdata)
    }

    pub fn get_mut(&mut self, code: &str) -> QASeries {
        let mut mk = QASeries::init();
        if self.market.contains_key(code) {
            println!("insert!!");
            mk = (self.market.get_mut(code).unwrap()).to_owned();
        }
        mk
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::ptr::hash;
    use std::time::Instant;


    use crate::qafastkline::{QAAskBidBase, QASeries};
    use crate::qamarketset::marketcollection;

    #[test]
    fn test_hash() {
        let mut u = HashMap::new();
        let mut mk = QASeries::init();

        //        if u.contains_key("S"){
        //            mk = (u.get_mut("S").unwrap()).to_owned();
        //        }
        u.insert("S".to_string(), mk);
        let l = u.get_mut("S").unwrap();

        println!("{:#?}", l);
    }

    #[test]
    fn test_update() {
        let mut mc = marketcollection::new();

        let new_data = QAAskBidBase {
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
            code: "000001".to_string(),
            open: 20.0,
            high: 24.0,
            low: 19.0,
            close: 21.0,
            amount: 10000,
            productid: 110,
            tickcount: 1120,
            time: "2020-01-20 10:20:02".to_string(),
            vol: 110,
        };

        let new_data1 = QAAskBidBase {
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
            code: "000001".to_string(),
            open: 21.0,
            high: 28.0,
            low: 25.0,
            close: 26.0,
            amount: 20000,
            productid: 110,
            tickcount: 1120,
            time: "2020-01-20 10:20:43".to_string(),
            vol: 210,
        };
        let new_data2 = QAAskBidBase {
            BuyPrices: vec![],
            BuyVols: vec![],
            SellPrices: vec![],
            SellVols: vec![],
            code: "000001".to_string(),
            open: 21.0,
            high: 28.0,
            low: 25.0,
            close: 26.0,
            amount: 20000,
            productid: 110,
            tickcount: 1120,
            time: "2020-01-20 10:25:20".to_string(),
            vol: 210,
        };
        let mut res: String;

        let start = Instant::now();
        let (res, fix) = mc.update(serde_json::to_value(new_data).unwrap().borrow());
        println!("time cost: {:?} us", start.elapsed().as_micros()); // us
        println!("fix~ {}", fix);

        let start = Instant::now();
        let (res, fix) = mc.update(serde_json::to_value(new_data1).unwrap().borrow());
        println!("time cost: {:?} us", start.elapsed().as_micros()); // us
        println!("fix~ {}", fix);

        //test new bar
        let start = Instant::now();
        let (res, fix) = mc.update(serde_json::to_value(new_data2).unwrap().borrow());
        println!("time cost: {:?} us", start.elapsed().as_micros()); // us
        println!("fix~ {}", fix);

        let mut mk = mc.get_mut("000001");
        let (res1, res2) = mk.to_json();
        println!("\nthis is json{:#?}\n", res1);

        //assert_eq!(res, res1)
    }
}
