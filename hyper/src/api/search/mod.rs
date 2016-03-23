//! http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_index_type<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                      _type: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 8 + index.len() +
                                  _type.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_search");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post_index_type<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                       _type: &'a str, body: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 8 + index.len() +
                                  _type.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_search");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get_index<'a>(client: &'a mut Client, base: &'a str, index: &'a str)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 8 + index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_search");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post<'a>(client: &'a mut Client, base: &'a str, body: &'a str)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 8);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_search");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_index<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                  body: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 8 + index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_search");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 8);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_search");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
