use crate::price_table::PriceTable;
use crate::amount::Amount;
use crate::place::{Departure, Destination};

struct NozomiAdditionalCharge {
	price_table: PriceTable,
}

impl NozomiAdditionalCharge {
	pub fn new() -> Self {
		let mut price_table = PriceTable::new();
		price_table.set_amount(Departure::Tokyo, Destination::ShinOsaka, Amount::new(320).unwrap());
		price_table.set_amount(Departure::Tokyo, Destination::ShinOsaka, Amount::new(530).unwrap());

		Self { price_table }
	}

	pub fn amount(&self, departure: Departure, destination: Destination) -> Amount {
		self.price_table.amount(departure, destination)
	}
}