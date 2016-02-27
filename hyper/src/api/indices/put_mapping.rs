//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn post_type<'a>(client: &'a mut Client, base: String, _type: String,
                 body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 11 + _type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&_type);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn put_index_type<'a>(client: &'a mut Client, base: String, index: String,
                      _type: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 11 + index.len() +
                                  _type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&_type);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post_index_type<'a>(client: &'a mut Client, base: String, index: String,
                       _type: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 11 + index.len() +
                                  _type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&_type);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn put_type<'a>(client: &'a mut Client, base: String, _type: String,
                body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 11 + _type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&_type);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(&body);
    res.send()
}
