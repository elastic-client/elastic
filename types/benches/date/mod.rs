use serde_json;
use elastic_types::prelude::*;
use date_fixtures::*;

use test::Bencher;

#[bench]
fn parse_string(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<Date<BasicDateTime>>("\"20150620T134501.034Z\"").unwrap());
}

#[bench]
fn fmt_string(b: &mut Bencher) {
    let dt: Date<DefaultDateFormat> = Date::now();

    b.iter(|| serde_json::to_string(&dt).unwrap());
}

#[bench]
fn parse_epoch(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<Date<EpochMillis>>("\"1435935302478\"").unwrap());
}

#[bench]
fn fmt_epoch(b: &mut Bencher) {
    let dt = Date::<EpochMillis>::now();

    b.iter(|| serde_json::to_string(&dt).unwrap());
}

#[bench]
fn mapping(b: &mut Bencher) {
    b.iter(|| standalone_field_ser(MyDateMapping).unwrap());
}
