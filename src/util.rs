//! 其他帮助函数模块
// #![allow(unused)]

use super::biz::BizContenter;
use crate::error::AliPayResult;

use chrono::Utc;
use serde_json;
use std::{collections::HashMap, usize};

use uuid::Uuid;

pub fn get_biz_content_str(w: &impl BizContenter) -> String {
    match serde_json::to_string(&w) {
        Ok(res) => res,
        Err(_) => "".to_owned(),
    }
}

pub fn get_now_beijing_time_str() -> String {
    let loc = chrono::FixedOffset::east_opt(3600 * 8).unwrap();
    let now_time = Utc::now().with_timezone(&loc);
    format!("{}", now_time.format("%Y-%m-%d %H:%M:%S"))
}

pub fn get_out_trade_no() -> String {
    Uuid::new_v4().to_string()
}

pub fn build_form(
    base_url: &str,
    parameters: &mut HashMap<String, String>,
) -> AliPayResult<String> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"<form name=\"alipaysubmit\" method=\"post\" action=\"");
    buf.extend_from_slice(base_url.as_bytes());
    buf.extend_from_slice(b"?charset=utf-8");
    buf.extend_from_slice(b"\">\n");
    buf.extend_from_slice(build_hidden_fields(parameters)?.as_bytes());
    buf.extend_from_slice("<input type=\"submit\" value=\"立即支付\" style=\"display:none\" >\n".as_bytes());
    buf.extend_from_slice(b"</form>\n");
    buf.extend_from_slice(b"<script>document.forms['alipaysubmit'].submit();</script>");
    Ok(String::from_utf8(buf)?)
}

fn build_hidden_fields(parameters: &mut HashMap<String, String>) -> AliPayResult<String> {
    if parameters.is_empty() {
        return Ok("".to_string());
    }
    let mut buf: Vec<u8> = Vec::new();
    for (key, value) in parameters {
        if value.is_empty() {
            continue;
        }
        buf.extend_from_slice(build_hidden_field(key, value)?.as_bytes());
    }
    Ok(String::from_utf8(buf)?)
}

fn build_hidden_field(key: &str, value: &str) -> AliPayResult<String> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"<input type=\"hidden\" name=\"");
    buf.extend_from_slice(key.as_bytes());
    buf.extend_from_slice(b"\" value=\"");
    // 转义双引号
    // let a = strings::ReplaceAll(value, "\"", "&quot;");
    let a = value.replace("\"", "&quot;");
    buf.extend_from_slice(a.as_bytes());
    buf.extend_from_slice(b"\">\n");
    // Ok(buf.String())
    Ok(String::from_utf8(buf)?)
}

// 只支持value是{}或[]或""包裹的key，不支持数字
pub fn json_get(result: &str, key: &str) -> String {
    let len = key.len();
    let i = result.rfind(key).unwrap_or(usize::MAX);
    let mut current = result.as_bytes()[i as usize + len];
    let mut index = i as usize + len;
    while current != b':' {
        index += 1;
        current = result.as_bytes()[index];
    }
    let mut start = index + 1;
    let end: usize;
    index += 1;
    current = result.as_bytes()[index];
    let mut left_brackets = 0_usize;
    if current == b'{' || current == b'[' {
        loop {
            index += 1;
            current = result.as_bytes()[index];
            if current == b'{' || current == b'[' {
                left_brackets += 1;
            }

            if (current == b']' || current == b'}') && left_brackets == 0 {
                break;
            }

            if (current == b']' || current == b'}') && left_brackets > 0 {
                left_brackets -= 1;
            }
        }
        end = index + 1;
    } else {
        index += 1;
        current = result.as_bytes()[index];
        start = index;
        while current != b'"' {
            index += 1;
            current = result.as_bytes()[index];
        }
        end = index;
    }
    match String::from_utf8(result.as_bytes()[start..end].to_vec()) {
        Ok(v) => v,
        Err(_) => "".to_string(),
    }
}

// use gostd::net::url;

// 获取支付宝CallBack异步消息的待签名字符串和签名
// 自行实现签名文档 https://opendocs.alipay.com/common/02mse7?pathHash=096e611e
// 返回值 source - 签名字符串 , sign - 签名 , sign_type - 签名类型
pub fn get_async_callback_msg_source(raw_body: &[u8]) -> AliPayResult<(String, String, String)> {
    // 解析 URL 查询字符串
    let raw_str = std::str::from_utf8(raw_body)?;

    // TO CHECK
    let mut values: HashMap<String, Vec<String>> = HashMap::new();
    url::form_urlencoded::parse(raw_str.as_bytes())
        .map(|(k,v)| values.entry(k.into_owned()).or_default().push(v.into_owned()));



    let sign_type = values.get("sign_type").unwrap()[0].to_owned();
    // 字符串的+会被解析成空格，需要还原回去
    let sign = values.get("sign").unwrap()[0].replace(" ", "+");

    // 待签名字符串不包括sign和sign_type,需要删除
    let mut filtered_values = values.clone();
    filtered_values.remove("sign");
    filtered_values.remove("sign_type");

    // 按字典排序
    let mut keys: Vec<String> = vec![];
    for (k, _) in &filtered_values {
        keys.push(k.to_string());
    }
    keys.sort();

    // 拼接成待签名字符串
    let source: String = keys
        .iter()
        .map(|k| format!("{}={}", k.to_string(), filtered_values.to_owned().get(k).unwrap()[0]))
        .collect::<Vec<String>>()
        .join("&");

    Ok((source, sign, sign_type))
}

use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn base64_encode<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(input)
}

pub fn base64_decode<T>(input: T) -> Result<Vec<u8>, DecodeError>
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.decode(input)
}
