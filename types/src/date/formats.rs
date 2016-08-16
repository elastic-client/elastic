use chrono;
use chrono::{ DateTime, NaiveDateTime, UTC, Timelike };
use std::error::Error;
use super::{ CustomDateFormat, ParseError };

/// Format for default `chrono::DateTime`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct ChronoFormat;
impl_date_fmt!(ChronoFormat, "yyyy-MM-ddTHH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ssZ");

/// Format for `basic_date_time_no_millis`.
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct BasicDateTimeNoMillis;
impl_date_fmt!(BasicDateTimeNoMillis, "%Y%m%dT%H%M%SZ", "basic_date_time_no_millis");

/// Format for `basic_date_time`.
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct BasicDateTime;
impl_date_fmt!(BasicDateTime, "%Y%m%dT%H%M%S%.3fZ", "basic_date_time");

/// Format for `epoch_millis`.
///
/// Takes up to a 13 digit string of millis since the epoch and converts to a `DateTime`.
/// This is an efficient formatter, so is a good choice for storing timestamps.
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct EpochMillis;
impl CustomDateFormat for EpochMillis {
	fn name() -> &'static str {
		"epoch_millis"
	}

	fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
		let millis = try!(date.parse::<i64>().map_err(|e| e.description().to_string()));

		//For positive timestamps:
		//Extract the millis straight off the timestamp (how many millis since the last second?)
		let (s, m) = if millis >= 0 {
			((millis / 1000), (millis % 1000))
		}
		//For negative timestamps:
		//The millis need to be inverted (how many millis before the next second?)
		else {
			((millis / 1000) - 1, (1000 + (millis % 1000)))
		};

		Ok(DateTime::from_utc(NaiveDateTime::from_num_seconds_from_unix_epoch(s, m as u32 * 1000000), UTC))
	}

	fn format(date: &DateTime<UTC>) -> String {
		let mut fmtd = String::new();
		
		let msec = ((date.timestamp() * 1000) + (date.nanosecond() as i64 / 1000000)).to_string();
		fmtd.push_str(&msec);

		fmtd
	}
}
