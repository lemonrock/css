// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/css-counter-styles/#typedef-symbol
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Symbol
{
	/// <string>
	String(String),
	
	/// <ident>
	Ident(String),
	
	// Not implemented:
	// /// <image>
	// Image(Image),
}

impl Parse for Symbol
{
	fn parse<'i, 't>(_context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		use self::Symbol::*;
		
		match input.next()
		{
			Ok(&Token::QuotedString(ref s)) => Ok(String(s.as_ref().to_owned())),
			
			Ok(&Token::Ident(ref s)) => Ok(Ident(s.as_ref().to_owned())),
			
			Ok(token) => Err(BasicParseError::UnexpectedToken(token.clone()).into()),
			
			Err(e) => Err(e.into()),
		}
	}
}

impl ToCss for Symbol
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::Symbol::*;
		
		match *self
		{
			String(ref s) => s.to_css(dest),
			Ident(ref s) => serialize_identifier(s, dest),
		}
	}
}

impl Symbol
{
	/// Returns whether this symbol is allowed in symbols() function.
	pub fn is_allowed_in_symbols(&self) -> bool
	{
		use self::Symbol::*;
		
		match self
		{
			// Identifier is not allowed.
			&Ident(_) => false,
			
			_ => true,
		}
	}
}
