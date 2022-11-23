#[derive(Debug, Clone)]
pub struct Amount {
	amount: i32,
}

impl std::fmt::Display for Amount {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}円", self.amount)
	}
}

impl Amount {
	pub fn new(amount: i32) -> Result<Amount, anyhow::Error> {
		if amount % 10 != 0 {
			return Err(anyhow::anyhow!("Amount must be a multiple of 10"));
		}
		
		Ok(Amount { amount })
	}

	pub fn add(&self, other: &Amount) -> Amount {
		Amount::new(self.amount + other.amount).unwrap()
	}

	pub fn value(&self) -> i32 {
		self.amount
	}
}