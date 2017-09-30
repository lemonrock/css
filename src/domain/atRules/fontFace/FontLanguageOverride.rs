// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub enum FontLanguageOverride
{
	normal,
	Override(OpenTypeLanguageTag),
}

impl ToCss for FontLanguageOverride
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::FontLanguageOverride::*;;
		
		match *self
		{
			normal => serialize_identifier("normal", dest),
			Override(openTypeLanguageTag) => openTypeLanguageTag.to_css(dest),
		}
	}
}

impl FontLanguageOverride
{
	/// Parse a font-family value
	pub(crate) fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::FontLanguageOverride::*;;
		
		if let Ok(value) = input.try(|i| i.expect_string())
		{
			return Ok(Override(OpenTypeLanguageTag::parse(value)?))
		}
		
		let identifier = input.expect_ident()?.clone();
		match_ignore_ascii_case!
		{
			&identifier,
			
			"normal" => Ok(normal),
			
			_ => Err(ParseError::Custom(CustomParseError::InvalidFontLanguageOverrideIdentifier(identifier.to_owned()))),
		}
	}
}
