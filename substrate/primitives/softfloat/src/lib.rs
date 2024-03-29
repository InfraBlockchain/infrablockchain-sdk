#![cfg_attr(not(feature = "std"), no_std)]

/// Creates a new [`F64`] from a floating-point literal.
#[macro_export]
macro_rules! f64 {
	($value:expr) => {{
		const C: $crate::F64 = { $crate::F64::from_native_f64($value) };
		C
	}};
}

mod conv;
mod soft_f64;

pub use crate::soft_f64::F64;

const fn abs_diff(a: i32, b: i32) -> u32 {
	a.wrapping_sub(b).wrapping_abs() as u32
}

pub trait BlockTimeWeight<Amount, BlockNumber>:
	Sized
	+ Copy
	+ core::fmt::Debug
	+ PartialEq
	+ PartialOrd
	+ core::ops::Add
	+ core::ops::Sub
	+ core::ops::Div
	+ core::ops::Mul
	+ core::ops::AddAssign
	+ core::ops::SubAssign
	+ core::ops::MulAssign
	+ core::ops::DivAssign
{
	fn rational(n: BlockNumber, d: BlockNumber) -> Self;

	fn block_time_weight(adjusted: Amount, current: BlockNumber, per_year: BlockNumber) -> Self;
}

macro_rules! impl_traits {
	($ty:ty, $native_ty:ty, $from_native:ident, $to_native:ident) => {
		impl From<$native_ty> for $ty {
			fn from(value: $native_ty) -> Self {
				Self::$from_native(value)
			}
		}

		impl From<$ty> for $native_ty {
			fn from(value: $ty) -> Self {
				value.$to_native()
			}
		}

		impl PartialEq<Self> for $ty {
			fn eq(&self, other: &Self) -> bool {
				match self.cmp(other) {
					Some(core::cmp::Ordering::Equal) => true,
					_ => false,
				}
			}
		}

		impl PartialOrd for $ty {
			fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
				self.cmp(other)
			}
		}

		impl core::ops::Add for $ty {
			type Output = Self;

			fn add(self, rhs: Self) -> Self::Output {
				Self::add(self, rhs)
			}
		}

		impl core::ops::Sub for $ty {
			type Output = Self;

			fn sub(self, rhs: Self) -> Self::Output {
				Self::sub(self, rhs)
			}
		}

		impl core::ops::Mul for $ty {
			type Output = Self;

			fn mul(self, rhs: Self) -> Self::Output {
				Self::mul(self, rhs)
			}
		}

		impl core::ops::Div for $ty {
			type Output = Self;

			fn div(self, rhs: Self) -> Self::Output {
				Self::div(self, rhs)
			}
		}

		impl core::ops::Neg for $ty {
			type Output = Self;

			fn neg(self) -> Self::Output {
				Self::neg(self)
			}
		}

		impl core::ops::AddAssign for $ty {
			fn add_assign(&mut self, rhs: Self) {
				*self = *self + rhs;
			}
		}

		impl core::ops::SubAssign for $ty {
			fn sub_assign(&mut self, rhs: Self) {
				*self = *self - rhs;
			}
		}

		impl core::ops::MulAssign for $ty {
			fn mul_assign(&mut self, rhs: Self) {
				*self = *self * rhs;
			}
		}

		impl core::ops::DivAssign for $ty {
			fn div_assign(&mut self, rhs: Self) {
				*self = *self / rhs;
			}
		}

		impl<Amount, BlockNumber> BlockTimeWeight<Amount, BlockNumber> for $ty
		where
			Amount: Into<$ty> + Clone,
			BlockNumber: Into<Amount>,
			$ty: From<i32>,
		{
			fn rational(n: BlockNumber, d: BlockNumber) -> $ty {
				let n: Amount = n.into();
				let d: Amount = d.into();
				let n: $ty = n.into();
				let d: $ty = d.into();
				let exp = Self::from(2i32).ln();
				let block_passed_rational = n / d;
				exp * block_passed_rational
			}

			fn block_time_weight(
				adjusted: Amount,
				current: BlockNumber,
				per_year: BlockNumber,
			) -> $ty {
				let block_time_rational = Self::rational(current, per_year).exp();
				let adjusted: $ty = adjusted.into();
				let block_time_weight = adjusted.mul(block_time_rational);
				block_time_weight
			}
		}
	};
}

impl_traits!(crate::soft_f64::F64, f64, from_native_f64, to_native_f64);

#[cfg(test)]
mod tests {
	use crate::soft_f64::F64;

	const RANGE: core::ops::Range<i32> = -1000..1000;
	const F64_FACTOR: f64 = 1000.0;

	#[test]
	fn f64_add() {
		for a in RANGE {
			let a = a as f64 * F64_FACTOR;
			for b in RANGE {
				let b = b as f64 * F64_FACTOR;
				assert_eq!(
					F64::from_native_f64(a).sub(F64::from_native_f64(b)).to_native_f64(),
					a - b
				);
			}
		}
	}

	#[test]
	fn f64_sub() {
		for a in RANGE {
			let a = a as f64 * F64_FACTOR;
			for b in RANGE {
				let b = b as f64 * F64_FACTOR;
				assert_eq!(
					F64::from_native_f64(a).sub(F64::from_native_f64(b)).to_native_f64(),
					a - b
				);
			}
		}
	}

	#[test]
	fn f64_mul() {
		for a in RANGE {
			let a = a as f64 * F64_FACTOR;
			for b in RANGE {
				let b = b as f64 * F64_FACTOR;
				assert_eq!(
					F64::from_native_f64(a).mul(F64::from_native_f64(b)).to_native_f64(),
					a * b
				);
			}
		}
	}

	#[test]
	fn f64_div() {
		for a in RANGE {
			let a = a as f64 * F64_FACTOR;
			for b in RANGE {
				let b = b as f64 * F64_FACTOR;
				let x = F64::from_native_f64(a).div(F64::from_native_f64(b)).to_native_f64();
				let y = a / b;
				assert!(x == y || x.is_nan() && y.is_nan())
			}
		}
	}
}
