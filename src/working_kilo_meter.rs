#[derive(Debug, Clone)]
pub struct WorkingKiloMeter {
	pub kilo_meter: i32,
}

impl std::fmt::Display for WorkingKiloMeter {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}km", self.kilo_meter)
	}
}

impl WorkingKiloMeter {
	pub fn new(kilo_meter: i32) -> Result<WorkingKiloMeter, anyhow::Error> {
		if kilo_meter < 0 {
			return Err(anyhow::anyhow!("KiloMeter must be a positive number"));
		}
		
		Ok(WorkingKiloMeter { kilo_meter })
	}
}