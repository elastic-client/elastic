//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-warmers.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_index<'a>(client: &'a mut Client, base: &'a str, index: &'a str)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 8 + index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_warmer");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_name<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                      name: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 8);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_warmer");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_name<'a>(client: &'a mut Client, base: &'a str, name: &'a str)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 9 + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_type_name<'a>(client: &'a mut Client, base: &'a str,
                           index: &'a str, _type: &'a str, name: &'a str)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  _type.len() + name.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
