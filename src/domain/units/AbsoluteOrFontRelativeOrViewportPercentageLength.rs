// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A `<length>` without taking `expressions` expressions into account
///
/// https://drafts.csswg.org/css-values/#lengths
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AbsoluteOrFontRelativeOrViewportPercentage<Number: CssNumber>
{
	/// An absolute length
	///
	/// https://drafts.csswg.org/css-values/#absolute-length
	Absolute(AbsoluteLength),
	
	/// A font-relative length:
	///
	/// https://drafts.csswg.org/css-values/#font-relative-lengths
	FontRelative(FontRelativeLength),
	
	/// A viewport-relative length.
	///
	/// https://drafts.csswg.org/css-values/#viewport-relative-lengths
	ViewportPercentage(ViewportPercentageLength),
}

impl<Number: CssNumber> ToCss for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			Absolute(ref length) => length.to_css(dest),
			FontRelative(ref length) => length.to_css(dest),
			ViewportPercentage(ref length) => length.to_css(dest),
		}
	}
}

impl<Number: CssNumber> Length for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Number = Number;
}

impl<Number: CssNumber> Unit for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Number = Number;
}

impl<Number: CssNumber> Default for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn default() -> Self
	{
		AbsoluteOrFontRelativeOrViewportPercentage::Absolute(AbsoluteLength::default())
	}
}

impl<Number: CssNumber> Add<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn add(self, rhs: Number) -> Self::Output
	{
		match self
		{
			Absolute(length) => Absolute(length + rhs),
			FontRelative(length) => FontRelative(length + rhs),
			ViewportPercentage(length) => ViewportPercentage(length + rhs),
		}
	}
}

impl<Number: CssNumber> AddAssign<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Number)
	{
		match *self
		{
			Absolute(ref mut length) => *length = *length + rhs,
			FontRelative(ref mut length) => *length = *length + rhs,
			ViewportPercentage(ref mut length) => *length = *length + rhs,
		}
	}
}

impl<Number: CssNumber> Sub<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn sub(self, rhs: Number) -> Self::Output
	{
		match self
		{
			Absolute(length) => Absolute(length - rhs),
			FontRelative(length) => FontRelative(length - rhs),
			ViewportPercentage(length) => ViewportPercentage(length - rhs),
		}
	}
}

impl<Number: CssNumber> SubAssign<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Number)
	{
		match *self
		{
			Absolute(ref mut length) => *length = *length - rhs,
			FontRelative(ref mut length) => *length = *length - rhs,
			ViewportPercentage(ref mut length) => *length = *length - rhs,
		}
	}
}

impl<Number: CssNumber> Mul<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn mul(self, rhs: Number) -> Self::Output
	{
		match self
		{
			Absolute(length) => Absolute(length * rhs),
			FontRelative(length) => FontRelative(length * rhs),
			ViewportPercentage(length) => ViewportPercentage(length * rhs),
		}
	}
}

impl<Number: CssNumber> MulAssign<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Number)
	{
		match *self
		{
			Absolute(ref mut length) => *length = *length * rhs,
			FontRelative(ref mut length) => *length = *length * rhs,
			ViewportPercentage(ref mut length) => *length = *length * rhs,
		}
	}
}

impl<Number: CssNumber> Div<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Output = ViewportPercentage<Number>;
	
	#[inline(always)]
	fn div(self, rhs: Number) -> Self::Output
	{
		match self
		{
			Absolute(length) => Absolute(length / rhs),
			FontRelative(length) => FontRelative(length / rhs),
			ViewportPercentage(length) => ViewportPercentage(length / rhs),
		}
	}
}

impl<Number: CssNumber> DivAssign<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		match *self
		{
			Absolute(ref mut length) => *length = *length / rhs,
			FontRelative(ref mut length) => *length = *length / rhs,
			ViewportPercentage(ref mut length) => *length = *length / rhs,
		}
	}
}

impl<Number: CssNumber> Rem<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Output = AbsoluteOrFontRelativeOrViewportPercentage<Number>;
	
	#[inline(always)]
	fn rem(self, rhs: Number) -> Self::Output
	{
		match self
		{
			Absolute(length) => Absolute(length % rhs),
			FontRelative(length) => FontRelative(length % rhs),
			ViewportPercentage(length) => ViewportPercentage(length % rhs),
		}
	}
}

impl<Number: CssNumber> RemAssign<Number> for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		match *self
		{
			Absolute(ref mut length) => *length = *length % rhs,
			FontRelative(ref mut length) => *length = *length % rhs,
			ViewportPercentage(ref mut length) => *length = *length % rhs,
		}
	}
}

