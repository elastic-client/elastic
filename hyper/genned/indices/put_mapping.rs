//Autogenerated
pub fn put_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_mapping");
    url_fmtd
}
pub fn post_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_mapping");
    url_fmtd
}
pub fn put_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn post_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn put_type(base: String, type: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn post_type(base: String, type: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn put_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_mappings");
    url_fmtd
}
pub fn post_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_mappings");
    url_fmtd
}
pub fn put_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 11 + index.len() + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn post_index_type(base: String, index: String, type: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 11 + index.len() + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn put_type(base: String, type: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 11 + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&type);
    url_fmtd
}
pub fn post_type(base: String, type: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 11 + type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_mappings/");
    url_fmtd.push_str(&type);
    url_fmtd
}
