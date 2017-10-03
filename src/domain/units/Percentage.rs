// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A percentage
#[derive(Debug, Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Percentage<Number: CssNumber>(pub Number);

impl<Number: CssNumber> ToCss for Percentage<Number>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_percentage(self.0.to_f32(), dest)
	}
}

impl<Number: CssNumber> Default for Percentage<Number>
{
	#[inline(always)]
	fn default() -> Self
	{
		Percentage(Number::default())
	}
}

impl<Number: CssNumber> Add<Number> for Percentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn add(self, rhs: Number) -> Self::Output
	{
		Percentage(self.0 + rhs)
	}
}

impl<Number: CssNumber> AddAssign<Number> for Percentage<Number>
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Number)
	{
		*self =	Percentage(self.0 + rhs);
	}
}

impl<Number: CssNumber> Sub<Number> for Percentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn sub(self, rhs: Number) -> Self::Output
	{
		Percentage(self.0 - rhs)
	}
}

impl<Number: CssNumber> SubAssign<Number> for Percentage<Number>
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Number)
	{
		*self =	Percentage(self.0 - rhs);
	}
}

impl<Number: CssNumber> Mul<Number> for Percentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn mul(self, rhs: Number) -> Self::Output
	{
		Percentage(self.0 * rhs)
	}
}

impl<Number: CssNumber> MulAssign<Number> for Percentage<Number>
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Number)
	{
		*self =	Percentage(self.0 * rhs);
	}
}

impl<Number: CssNumber> Div<Number> for Percentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn div(self, rhs: Number) -> Self::Output
	{
		Percentage(self.0 / rhs)
	}
}

impl<Number: CssNumber> DivAssign<Number> for Percentage<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		*self =	Percentage(self.0 / rhs);
	}
}

impl<Number: CssNumber> Rem<Number> for Percentage<Number>
{
	type Output = Percentage<Number>;
	
	#[inline(always)]
	fn rem(self, rhs: Number) -> Self::Output
	{
		Percentage(self.0 % rhs)
	}
}

impl<Number: CssNumber> RemAssign<Number> for Percentage<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		*self =	Percentage(self.0 % rhs);
	}
}

impl<Number: CssNumber> Neg for Percentage<Number>
{
	type Output = Percentage<Number>;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		Percentage(-self.0)
	}
}

impl<Number: CssNumber> CssNumberNewType for Percentage<Number>
{
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.to_CssNumber().to_f32()
	}
	
	#[inline(always)]
	fn as_CssNumber(&self) -> &Number
	{
		self.0.as_CssNumber()
	}
}

impl<Number: CssNumber> Unit for Percentage<Number>
{
	type Number = Number;
	
	#[inline(always)]
	fn to_canonical_dimension(self) -> Self
	{
		self
	}
}

impl<Number: CssNumber> Percentage<Number>
{
	pub const ZeroPercent: Percentage<Number> = Percentage(Number::Zero);
	
	pub const OneHundredPercent: Percentage<Number> = Percentage(Number::One);
	
	#[inline(always)]
	pub fn to_absolute_unit<Conversion: PercentageOfLengthConversion<Number>>(&self, conversion: &Conversion) -> Number
	{
		self.as_CssNumber() * conversion.one_hundred_percent_in_absolute_units()
	}
}