impl<Number: CssNumber> Neg for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	type Output = AbsoluteOrFontRelativeOrViewportPercentage<Number>;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		match self
		{
			Absolute(length) => Absolute(-length),
			FontRelative(length) => FontRelative(-length),
			ViewportPercentage(length) => ViewportPercentage(-length),
		}
	}
}

impl<Number: CssNumber> CssNumberNewType for AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.to_CssNumber().to_f32()
	}
	
	#[inline(always)]
	fn as_CssNumber(&self) -> &Number
	{
		match *self
		{
			Absolute(ref length) => length.as_CssNumber(),
			FontRelative(ref length) => length.as_CssNumber(),
			ViewportPercentage(ref length) => length.as_CssNumber(),
		}
	}
}

impl<Number: CssNumber> AbsoluteOrFontRelativeOrViewportPercentage<Number>
{
	#[inline(always)]
	pub fn is_absolute_length(&self) -> bool
	{
		match *self
		{
			Absolute(..) => true,
			FontRelative(..) | ViewportPercentage(..) => false,
		}
	}
	
	#[inline(always)]
	pub fn is_relative_length(&self) -> bool
	{
		!self.is_absolute_length()
	}
	
	pub(crate) fn parse_length_dimension<'i>(context: &ParserContext, value: Number, unit: &CowRcStr<'i>) -> Result<Self, CustomParseError<'i>>
	{
		use self::AbsoluteLength::*;
		use self::FontRelativeLength::*;
		use self::ViewportPercentage::*;
		
		let isInPageRule = context.isInPageRule();
		match_ignore_ascii_case!
		{
			&*unit,
			
            "px" => Ok(Absolute(px(value))),
            
            "in" => Ok(Absolute(in_(value))),
            
            "cm" => Ok(Absolute(cm(value))),
            
            "mm" => Ok(Absolute(mm(value))),
            
            "q" => Ok(Absolute(q(value))),
            
            "pt" => Ok(Absolute(pt(value))),
            
            "pc" => Ok(Absolute(pc(value))),
            
            "em" => Ok(FontRelative(em(value))),
            
            "ex" => Ok(FontRelative(ex(value))),
            
            "ch" => Ok(FontRelative(ch(value))),
            
            "rem" => Ok(FontRelative(rem(value))),
            
            "Absolute" => if isInPageRule
			{
				Err(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule)
			}
			else
			{
				Ok(ViewportPercentage(Absolute(value)))
			}
            
            "FontRelative" => if isInPageRule
			{
				Err(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule)
			}
			else
			{
				Ok(ViewportPercentage(FontRelative(value)))
			}
            
            "ViewportPercentage" => if isInPageRule
			{
				Err(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule)
			}
			else
			{
				Ok(ViewportPercentage(ViewportPercentage(value)))
			}
            
            "vmax" => if isInPageRule
			{
				Err(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule)
			}
			else
			{
				Ok(ViewportPercentage(vmax(value)))
			}
            
            _ => Err(CustomParseError::LengthDimensionWasUnrecognised(unit.to_owned()))
        }
	}
	
	/// Checks whether the length value is zero.
	#[inline]
	pub fn is_absolute_zero(&self) -> bool
	{
		use self::AbsoluteOrFontRelativeOrViewportPercentage::Absolute;
		
		match *self
		{
			Absolute(length) => length.is_zero(),
			_ => false
		}
	}
	
	/// Get an absolute length from a px value.
	#[inline]
	pub fn from_px(pixels: Number) -> Self
	{
		use self::AbsoluteOrFontRelativeOrViewportPercentage::Absolute;
		use self::AbsoluteLength::Px;
		
		Absolute(px(pixels))
	}
	
	/// Convert this into a pixel value.
	#[inline(always)]
	pub fn to_px<Conversion: FontRelativeLengthConversion<Number> + ViewportPercentageLengthConversion<Number>>(&self, conversion: &Conversion) -> L::Number
	{
		match *self
		{
			Absolute(ref length) => length.to_px(),
			FontRelative(ref length) => length.to_px(conversion),
			ViewportPercentage(ref length) => length.to_px(conversion),
		}
	}
	
	/// Convert this into AppUnits.
	#[inline]
	pub fn to_app_units<Conversion: FontRelativeLengthConversion<Number> + ViewportPercentageLengthConversion<Number>>(&self, conversion: &Conversion) -> Number
	{
		match *self
		{
			Absolute(ref length) => length.app_units(),
			FontRelative(ref length) => length.app_units(conversion),
			ViewportPercentage(ref length) => length.app_units(conversion),
		}
	}
}
