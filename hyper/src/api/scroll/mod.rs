//! http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-scroll.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn post<'a>(client: &'a mut Client, base: String, body: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 15);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_search/scroll");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post_scroll_id<'a>(client: &'a mut Client, base: String, scroll_id: String,
                      body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + scroll_id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_search/scroll/");
    url_fmtd.push_str(&scroll_id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 15);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_search/scroll");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_scroll_id<'a>(client: &'a mut Client, base: String, scroll_id: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + scroll_id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_search/scroll/");
    url_fmtd.push_str(&scroll_id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
