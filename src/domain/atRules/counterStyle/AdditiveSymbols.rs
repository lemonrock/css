// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-additive-symbols
#[derive(Clone, Debug, ToCss)]
pub struct AdditiveSymbols(pub Vec<AdditiveTuple>);

impl Parse for AdditiveSymbols
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		let tuples = Vec::<AdditiveTuple>::parse(context, input)?;
		// FIXME maybe? https://github.com/w3c/csswg-drafts/issues/1220
		if tuples.windows(2).any(|window| window[0].weight <= window[1].weight)
		{
			return Err(StyleParseError::UnspecifiedError.into())
		}
		Ok(AdditiveSymbols(tuples))
	}
}
