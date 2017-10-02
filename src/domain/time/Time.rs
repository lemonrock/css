// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// A time value according to CSS-VALUES § 6.2.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time
{
	seconds: CssFloat,
	unit: TimeUnit,
}

impl Parse for Time
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_with_clamping_mode(context, input, AllowedNumericType::All)
	}
}

impl ToCss for Time
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match self.unit
		{
			Second =>
			{
				self.seconds.to_css(dest)?;
				dest.write_str("s")?;
			}
			
			Millisecond =>
			{
				(self.seconds * 1000.).to_css(dest)?;
				dest.write_str("ms")?;
			}
		}
		
		Ok(())
	}
}

impl Time
{
	/// Returns a time value that represents `seconds` seconds.
	pub fn from_seconds(seconds: CssFloat) -> Self
	{
		Self
		{
			seconds,
			unit: Second,
		}
	}
	
	/// Returns a `Time` value from a CSS `calc()` expression.
	pub(crate) fn from_calc(seconds: CssFloat) -> Self
	{
		Self
		{
			seconds,
			unit: Second,
		}
	}
	
	/// Returns `0s`.
	pub fn zero() -> Self
	{
		Self::from_seconds(0.0)
	}
	
	/// Returns the time in fractional seconds.
	pub fn seconds(self) -> CssFloat
	{
		self.seconds
	}
	
	/// Parses a time according to CSS-VALUES § 6.2.
	#[inline(always)]
	pub(crate) fn parse_dimension(value: CssFloat, unit: &str, _was_calc: bool) -> Result<Self, ()>
	{
		let (seconds, unit) = match_ignore_ascii_case!
		{
			unit,
            
            "s" => (value, Second),
            
            "ms" => (value / 1000.0, Millisecond),
            
            _ => return Err(())
        };
		
		Ok
		(
			Self
			{
				seconds,
				unit,
			}
		)
	}
	
	/// Parses a non-negative time value.
	#[inline(always)]
	pub(crate) fn parse_non_negative<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse_with_clamping_mode(context, input, AllowedNumericType::NonNegative)
	}
	
	fn parse_with_clamping_mode<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, clamping_mode: AllowedNumericType) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::Token::*;
		
		match input.next()
		{
			// Note that we generally pass ParserContext to is_ok() to check that the ParserMode of the ParserContext allows all numeric values for SMIL regardless of clamping_mode, but in this Time value case, the value does not animate for SMIL at all, so we use PARSING_MODE_DEFAULT directly.
			Ok(&Dimension { value, ref unit, .. }) if clamping_mode.is_ok(ParsingMode::PARSING_MODE_DEFAULT, value) =>
			{
				const WasNotFromCalc: bool = false;
				
				return Self::parse_dimension(value, unit, WasNotFromCalc).map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseTimeDimension))
			}
			
			Ok(&Function(ref name)) if name.eq_ignore_ascii_case("calc") =>
			{
			}
			
			Ok(unexpectedToken) => return Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
			
			Err(error) => return Err(error.into())
		}
		
		match input.parse_nested_block(|input| CalcNode::parse_time(context, input))
		{
			Ok(time) if clamping_mode.is_ok(ParsingMode::PARSING_MODE_DEFAULT, time.seconds) => Ok(time),
			_ => Err(ParseError::Custom(CustomParseError::CouldNotParseTimeDimensionInCalcFunction)),
		}
	}
}
