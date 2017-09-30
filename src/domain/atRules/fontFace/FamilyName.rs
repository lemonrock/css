// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FamilyName
{
	pub name: Atom,
	pub syntax: FamilyNameSyntax,
}

impl ToCss for FamilyName
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::FamilyNameSyntax::*;
		
		match self.syntax
		{
			Quoted => serialize_string(self.name.as_ref()),
			
			// Note that `serialization` is already escaped or serialized appropriately.
			Identifiers(ref serialization) => dest.write_str(&*serialization),
		}
	}
}

// `FamilyName::parse` is based on `FontFamily::parse` and not the other way around because we want to exclude generic family keywords.
impl Parse for FamilyName
{
	fn parse<'i, 't>(_: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::FontFamily::*;
		
		match FontFamily::parse(input)
		{
			Ok(FamilyName(name)) => Ok(name),
			Ok(Generic(_)) => Err(ParseError::Custom(CustomParseError::FontFaceAtRuleFontFamilyCanNotBeGeneric)),
			Err(error) => Err(error)
		}
	}
}

impl FamilyName
{
	/// Parses a list of `FamilyName`s.
	pub(crate) fn parse_family_name_list<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Vec<Self>, ParseError<'i, CustomParseError<'i>>>
	{
		input.parse_comma_separated(|i| Self::parse(context, i)).map_err(|e| e.into())
	}
}
