// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/css-counter-styles/#counter-style-speak-as
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToCss)]
pub enum SpeakAs
{
	/// auto
	Auto,
	
	/// bullets
	Bullets,
	
	/// numbers
	Numbers,
	
	/// words
	Words,
	
	SpellOut,
	
	/// <counter-style-name>
	Other(CounterStyleIdent),
}

impl Parse for SpeakAs
{
	fn parse<'i, 't>(_context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		use self::SpeakAs::*;
		
		let result = input.try(|input|
		{
			let ident = input.expect_ident().map_err(|_| ())?;
			match_ignore_ascii_case!
			{
				&*ident,
                
                "auto" => Ok(Auto),
                
                "bullets" => Ok(Bullets),
                
                "numbers" => Ok(Numbers),
                
                "words" => Ok(Words),
                
                "spell-out" => Ok(SpellOut),
                
                _ => Err(()),
            }
		});
		
		result.or_else(|_|
		{
			Ok(Other(CounterStyleIdent::parse(input)?))
		})
	}
}
