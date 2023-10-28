use crate::running::configuring::{Label, TimePeriod};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "method")]
pub enum Analysis {
	#[serde(alias = "sum")]
	Sum {
		#[serde(rename = "headline")]
		headline: String,
		#[serde(rename = "labels")]
		label_list: Vec<Label>,
		#[serde(rename = "time period")]
		time_period: TimePeriod,
	},
}

#[derive(Deserialize, Debug)]
pub struct Config {
	#[serde(rename = "analysis")]
	pub analysis_list: Vec<Analysis>,
}
