use crate::price_table::PriceTable;
use crate::amount::Amount;
use crate::place::{Departure, Destination};

pub struct HikariReservedSeatSurcharge {
	price_table: PriceTable,
}

impl HikariReservedSeatSurcharge {
	pub fn new() -> Self {
		let mut price_table = PriceTable::new();
		price_table.set_amount(Departure::Tokyo, Destination::ShinOsaka, Amount::new(5490).unwrap());
		price_table.set_amount(Departure::Tokyo, Destination::ShinOsaka, Amount::new(5920).unwrap());

		Self { price_table }
	}

	pub fn amount(&self, departure: Departure, destination: Destination) -> Amount {
		self.price_table.amount(departure, destination)
	}
}