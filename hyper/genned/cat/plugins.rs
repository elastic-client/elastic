//Autogenerated
pub fn get(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 13);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cat/plugins");
    url_fmtd
}
