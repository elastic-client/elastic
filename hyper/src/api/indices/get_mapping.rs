//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html

// Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_index<'a>(client: &'a mut Client, base: &'a str, index: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 1 + 9 + index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_mapping");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_type<'a>(client: &'a mut Client,
                          base: &'a str,
                          index: &'a str,
                          _type: &'a str)
                          -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 1 + 10 + index.len() + _type.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(_type);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 9);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_mapping");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_type<'a>(client: &'a mut Client, base: &'a str, _type: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 10 + _type.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(_type);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
