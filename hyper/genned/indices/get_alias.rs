//Autogenerated
pub fn get(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 7);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_alias");
    url_fmtd
}
pub fn get_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 8 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_alias/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn get_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 8 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_alias/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn get_index(base: String, index: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 7 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_alias");
    url_fmtd
}
