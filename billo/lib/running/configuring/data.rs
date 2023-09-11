use crate::err;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct Sender(String);

#[derive(Deserialize, Debug)]
pub struct Receiver(String);

#[derive(Deserialize, Debug)]
pub struct Topic(String);

#[derive(Deserialize, Default, Debug)]
pub struct Reference(String);

#[derive(Deserialize, Debug)]
pub struct Label(String);

#[derive(Deserialize, Debug)]
pub enum Currency {
	Eur,
}

impl FromStr for Currency {
	type Err = err::Msg;

	fn from_str(s: &str) -> err::Result<Currency> {
		match s {
			"EUR" => Ok(Currency::Eur),
			_ => Err(err::Msg::from(format!("Unknown currency {}", s))),
		}
	}
}

#[derive(Deserialize, Debug)]
enum Turnus {
	#[serde(alias = "once")]
	Once,
	#[serde(alias = "each 12 months", alias = "yearly")]
	PerYear1Time {
		#[serde(rename = "is active")]
		is_active: bool,
	},
	#[serde(alias = "each 6 months", alias = "half a year")]
	PerYear2Times {
		#[serde(rename = "is active")]
		is_active: bool,
	},
	#[serde(alias = "each 4 months")]
	PerYear3Times {
		#[serde(rename = "is active")]
		is_active: bool,
	},
	#[serde(alias = "each 3 months", alias = "each quarter")]
	PerYear4Times {
		#[serde(rename = "is active")]
		is_active: bool,
	},
	#[serde(alias = "each 1 months", alias = "monthly")]
	PerMonth1Time {
		#[serde(rename = "is active")]
		is_active: bool,
	},
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

fn deserialize_amount<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Amount, D::Error> {
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

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
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

fn default_turnus() -> Turnus {
	Turnus::Once
}

fn default_true() -> bool {
	true
}

#[derive(Deserialize, Debug)]
pub struct Booking {
	#[serde(deserialize_with = "deserialize_date")]
	date: NaiveDate,
	sender: Sender,
	receiver: Receiver,
	#[serde(deserialize_with = "deserialize_amount")]
	amount: Amount,
	topic: Topic,
	#[serde(default)]
	reference: Reference,
	#[serde(default = "default_turnus")]
	turnus: Turnus,
	#[serde(rename = "labels")]
	label_list: Vec<Label>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
	#[serde(rename = "ongoing bookings")]
	pub booking_list: Vec<Booking>,
}
