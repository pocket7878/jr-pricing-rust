use super::place::{Departure, Destination};
use super::amount::Amount;
use super::price_table::PriceTable;

pub struct BasicFare {
	price_table: PriceTable,
} 

impl BasicFare {
	pub fn new() -> Self {
		let mut price_table = PriceTable::new();
		price_table.set_amount(Departure::Tokyo, Destination::ShinOsaka, Amount::new(8910).unwrap());
		price_table.set_amount(Departure::Tokyo, Destination::ShinOsaka, Amount::new(10010).unwrap());

		Self { price_table }
	}

	pub fn amount(&self, departure: Departure, destination: Destination) -> Amount {
		self.price_table.amount(departure, destination)
	}
}