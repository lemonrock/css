// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Type used as the associated type in the `Separated` trait on a type to indicate that a serialized list of elements of this type is separated by commas, but spaces without commas are also allowed when parsing.
pub(crate) struct CommaWithSpace;

impl Separator for CommaWithSpace
{
	fn separator() -> &'static str
	{
		", "
	}
	
	fn parse<'i, 't, F, T, E>(input: &mut Parser<'i, 't>, mut parse_one: F) -> Result<Vec<T>, ParseError<'i, E>> where F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>
	{
		skip_whitespace_even_though_it_is_unnecessary_for_correctness_as_it_may_help_try_rewind_less(input);
		
		let mut results = vec![parse_one(input)?];
		loop
		{
			skip_whitespace_even_though_it_is_unnecessary_for_correctness_as_it_may_help_try_rewind_less(input);
			
			let comma = input.try(|i| i.expect_comma()).is_ok();
			
			skip_whitespace_even_though_it_is_unnecessary_for_correctness_as_it_may_help_try_rewind_less(input);
			if let Ok(item) = input.try(&mut parse_one)
			{
				results.push(item);
			}
			else if comma
			{
				return Err(BasicParseError::UnexpectedToken(Token::Comma).into());
			}
			else
			{
				break;
			}
		}
		Ok(results)
	}
}
