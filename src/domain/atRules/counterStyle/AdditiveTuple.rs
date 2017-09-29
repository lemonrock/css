// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// <integer> && <symbol>
#[derive(Clone, Debug, ToCss)]
pub struct AdditiveTuple
{
	/// <integer>
	pub weight: u32,
	
	/// <symbol>
	pub symbol: Symbol,
}

impl OneOrMoreSeparated for AdditiveTuple
{
	type S = Comma;
}

impl Parse for AdditiveTuple
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		let symbol = input.try(|input| Symbol::parse(context, input));
		let weight = input.expect_integer()?;
		if weight < 0
		{
			return Err(StyleParseError::UnspecifiedError.into())
		}
		let symbol = symbol.or_else(|_| Symbol::parse(context, input))?;
		Ok
		(
			Self
			{
				weight: weight as u32,
				symbol,
			}
		)
	}
}
