// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Either a `<length>`, a `<percentage>`, or the `auto` keyword.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum LengthOrPercentageOrAuto
{
	Length(NoCalcLength),
	
	Percentage(Percentage),
	
	Auto,
	
	Calc(Box<CalcLengthOrPercentage>),
}

impl ToCss for LengthOrPercentageOrAuto
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::LengthOrPercentageOrAuto::*;
		
		match *self
		{
			Length(ref length) => length.to_css(dest),
			
			Percentage(ref percentage) => percentage.to_css(dest),
			
			Auto => dest.write_str("auto"),
			
			Calc(ref calc) => calc.to_css(dest)
		}
	}
}

impl From<NoCalcLength> for LengthOrPercentageOrAuto
{
	#[inline]
	fn from(len: NoCalcLength) -> Self
	{
		LengthOrPercentageOrAuto::Length(len)
	}
}

impl From<::domain::Percentage> for LengthOrPercentageOrAuto
{
	#[inline]
	fn from(percentage: ::domain::Percentage) -> Self
	{
		LengthOrPercentageOrAuto::Percentage(percentage)
	}
}

impl Parse for LengthOrPercentageOrAuto
{
	#[inline]
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_all(context, input)
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
		LengthOrPercentageOrAuto::Percentage(domain::Percentage::zero())
	}
	
	#[inline]
	pub(crate) fn parse_all<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_internal(context, input, AllowedNumericType::All)
	}
	
	#[inline]
	pub(crate) fn parse_non_negative<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_internal(context, input, AllowedNumericType::NonNegative)
	}
	
	fn parse_internal<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, num_context: AllowedNumericType) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
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
				
				Token::Percentage { unit_value, .. } if num_context.is_ok(context.parsing_mode, unit_value) => return Ok(Percentage(domain::Percentage(unit_value))),
				
				// A little lenient
				Token::Number { value, .. } if num_context.is_ok(context.parsing_mode, value) => Ok(Length(Absolute(Px(value)))),
				
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
