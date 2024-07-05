#![allow(unused)]
use std::hash::BuildHasher;

use super::{BizContenter, BizObject,V};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeCancelBiz(BizObject);

impl BizContenter for TradeCancelBiz

{
    fn method(&self) -> String {
        "alipay.trade.cancel".to_string()
    }

    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }
}
// 以下是设置必选字段方法
impl TradeCancelBiz

{
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_trade_no(&mut self, value: V) {
        self.set("trade_no", value);
    }

    pub fn set_out_trade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }
}
