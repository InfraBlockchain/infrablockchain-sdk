use crate::soft_f64::{u64_widen_mul, F64};

type F = F64;

type FInt = u64;

const fn widen_mul(a: FInt, b: FInt) -> (FInt, FInt) {
	u64_widen_mul(a, b)
}

pub(crate) const fn mul(a: F, b: F) -> F {
	let one: FInt = 1;
	let zero: FInt = 0;

	let bits = F::BITS;
	let significand_bits = F::SIGNIFICAND_BITS;
	let max_exponent = F::EXPONENT_MAX;

	let exponent_bias = F::EXPONENT_BIAS;

	let implicit_bit = F::IMPLICIT_BIT;
	let significand_mask = F::SIGNIFICAND_MASK;
	let sign_bit = F::SIGN_MASK as FInt;
	let abs_mask = sign_bit - one;
	let exponent_mask = F::EXPONENT_MASK;
	let inf_rep = exponent_mask;
	let quiet_bit = implicit_bit >> 1;
	let qnan_rep = exponent_mask | quiet_bit;
	let exponent_bits = F::EXPONENT_BITS;

	let a_rep = a.repr();
	let b_rep = b.repr();

	let a_exponent = (a_rep >> significand_bits) & max_exponent as FInt;
	let b_exponent = (b_rep >> significand_bits) & max_exponent as FInt;
	let product_sign = (a_rep ^ b_rep) & sign_bit;

	let mut a_significand = a_rep & significand_mask;
	let mut b_significand = b_rep & significand_mask;
	let mut scale = 0;

	// Detect if a or b is zero, denormal, infinity, or NaN.
	if a_exponent.wrapping_sub(one) >= (max_exponent - 1) as FInt ||
		b_exponent.wrapping_sub(one) >= (max_exponent - 1) as FInt
	{
		let a_abs = a_rep & abs_mask;
		let b_abs = b_rep & abs_mask;

		// NaN + anything = qNaN
		if a_abs > inf_rep {
			return F::from_repr(a_rep | quiet_bit)
		}
		// anything + NaN = qNaN
		if b_abs > inf_rep {
			return F::from_repr(b_rep | quiet_bit)
		}

		if a_abs == inf_rep {
			if b_abs != zero {
				// infinity * non-zero = +/- infinity
				return F::from_repr(a_abs | product_sign)
			} else {
				// infinity * zero = NaN
				return F::from_repr(qnan_rep)
			}
		}

		if b_abs == inf_rep {
			if a_abs != zero {
				// infinity * non-zero = +/- infinity
				return F::from_repr(b_abs | product_sign)
			} else {
				// infinity * zero = NaN
				return F::from_repr(qnan_rep)
			}
		}

		// zero * anything = +/- zero
		if a_abs == zero {
			return F::from_repr(product_sign)
		}

		// anything * zero = +/- zero
		if b_abs == zero {
			return F::from_repr(product_sign)
		}

		// one or both of a or b is denormal, the other (if applicable) is a
		// normal number.  Renormalize one or both of a and b, and set scale to
		// include the necessary exponent adjustment.
		if a_abs < implicit_bit {
			let (exponent, significand) = F::normalize(a_significand);
			scale += exponent;
			a_significand = significand;
		}

		if b_abs < implicit_bit {
			let (exponent, significand) = F::normalize(b_significand);
			scale += exponent;
			b_significand = significand;
		}
	}

	// Or in the implicit significand bit.  (If we fell through from the
	// denormal path it was already set by normalize( ), but setting it twice
	// won't hurt anything.)
	a_significand |= implicit_bit;
	b_significand |= implicit_bit;

	// Get the significand of a*b.  Before multiplying the significands, shift
	// one of them left to left-align it in the field.  Thus, the product will
	// have (exponentBits + 2) integral digits, all but two of which must be
	// zero.  Normalizing this result is just a conditional left-shift by one
	// and bumping the exponent accordingly.
	let (mut product_low, mut product_high) =
		widen_mul(a_significand, b_significand << exponent_bits);

	let a_exponent_i32: i32 = a_exponent as _;
	let b_exponent_i32: i32 = b_exponent as _;
	let mut product_exponent: i32 = a_exponent_i32
		.wrapping_add(b_exponent_i32)
		.wrapping_add(scale)
		.wrapping_sub(exponent_bias as i32);

	// Normalize the significand, adjust exponent if needed.
	if (product_high & implicit_bit) != zero {
		product_exponent = product_exponent.wrapping_add(1);
	} else {
		product_high = (product_high << 1) | (product_low >> (bits - 1));
		product_low <<= 1;
	}

	// If we have overflowed the type, return +/- infinity.
	if product_exponent >= max_exponent as i32 {
		return F::from_repr(inf_rep | product_sign)
	}

	if product_exponent <= 0 {
		// Result is denormal before rounding
		//
		// If the result is so small that it just underflows to zero, return
		// a zero of the appropriate sign.  Mathematically there is no need to
		// handle this case separately, but we make it a special case to
		// simplify the shift logic.
		let shift = one.wrapping_sub(product_exponent as FInt) as u32;
		if shift >= bits {
			return F::from_repr(product_sign)
		}

		// Otherwise, shift the significand of the result so that the round
		// bit is the high bit of productLo.
		if shift < bits {
			let sticky = product_low << (bits - shift);
			product_low = product_high << (bits - shift) | product_low >> shift | sticky;
			product_high >>= shift;
		} else if shift < (2 * bits) {
			let sticky = product_high << (2 * bits - shift) | product_low;
			product_low = product_high >> (shift - bits) | sticky;
			product_high = zero;
		} else {
			product_high = zero;
		}
	} else {
		// Result is normal before rounding; insert the exponent.
		product_high &= significand_mask;
		product_high |= (product_exponent as FInt) << significand_bits;
	}

	// Insert the sign of the result:
	product_high |= product_sign;

	// Final rounding.  The final result may overflow to infinity, or underflow
	// to zero, but those are the correct results in those cases.  We use the
	// default IEEE-754 round-to-nearest, ties-to-even rounding mode.
	if product_low > sign_bit {
		product_high += one;
	}

	if product_low == sign_bit {
		product_high += product_high & one;
	}

	F::from_repr(product_high)
}

#[cfg(test)]
mod test {
	#[test]
	fn sanity_check() {
		assert_eq!(f64!(2.0).mul(f64!(2.0)), f64!(4.0))
	}
}
