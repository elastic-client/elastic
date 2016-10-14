#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, json_str, elastic_types_derive, elastic_date_macros)]

pub mod mapping;
pub mod formats;

extern crate serde;
extern crate serde_json;
pub extern crate chrono;
extern crate elastic_types;

use chrono::offset::TimeZone;

use elastic_types::date::prelude::*;

const MYTYPE_DATE_FMT_1: &'static str = "%Y/%m/%d %H:%M:%S";
const MYTYPE_DATE_FMT_2: &'static str = "%d/%m/%Y %H:%M:%S";

#[derive(Default, Clone)]
pub struct TestDateFormat1;
date_fmt!(TestDateFormat1, "%Y/%m/%d %H:%M:%S", "test_date_1");

#[derive(Default, Clone, Copy)]
pub struct TestDateFormat2;
date_fmt!(TestDateFormat2, "yyyyMMdd", "test_date_2");

#[test]
fn dates_should_use_chrono_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = _dt.format(MYTYPE_DATE_FMT_1).to_string();

	let dt = Date::<TestDateFormat1>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn dates_should_use_es_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = "20150513".to_string();

	let dt = Date::<TestDateFormat2>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn can_change_date_mapping() {
	fn takes_epoch_millis(_: Date<EpochMillis>) -> bool {
		true
	}

	let date: Date<BasicDateTime> = Date::now();

	assert!(takes_epoch_millis(date.remap()));
}

#[test]
fn can_build_date_from_chrono() {
	let date: Date<DefaultDateFormat> = Date::new(
		chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap()
	);

	assert_eq!((2015, 5, 13, 0, 0, 0), (
		date.year(),
		date.month(),
		date.day(),
		date.hour(),
		date.minute(),
		date.second()
	));
}

#[test]
fn can_build_date_from_prim() {
	let date: Date<DefaultDateFormat> = Date::build(
		2015, 5, 13, 0, 0, 0, 0
	);

	assert_eq!((2015, 5, 13, 0, 0, 0), (
		date.year(),
		date.month(),
		date.day(),
		date.hour(),
		date.minute(),
		date.second()
	));
}

#[test]
fn serialise_elastic_date() {
	let date = Date::<BasicDateTime>::new(
		chrono::UTC.datetime_from_str(
			"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
		).unwrap()
	);

	let ser = serde_json::to_string(&date).unwrap();

	assert_eq!(r#""20150513T000000.000Z""#, ser);
}

#[test]
fn deserialise_elastic_date() {
	let date: Date<BasicDateTime> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

	assert_eq!((2015, 5, 13), (
		date.year(),
		date.month(),
		date.day()
	));
}

#[test]
fn serialise_elastic_date_brw() {
	let chrono_date = chrono::UTC.datetime_from_str(
		"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
	).unwrap();

	let date = DateBrw::<BasicDateTime>::new(&chrono_date);

	let ser = serde_json::to_string(&date).unwrap();

	assert_eq!(r#""20150513T000000.000Z""#, ser);
}
