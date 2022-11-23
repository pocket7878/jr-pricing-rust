use crate::amount::Amount;
use crate::rider_type::RiderType;

pub struct RiderTypeDiscount {
	rate: f64,
}

impl RiderTypeDiscount {
	pub fn new(rider_type: RiderType) -> Self {
		let rate = match rider_type {
			RiderType::Adult => 1.0,
			RiderType::Child => 0.5,
		};
		Self { rate }
	}

	pub fn apply(&self, amount: Amount) -> Amount {
		let mut discounted_amount = amount.value() as f64;
		discounted_amount *= self.rate;
		let mut discounted_amount = discounted_amount as i32;
		if discounted_amount % 10 == 5 {
			discounted_amount -= 5;
		}

		Amount::new(discounted_amount).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_adult_discount() {
		let discount = RiderTypeDiscount::new(RiderType::Adult);
		let amount = Amount::new(8910).unwrap();
		let discounted_amount = discount.apply(amount);
		assert_eq!(discounted_amount.value(), 8910);
	}

	#[test]
	fn test_child_discount() {
		let discount = RiderTypeDiscount::new(RiderType::Child);
		let amount = Amount::new(8910).unwrap();
		let discounted_amount = discount.apply(amount);
		assert_eq!(discounted_amount.value(), 4450);
	}
}