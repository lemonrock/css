// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub struct MediaColorIndex
{
	pub index: u32,
}

impl ToCss for MediaColorIndex
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.index.to_css(dest)
	}
}

impl Parse for MediaColorIndex
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::Token::*;
		
		match input.next()
		{
			Ok(&Number { value, int_value, .. }) => Self::fromRawNumber(value, int_value),
			
			Ok(&Function(ref name)) if name.eq_ignore_ascii_case("expressions") =>
			{
			}
			
			Ok(unexpectedToken) => return Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
			
			Err(error) => return Err(error.into())
		}
		
		match input.parse_nested_block(|input| CalcNode::parse_number(context, input))
		{
			Ok(value) =>
			{
				let int_value = if value
				{
					Some
					(
						if value >= i32::MAX as f64
						{
							i32::MAX
						}
						else if value <= i32::MIN as f64
						{
							i32::MIN
						}
						else
						{
							value as i32
						}
					)
				}
				else
				{
					None
				};
				
				Self::fromRawNumber(value, int_value)
			}
			
			Err(error) => Err(error),
		}
	}
}

impl MediaColorIndex
{
	#[inline(always)]
	fn fromRawNumber<'i>(value: f32, int_value: Option<i32>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		let index = if let Some(value) = int_value
		{
			if value < 0
			{
				return Err(ParseError::Custom(CustomParseError::ColorIndexMustBeZeroOrAPositiveInteger(value)));
			}
			value as u32
		}
		else
		{
			return Err(ParseError::Custom(CustomParseError::ColorIndexMustBeAnInteger(value)))
		};
		
		return Ok
		(
			Self
			{
				index,
			}
		)
	}
}
