// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Either a `<length>`, a `<percentage>`, or the `auto` keyword.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, ToCss)]
pub enum LengthOrPercentageOrAuto
{
	Length(NoCalcLength),
	
	Percentage(::css::domain::Percentage),
	
	Auto,
	
	/*
	
		There are two implementations of CalcLengthOrPercentage:-
			components/style/values/specified/calc.rs
			components/style/values/computed/length.rs
		
		CalcNode resides in components/style/values/specified/calc.rs
	
	
	*/
	Calc(Box<CalcLengthOrPercentage>),
}

impl From<NoCalcLength> for LengthOrPercentageOrAuto
{
	#[inline]
	fn from(len: NoCalcLength) -> Self
	{
		LengthOrPercentageOrAuto::Length(len)
	}
}

impl From<computed::Percentage> for LengthOrPercentageOrAuto
{
	#[inline]
	fn from(pc: ::css::domain::Percentage) -> Self
	{
		LengthOrPercentageOrAuto::Percentage(pc)
	}
}

impl Parse for LengthOrPercentageOrAuto
{
	#[inline]
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		Self::parse_quirky(context, input, AllowQuirks::No)
	}
}

impl LengthOrPercentageOrAuto
{
	/// Returns the `auto` value.
	pub fn auto() -> Self
	{
		LengthOrPercentageOrAuto::Auto
	}
	
	/// Returns a value representing a `0` length.
	pub fn zero() -> Self
	{
		LengthOrPercentageOrAuto::Length(NoCalcLength::zero())
	}
	
	/// Returns a value representing `0%`.
	pub fn zero_percent() -> Self
	{
		LengthOrPercentageOrAuto::Percentage(computed::Percentage::zero())
	}
	
	/// Parses, with quirks.
	#[inline]
	pub(crate) fn parse_quirky<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, allow_quirks: AllowQuirks) -> Result<Self, ParseError<'i>>
	{
		Self::parse_internal(context, input, AllowedNumericType::All, allow_quirks)
	}
	
	/// Parse a non-negative length, percentage, or auto.
	#[inline]
	pub(crate) fn parse_non_negative<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		Self::parse_non_negative_quirky(context, input, AllowQuirks::No)
	}
	
	/// Parse a non-negative length, percentage, or auto.
	#[inline]
	pub fn parse_non_negative_quirky<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, allow_quirks: AllowQuirks) -> Result<Self, ParseError<'i>>
	{
		Self::parse_internal(context, input, AllowedNumericType::NonNegative, allow_quirks)
	}
	
	fn parse_internal<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, num_context: AllowedNumericType, allow_quirks: AllowQuirks) -> Result<Self, ParseError<'i>>
	{
		use self::LengthOrPercentageOrAuto::*;
		use self::NoCalcLength::Absolute;
		use self::AbsoluteLength::Px;
		
		{
			let token = input.next()?;
			match *token
			{
				Token::Dimension { value, ref unit, .. } if num_context.is_ok(context.parsing_mode, value) =>
				{
					return NoCalcLength::parse_dimension(context, value, unit)
					.map(Length)
					.map_err(|()| BasicParseError::UnexpectedToken(token.clone()).into())
				}
				
				Token::Percentage { unit_value, .. } if num_context.is_ok(context.parsing_mode, unit_value) => return Ok(Percentage(computed::Percentage(unit_value))),
				
				Token::Number { value, .. } if num_context.is_ok(context.parsing_mode, value) =>
				{
					if value != 0. && !context.parsing_mode.allows_unitless_lengths() && !allow_quirks.allowed(context.quirks_mode)
					{
						return Err(StyleParseError::UnspecifiedError.into())
					}
					return Ok(Length(Absolute(Px(value))))
				}
				
				Token::Ident(ref value) if value.eq_ignore_ascii_case("auto") => return Ok(Auto),
				
				Token::Function(ref name) if name.eq_ignore_ascii_case("calc") =>
				{
				}
				
				_ => return Err(BasicParseError::UnexpectedToken(token.clone()).into())
			}
		}
		
		let calc = input.parse_nested_block(|i| CalcNode::parse_length_or_percentage(context, i, num_context))?;
		Ok(Calc(Box::new(calc)))
	}
}
