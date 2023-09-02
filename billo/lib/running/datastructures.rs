use crate::err;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;

type Sender = String;
type Receiver = String;
type Topic = String;
type Reference = String;

#[derive(Deserialize, Debug)]
pub enum Label {
	#[serde(rename = "Bargeld")]
	CASH,
}

#[derive(Deserialize, Debug)]
pub enum Currency {
	EUR,
}

impl FromStr for Currency {
	type Err = err::Msg;

	fn from_str(s: &str) -> err::Result<Currency> {
		match s {
			"EUR" => Ok(Currency::EUR),
			_ => Err(err::Msg::from(format!("Unknown currency {}", s))),
		}
	}
}

#[derive(Deserialize, Debug)]
enum Turnus {
	#[serde(alias = "yearly")]
	#[serde(alias = "each 12 months")]
	PER_YEAR_1_TIME,
	#[serde(alias = "half a year")]
	#[serde(alias = "each 6 months")]
	PER_YEAR_2_TIMES,
	#[serde(alias = "each 4 months")]
	PER_YEAR_3_TIMES,
	#[serde(alias = "each quarter")]
	#[serde(alias = "each 3 months")]
	PER_YEAR_4_TIMES,
	#[serde(alias = "monthly")]
	#[serde(alias = "each 1 months")]
	PER_MONTH_1_TIME,
}

#[derive(Deserialize, Debug)]
struct Amount {
	cents: i32,
	currency: Currency,
}

impl FromStr for Amount {
	type Err = err::Msg;

	fn from_str(s: &str) -> err::Result<Amount> {
		lazy_static! {
			static ref RE: Regex =
				Regex::new(r"^(?<major>[0-9]+),(?<minor>[0-9][0-9]) (?<currency>EUR)$")
					.expect("Regex definition");
		}
		let captures = RE.captures(s).ok_or(err::Msg::from(format!(
			"Wrong amount '{}' ({})",
			s,
			RE.to_string()
		)))?;

		let cents: i32 = 100
			* captures["major"]
				.parse::<i32>()
				.expect("Amount major digits")
			+ captures["minor"]
				.parse::<i32>()
				.expect("Amount minor digits");
		let currency: Currency = Currency::from_str(&captures["currency"])?;

		Ok(Amount { cents, currency })
	}
}

fn de_amount<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Amount, D::Error> {
	match Value::deserialize(deserializer)
		.map_err(|e| e.to_string())
		.map_err(err::Msg::from)
		.map_err(de::Error::custom)?
	{
		Value::String(s) => Amount::from_str(&s)
			.map_err(|e| e.to_string())
			.map_err(err::Msg::from)
			.map_err(de::Error::custom),
		s => {
			return Err(de::Error::custom(err::Msg::from(format!(
				"Can't interprete amount '{}'",
				s
			))))
		}
	}
}

fn de_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
	match Value::deserialize(deserializer)
		.map_err(|e| e.to_string())
		.map_err(err::Msg::from)
		.map_err(de::Error::custom)?
	{
		Value::String(s) => NaiveDate::parse_from_str(&s, "%Y.%m.%d")
			.map_err(|e| e.to_string())
			.map_err(err::Msg::from)
			.map_err(de::Error::custom),
		s => {
			return Err(de::Error::custom(err::Msg::from(format!(
				"Can't interprete date '{}'",
				s
			))))
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct Analysis {}

#[derive(Deserialize, Debug)]
pub struct Booking {
	#[serde(deserialize_with = "de_date")]
	date: NaiveDate,
	sender: Sender,
	receiver: Receiver,
	#[serde(deserialize_with = "de_amount")]
	amount: Amount,
	topic: Topic,
	reference: Reference,
	#[serde(rename = "labels")]
	label_list: Vec<Label>,
}

#[derive(Deserialize, Debug)]
pub struct Regularity {
	#[serde(deserialize_with = "de_date")]
	date: NaiveDate,
	sender: Sender,
	receiver: Receiver,
	#[serde(deserialize_with = "de_amount")]
	amount: Amount,
	turnus: Turnus,
	#[serde(rename = "labels")]
	label_list: Vec<Label>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
	#[serde(rename = "analysis")]
	pub analysis_list: Vec<Analysis>,
	#[serde(rename = "regular bookings")]
	pub regularity_list: Vec<Regularity>,
	#[serde(rename = "ongoing bookings")]
	pub booking_list: Vec<Booking>,
}