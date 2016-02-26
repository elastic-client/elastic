//Autogenerated
use hyper::Client;
pub fn get(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 16);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_validate/query");
    url_fmtd
}
pub fn post(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 16);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_validate/query");
    url_fmtd
}
pub fn get_index(base: String, index: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 16 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_validate/query");
    url_fmtd
}
pub fn post_index(base: String, index: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 16 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_validate/query");
    url_fmtd
}
pub fn get_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 16 + index.len() +
                                  type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_validate/query");
    url_fmtd
}
pub fn post_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 16 + index.len() +
                                  type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_validate/query");
    url_fmtd
}
