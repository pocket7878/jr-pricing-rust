use std::collections::HashMap;

use super::place;
use super::amount;

pub struct PriceTable {
	table: HashMap<(place::Departure, place::Destination), amount::Amount>,
} 

impl PriceTable {
	pub fn new() -> Self {
		let mut table = HashMap::new();
		Self { table }
	}

	pub fn set_amount(&mut self, departure: place::Departure, destination: place::Destination, amount: amount::Amount) {
		self.table.insert((departure, destination), amount);
	}

	pub fn amount(&self, departure: place::Departure, destination: place::Destination) -> amount::Amount {
		self.table.get(&(departure, destination)).unwrap().clone()
	}
}