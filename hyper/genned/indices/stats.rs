//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 7);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_stats");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_metric(client: &'a mut hyper::Client, base: String, metric: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 8 + metric.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_stats/");
    url_fmtd.push_str(&metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index(client: &'a mut hyper::Client, base: String, index: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 7 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_stats");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_metric(client: &'a mut hyper::Client, base: String,
                    index: String, metric: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 8 + index.len() +
                                  metric.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_stats/");
    url_fmtd.push_str(&metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
