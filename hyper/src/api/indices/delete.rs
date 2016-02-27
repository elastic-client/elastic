//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html

// Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn delete_index<'a>(client: &'a mut Client, base: &'a str, index: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 1 + index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.delete(&url_fmtd).headers(headers);
    res.send()
}
