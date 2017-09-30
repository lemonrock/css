// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// An extension to `NoCalcLength` to parse `calc` expressions.
/// This is commonly used for the `<length>` values.
///
/// https://drafts.csswg.org/css-values/#lengths
#[derive(Clone, Debug, PartialEq)]
pub enum Length
{
	/// The internal length type that cannot parse `calc`
	NoCalc(NoCalcLength),
	
	/// A calc expression.
	///
	/// https://drafts.csswg.org/css-values/#calc-notation
	Calc(Box<CalcLengthOrPercentage>),
}

impl ToCss for Length
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::Length::*;
		
		match *self
		{
			NoCalc(ref value) => value.to_css(dest),
			Calc(ref value) => value.to_css(dest),
		}
	}
}

impl From<NoCalcLength> for Length
{
	#[inline(always)]
	fn from(len: NoCalcLength) -> Self
	{
		Length::NoCalc(len)
	}
}

impl Mul<CssFloat> for Length
{
	type Output = Length;
	
	#[inline(always)]
	fn mul(self, scalar: CssFloat) -> Length
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
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_all(context, input)
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
	pub fn from_px(px_value: CssFloat) -> Length
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
	
	#[inline(always)]
	pub(crate) fn parse_dimension(context: &ParserContext, value: CssFloat, unit: &str) -> Result<Length, ()>
	{
		NoCalcLength::parse_dimension(context, value, unit).map(Length::NoCalc)
	}
	
	#[inline(always)]
	pub(crate) fn parse_all<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Length, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_internal(context, input, AllowedNumericType::All)
	}
	
	#[inline(always)]
	pub(crate) fn parse_non_negative<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Length, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_internal(context, input, AllowedNumericType::NonNegative)
	}
	
	#[inline(always)]
	fn parse_internal<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, num_context: AllowedNumericType) -> Result<Length, ParseError<'i, CustomParseError<'i>>>
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
				
				// This is a bit lenient
				Token::Number { value, .. } if num_context.is_ok(context.parsing_mode, value) => Ok(NoCalc(Absolute(Px(value)))),
				
				Token::Function(ref name) if name.eq_ignore_ascii_case("calc") =>
				{
				}
				
				ref unexpectedToken => return Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into())
			}
		}
		
		input.parse_nested_block(|input| CalcNode::parse_length(context, input, num_context).map(|calc| Calc(Box::new(calc))))
	}
}
