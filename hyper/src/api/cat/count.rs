//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_index<'a>(client: &'a mut Client, base: &'a str, index: &'a str)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 12 + index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cat/count/");
    url_fmtd.push_str(index);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 11);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cat/count");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
