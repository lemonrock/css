// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


// XXXManishearth Replace all specified value parse impls with impls of this trait.
// This will make it easy to write more generic values in the future.
/// A trait to abstract parsing of a specified value given a `ParserContext` and CSS input.
pub trait Parse: Sized
{
	/// Parse a value of this type.
	///
	/// Returns an error on failure.
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>;
}

impl<T> Parse for Vec<T> where T: Parse + OneOrMoreSeparated, <T as OneOrMoreSeparated>::S: Separator,
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		<T as OneOrMoreSeparated>::S::parse(input, |i| T::parse(context, i))
	}
}

impl Parse for ::cssparser::UnicodeRange
{
	fn parse<'i, 't>(_context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		UnicodeRange::parse(input).map_err(|e| e.into())
	}
}
