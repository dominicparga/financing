use crate::err;
use serde::Deserialize;
use std::str::FromStr;

mod analysis;
mod data;
pub use analysis::{Analysis, Config as AnalysisConfig};
pub use data::{Amount, Config as DataConfig};

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum TimePeriod {
	#[serde(alias = "monthly")]
	MONTHLY,
	#[serde(alias = "quarterly")]
	QUARTERLY,
}

#[derive(Deserialize, PartialEq, Eq)]
pub struct Label(String);

impl Label {
	pub fn lowercase(&self) -> Label {
		Label(self.0.to_lowercase())
	}
}

impl std::fmt::Debug for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::fmt::Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self, f)
	}
}

#[derive(Default, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
	#[default]
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

impl std::fmt::Display for Currency {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Eur => "EUR",
		}
		.fmt(f)
	}
}
