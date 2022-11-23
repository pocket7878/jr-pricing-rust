use std::collections::HashMap;

use crate::working_kilo_meter::WorkingKiloMeter;
use crate::place::{Departure, Destination};

pub struct DistanceTable {
	table: HashMap<(Departure, Destination), WorkingKiloMeter>,
}

impl DistanceTable {
	pub fn new() -> Self {
		let mut table = HashMap::new();
		table.insert((Departure::Tokyo, Destination::ShinOsaka), WorkingKiloMeter::new(553).unwrap());
		table.insert((Departure::Tokyo, Destination::Himeji), WorkingKiloMeter::new(664).unwrap());
		Self { table }
	}

	pub fn distance(&self, departure: Departure, destination: Destination) -> WorkingKiloMeter {
		self.table.get(&(departure, destination)).unwrap().clone()
	}
}