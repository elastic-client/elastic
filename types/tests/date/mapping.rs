#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;
use ::date_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultDateMapping::<BasicDateTime>::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "date",
		"format": "basic_date_time"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyDateMapping::<EpochMillis>::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "date",
		"format": "epoch_millis",
		"boost": 1.01,
		"doc_values": true,
		"include_in_all": false,
		"index": true,
		"store": true,
		"ignore_malformed": true,
		"null_value": "1426351513778"
	});

	assert_eq!(expected, ser);
}
