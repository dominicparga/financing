use crate::datetime;
use chrono::NaiveDate;

use crate::running::configuring::{Amount, DataConfig, Label, TimePeriod};

struct Date {
	pretty: String,
	value: NaiveDate,
}

pub fn compute_sums(
	label_list: &[Label],
	time_period: TimePeriod,
	data_config: &DataConfig,
) -> Vec<String> {
	// assume everything being in UTC

	// filling timestamps for grouping the bookings later
	let mut timestamp_list: Vec<Date> = vec![];
	{
		let today: NaiveDate = datetime::today();
		let min_date = data_config
			.booking_list
			.iter()
			.map(|booking| booking.date)
			.min()
			.unwrap();
		match time_period {
			TimePeriod::MONTHLY => {
				let mut last = datetime::last_day_of_month(today);
				while last >= min_date {
					timestamp_list.push(Date {
						pretty: last.format(datetime::FMT_YM).to_string(),
						value: last,
					});
					last = datetime::last_day_of_previous_month(last);
				}
				// needs to be done separatedly since "last" might be "min_date"
				last = datetime::last_day_of_previous_month(min_date);
				timestamp_list.push(Date {
					pretty: last.format(datetime::FMT_YM).to_string(),
					value: last,
				})
			}
			TimePeriod::QUARTERLY => {
				let mut last = datetime::last_day_of_quarter(today);
				while last >= min_date {
					timestamp_list.push(Date {
						pretty: last.format(datetime::FMT_YM).to_string(),
						value: last,
					});
					last = datetime::last_day_of_month(datetime::months_added(last, -3));
				}
				// needs to be done separatedly since "last" might be "min_date"
				last = datetime::last_day_of_month(datetime::months_added(
					datetime::last_day_of_quarter(min_date),
					-3,
				));
				timestamp_list.push(Date {
					pretty: last.format(datetime::FMT_YM).to_string(),
					value: last,
				});
			}
		};
	}

	// list of sums for the average, according to the month timestamp list
	let mut sum_list = vec![Amount::default(); timestamp_list.len()];
	for booking in &data_config.booking_list {
		// ignore the future
		if booking.date > timestamp_list[0].value {
			continue;
		}

		// ignore wrong labels
		let normed_label_list: Vec<Label> =
			label_list.iter().map(|label| label.lowercase()).collect();
		if booking
			.label_list
			.iter()
			.all(|label| !normed_label_list.contains(&label.lowercase()))
		{
			continue;
		}

		// by construction
		// - timestamps are sorted in descending order
		// - timestamps are referring to last day of the time period
		// Hence booking date must be checked in ascending comparing order
		let mut i = timestamp_list.len() - 1;
		while booking.date > timestamp_list[i].value {
			i -= 1;
		}
		sum_list[i] += booking.amount;
	}

	assert_eq!(timestamp_list.len(), sum_list.len());

	// for prettier formatting
	let amount_max_len = sum_list
		.iter()
		.map(|amount| amount.to_string())
		.map(|amount| amount.len())
		.max()
		.unwrap();

	// actual format
	let mut result_list = vec![];
	for i in 0..sum_list.len() {
		let timestamp = timestamp_list[i].pretty.clone();
		result_list.push(format!(
			"{timestamp}: {amount: >width$}",
			timestamp = timestamp,
			amount = sum_list[i],
			width = amount_max_len
		));
	}

	result_list
}
