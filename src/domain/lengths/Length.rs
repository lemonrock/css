// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// An extension to `NoCalcLength` to parse `calc` expressions.
/// This is commonly used for the `<length>` values.
///
/// https://drafts.csswg.org/css-values/#lengths
#[derive(Clone, Debug, PartialEq, ToCss)]
pub enum Length
{
	/// The internal length type that cannot parse `calc`
	NoCalc(NoCalcLength),
	
	/// A calc expression.
	///
	/// https://drafts.csswg.org/css-values/#calc-notation
	Calc(Box<CalcLengthOrPercentage>),
}

impl From<NoCalcLength> for Length
{
	#[inline(always)]
	fn from(len: NoCalcLength) -> Self
	{
		Length::NoCalc(len)
	}
}

impl Mul<CSSFloat> for Length
{
	type Output = Length;
	
	#[inline(always)]
	fn mul(self, scalar: CSSFloat) -> Length
	{
		use self::Length::*;
		
		match self
		{
			NoCalc(inner) => NoCalc(inner * scalar),
			Calc(..) => panic!("Can't multiply Calc!"),
		}
	}
}

impl Parse for Length
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		Self::parse_quirky(context, input, AllowQuirks::No)
	}
}

impl Length
{
	/// Returns a `zero` length.
	#[inline(always)]
	pub fn zero() -> Length
	{
		Length::NoCalc(NoCalcLength::zero())
	}
	
	/// Get an absolute length from a px value.
	#[inline(always)]
	pub fn from_px(px_value: CSSFloat) -> Length
	{
		Length::NoCalc(NoCalcLength::from_px(px_value))
	}
	
	/// Extract inner length without a clone, replacing it with zero
	///
	/// Use when you need to move out of a length array without cloning
	#[inline(always)]
	pub fn take(&mut self) -> Self
	{
		replace(self, Self::zero())
	}
	
	/// Parse a given absolute or relative dimension.
	#[inline(always)]
	pub(crate) fn parse_dimension(context: &ParserContext, value: CSSFloat, unit: &str) -> Result<Length, ()>
	{
		NoCalcLength::parse_dimension(context, value, unit).map(Length::NoCalc)
	}
	
	/// Parse a non-negative length
	#[inline(always)]
	pub(crate) fn parse_non_negative<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Length, ParseError<'i>>
	{
		Self::parse_non_negative_quirky(context, input, AllowQuirks::No)
	}
	
	/// Parse a non-negative length, allowing quirks.
	#[inline(always)]
	pub(crate) fn parse_non_negative_quirky<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, allow_quirks: AllowQuirks) -> Result<Length, ParseError<'i>>
	{
		Self::parse_internal(context, input, AllowedNumericType::NonNegative, allow_quirks)
	}
	
	/// Parses a length, with quirks.
	#[inline(always)]
	pub(crate) fn parse_quirky<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, allow_quirks: AllowQuirks) -> Result<Self, ParseError<'i>>
	{
		Self::parse_internal(context, input, AllowedNumericType::All, allow_quirks)
	}
	
	#[inline(always)]
	fn parse_internal<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, num_context: AllowedNumericType, allow_quirks: AllowQuirks) -> Result<Length, ParseError<'i>>
	{
		use self::Length::*;
		use self::NoCalcLength::Absolute;
		use self::AbsoluteLength::Px;
		
		{
			let token = input.next()?;
			match *token
			{
				Token::Dimension { value, ref unit, .. } if num_context.is_ok(context.parsing_mode, value) =>
				{
					return Self::parse_dimension(context, value, unit).map_err(|()| BasicParseError::UnexpectedToken(token.clone()).into())
				}
				
				Token::Number { value, .. } if num_context.is_ok(context.parsing_mode, value) =>
				{
					if value != 0. && context.parsingModeDoesNotAllowUnitLessLengths() && !allow_quirks.allowed(context.quirks_mode)
					{
						return Err(StyleParseError::UnspecifiedError.into())
					}
					return Ok(NoCalc(Absolute(Px(value))))
				}
				
				Token::Function(ref name) if name.eq_ignore_ascii_case("calc") =>
				{
				}
				
				ref unexpectedToken => return Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into())
			}
		}
		
		input.parse_nested_block(|input| CalcNode::parse_length(context, input, num_context).map(|calc| Calc(Box::new(calc))))
	}
}
