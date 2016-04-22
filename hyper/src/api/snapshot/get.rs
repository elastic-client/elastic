//! http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html

//Autogenerated

use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

pub fn get_repository_snapshot<'a>(client: &'a mut Client, req: RequestParams,
                               repository: &'a str, snapshot: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 11 + 1 + repository.len() +
                                  snapshot.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_snapshot/");
    url_fmtd.push_str(repository);
    url_fmtd.push_str("/");
    url_fmtd.push_str(snapshot);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers);
    res.send()
}
