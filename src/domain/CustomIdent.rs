// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/css-values-4/#custom-idents
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CustomIdent(pub Atom);

impl CustomIdent
{
	/// Parse an already-tokenizer identifier
	pub(crate) fn from_ident<'i>(ident: &CowRcStr<'i>, excluding: &[&str]) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		match_ignore_ascii_case!
		{
			ident,
            "initial" | "inherit" | "unset" | "default" => return Err(ParseError::Custom(CustomParseError::UnexpectedCustomIdent(ident.clone()).into())),
            _ =>
            {
            }
        };
		
		let identAsString = ident.clone().into();
		
		if excluding.iter().any(|s| ident.eq_ignore_ascii_case(s))
		{
			Err(ParseError::Custom(CustomParseError::CustomIdentWasExcluded(identAsString)))
		}
		else
		{
			Ok(CustomIdent(Atom(identAsString)))
		}
	}
}

impl ToCss for CustomIdent
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.0.to_css(dest)
	}
}
