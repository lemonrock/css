// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MonochromeBitDepth
{
	pub depth: u32,
}

impl ToCss for MonochromeBitDepth
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.depth.to_css(dest)
	}
}

impl Parse for MonochromeBitDepth
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		unimplemented!();
	}
}

impl MonochromeBitDepth
{
	#[inline(always)]
	fn fromRawNumber<'i>(value: f32, int_value: Option<i32>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		let depth = if let Some(value) = int_value
		{
			if value < 0
			{
				return Err(ParseError::Custom(CustomParseError::MonochromeBitDepthMustBeZeroOrAPositiveInteger(value)));
			}
			value as u32
		}
		else
		{
			return Err(ParseError::Custom(CustomParseError::MonochromeBitDepthMustBeAnInteger(value)))
		};
		
		return Ok
		(
			Self
			{
				depth,
			}
		)
	}
}
