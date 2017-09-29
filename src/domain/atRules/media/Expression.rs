// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A single expression as per http://dev.w3.org/csswg/mediaqueries-3/#media1
#[derive(Clone, Debug, PartialEq)]
pub struct Expression(pub ExpressionKind);

impl Expression
{
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		use self::ExpressionKind::*;
		use self::Range::*;
		
		input.expect_parenthesis_block()?;
		input.parse_nested_block(|input|
		{
			let name = input.expect_ident_cloned()?;
			input.expect_colon()?;
			
			Ok
			(
				Expression
				(
					match_ignore_ascii_case!
					{
						&name,
						
						"min-width" => Width(AtLeast(Length::parse_non_negative(context, input)?)),
						
						"max-width" => Width(AtMost(Length::parse_non_negative(context, input)?)),
						
						"width" => Width(Exact(Length::parse_non_negative(context, input)?)),
						
						_ => return Err(SelectorParseError::UnexpectedIdent(name.clone()).into())
					}
				)
			)
		})
	}
	
	/// Evaluate this expression and return whether it matches the current device.
	pub fn matches<D: Device>(&self, device: &D, quirks_mode: QuirksMode) -> bool
	{
		use self::ExpressionKind::*;
		
		match self.0
		{
			Width(ref range) => device.viewportWidthInAppUnitsMatches(range)
		}
	}
}

impl ToCss for Expression
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::ExpressionKind::*;
		use self::Range::*;
		
		let (start, length) = match self.0
		{
			Width(AtLeast(ref length)) => ("(min-width: ", length),
			Width(AtMost(ref length)) => ("(max-width: ", length),
			Width(Exact(ref length)) => ("(width: ", length),
		};
		dest.write_str(start)?;
		length.to_css(dest)?;
		dest.write_char(')')
	}
}
