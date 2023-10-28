use crate::datetime;
use crate::err;
use crate::running::configuring::{Currency, Label};
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

#[derive(Default, Deserialize, Clone, Copy)]
pub struct Amount {
	pub cents: i32,
	pub currency: Currency,
}

impl Amount {
	fn pretty_printable(&self) -> String {
		let major = self.cents / 100;
		let minor = self.cents - major * 100;
		format!(
			"{}.{} {}",
			major,
			{
				if minor == 0 {
					String::from("00")
				} else {
					minor.to_string()
				}
			}
			.as_str(),
			self.currency
		)
	}
}

impl std::fmt::Debug for Amount {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.pretty_printable().fmt(f)
	}
}

impl std::fmt::Display for Amount {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.pretty_printable().fmt(f)
	}
}

impl std::ops::Add for Amount {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		if self.currency != rhs.currency {
			panic!("Unsupported addition between two currencies.")
		}
		Amount {
			cents: self.cents + rhs.cents,
			currency: Currency::Eur,
		}
	}
}

impl std::ops::AddAssign for Amount {
	fn add_assign(&mut self, rhs: Self) {
		if self.currency != rhs.currency {
			panic!("Unsupported addition between two currencies.")
		}
		self.cents += rhs.cents;
	}
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
		Value::String(s) => datetime::parse_from_str(&s).map_err(de::Error::custom),
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
	pub date: NaiveDate,
	sender: Sender,
	receiver: Receiver,
	#[serde(deserialize_with = "deserialize_amount")]
	pub amount: Amount,
	topic: Topic,
	#[serde(default)]
	reference: Reference,
	#[serde(default = "default_turnus")]
	turnus: Turnus,
	#[serde(rename = "labels")]
	pub label_list: Vec<Label>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
	#[serde(rename = "ongoing bookings")]
	pub booking_list: Vec<Booking>,
}
