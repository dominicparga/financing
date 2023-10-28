use crate::err;
use chrono::{Datelike, NaiveDate, Utc};

pub const FMT_YM: &str = "%Y.%m";
pub const FMT_YMD: &str = "%Y.%m.%d";

pub fn parse_from_str(s: &str) -> err::Result<NaiveDate> {
	NaiveDate::parse_from_str(&s, FMT_YMD)
		.map_err(|e| e.to_string())
		.map_err(err::Msg::from)
}

pub fn today() -> NaiveDate {
	Utc::now().naive_utc().into()
}

pub fn first_day_of_previous_month(date: NaiveDate) -> NaiveDate {
	date.with_day(1)
		.unwrap()
		.pred_opt()
		.unwrap()
		.with_day(1)
		.unwrap()
}

pub fn last_day_of_previous_month(date: NaiveDate) -> NaiveDate {
	date.with_day(1).unwrap().pred_opt().unwrap()
}

pub fn first_day_of_month(date: NaiveDate) -> NaiveDate {
	// pub fn first_day_of_month(year: i32, month: u32) -> NaiveDate {
	// 	NaiveDate::from_ymd_opt(year, month + 1, 1)
	// 		.unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
	// 		.pred_opt()
	// 		.unwrap()
	// }

	date.with_day(1).unwrap()
}

pub fn last_day_of_month(date: NaiveDate) -> NaiveDate {
	// pub fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
	// 	NaiveDate::from_ymd_opt(year, month + 1, 1)
	// 		.unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
	// 		.pred_opt()
	// 		.unwrap()
	// }

	date.checked_add_months(chrono::Months::new(1))
		.unwrap()
		.with_day(1)
		.unwrap()
		.pred_opt()
		.unwrap()
}

pub fn last_day_of_quarter(date: NaiveDate) -> NaiveDate {
	last_day_of_month(
		match date.month() {
			1..=3 => NaiveDate::from_ymd_opt(date.year(), 3, 1),
			4..=6 => NaiveDate::from_ymd_opt(date.year(), 6, 1),
			7..=9 => NaiveDate::from_ymd_opt(date.year(), 9, 1),
			10..=12 => NaiveDate::from_ymd_opt(date.year(), 12, 1),
			_ => panic!("Month out of 1..=12 should not happen."),
		}
		.unwrap(),
	)
}

pub fn months_added(date: NaiveDate, m: i32) -> NaiveDate {
	if m >= 0 {
		date.checked_add_months(chrono::Months::new(m.try_into().unwrap()))
			.unwrap()
	} else {
		date.checked_sub_months(chrono::Months::new((-m).try_into().unwrap()))
			.unwrap()
	}
}
