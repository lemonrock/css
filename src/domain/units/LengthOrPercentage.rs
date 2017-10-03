// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LengthOrPercentage<L: Length>
{
	IsLength(L),
	IsPercentage(Percentage<L::Number>)
}

impl<L: Length> ToCss for LengthOrPercentage<L>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			IsLength(ref length) => length.to_css(dest),
			IsPercentage(ref length) => length.to_css(dest),
		}
	}
}

impl<L: Length> Default for LengthOrPercentage<L>
{
	#[inline(always)]
	fn default() -> Self
	{
		LengthOrPercentage::IsLength(L::default())
	}
}

impl<L: Length> Add<Number> for LengthOrPercentage<L>
{
	type Output = LengthOrPercentage<L>;
	
	#[inline(always)]
	fn add(self, rhs: L::Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length + rhs),
			IsPercentage(length) => IsPercentage(length + rhs),
		}
	}
}

impl<L: Length> AddAssign<Number> for LengthOrPercentage<L>
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: L::Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length + rhs,
			IsPercentage(ref mut length) => *length = *length + rhs,
		}
	}
}

impl<L: Length> Sub<Number> for LengthOrPercentage<L>
{
	type Output = LengthOrPercentage<L>;
	
	#[inline(always)]
	fn sub(self, rhs: L::Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length - rhs),
			IsPercentage(length) => IsPercentage(length - rhs),
		}
	}
}

impl<L: Length> SubAssign<Number> for LengthOrPercentage<L>
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: L::Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length - rhs,
			IsPercentage(ref mut length) => *length = *length - rhs,
		}
	}
}

impl<L: Length> Mul<Number> for LengthOrPercentage<L>
{
	type Output = LengthOrPercentage<L>;
	
	#[inline(always)]
	fn mul(self, rhs: L::Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length * rhs),
			IsPercentage(length) => IsPercentage(length * rhs),
		}
	}
}

impl<L: Length> MulAssign<Number> for LengthOrPercentage<L>
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: L::Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length * rhs,
			IsPercentage(ref mut length) => *length = *length * rhs,
		}
	}
}

impl<L: Length> Div<Number> for LengthOrPercentage<L>
{
	type Output = LengthOrPercentage<L>;
	
	#[inline(always)]
	fn div(self, rhs: L::Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length / rhs),
			IsPercentage(length) => IsPercentage(length / rhs),
		}
	}
}

impl<L: Length> DivAssign<Number> for LengthOrPercentage<L>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: L::Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length / rhs,
			IsPercentage(ref mut length) => *length = *length / rhs,
		}
	}
}

impl<L: Length> Rem<Number> for LengthOrPercentage<L>
{
	type Output = LengthOrPercentage<L>;
	
	#[inline(always)]
	fn rem(self, rhs: L::Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length % rhs),
			IsPercentage(length) => IsPercentage(length % rhs),
		}
	}
}

impl<L: Length> RemAssign<Number> for LengthOrPercentage<L>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: L::Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length % rhs,
			IsPercentage(ref mut length) => *length = *length % rhs,
		}
	}
}

impl<L: Length> Neg for LengthOrPercentage<L>
{
	type Output = LengthOrPercentage<L>;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(-length),
			IsPercentage(length) => IsPercentage(-length),
		}
	}
}

impl<L: Length> CssNumberNewType for LengthOrPercentage<L>
{
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.to_CssNumber().to_f32()
	}
	
	#[inline(always)]
	fn as_CssNumber(&self) -> &L::Number
	{
		match *self
		{
			IsLength(ref length) => length.as_CssNumber(),
			IsPercentage(ref length) => length.as_CssNumber(),
		}
	}
}

impl<L: Length> Unit for LengthOrPercentage<L>
{
	type Number = L::Number;
	
	#[inline(always)]
	fn to_canonical_dimension(self) -> Self
	{
		match self
		{
			IsLength(length) => IsLength(length.to_canonical_dimension()),
			IsPercentage(..) => self,
		}
	}
}

impl<L: Length> LengthOrPercentage<L>
{
	/// Get an absolute length using a conversion
	#[inline(always)]
	pub fn to_px<Conversion: FontRelativeLengthConversion<L::Number> + ViewportPercentageLengthConversion<L::Number> + PercentageOfLengthConversion<L::Number>>(&self, conversion: &Conversion) -> L::Number
	{
		match *self
		{
			IsLength(ref length) => length.to_px(conversion),
			IsPercentage(ref percentage) => percentage.to_absolute_unit(conversion),
		}
	}
	
	/// Convert this into AppUnits.
	#[inline]
	pub fn to_app_units<Conversion: FontRelativeLengthConversion<L::Number> + ViewportPercentageLengthConversion<L::Number> + PercentageOfLengthConversion<L::Number>>(&self, conversion: &Conversion) -> L::Number
	{
		self.to_px(conversion) * L::Number::AppUnitsPerPX
	}
}
