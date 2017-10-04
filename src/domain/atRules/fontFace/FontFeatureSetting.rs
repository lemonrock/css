// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct FontFeatureSetting(String, u32);

impl FontFeatureSetting
{
	fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		let openTypeFeatureTag = input.expect_string()?;
		if openTypeFeatureTag.len() != 4
		{
			return Err(ParseError::Custom(CustomParseError::FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(openTypeFeatureTag.clone())))
		}
		
		for character in openTypeFeatureTag.chars()
		{
			if character <= '\x20' || character > '\x7E'
			{
				return Err(ParseError::Custom(CustomParseError::FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(openTypeFeatureTag.clone())))
			}
		}
		
		let integer = input.try(|input| input.expect_integer());
		if let Ok(integer) = integer
		{
			if integer < 0
			{
				Err(ParseError::Custom(CustomParseError::FontFeatureSettingIntegerMustBePositive(integer)))
			}
			else
			{
				Ok(FontFeatureSetting(openTypeFeatureTag.as_ref().into(), integer as u32))
			}
		}
		else
		{
			let ident = input.expect_ident()?;
			
			match_ignore_ascii_case!
			{
				&ident,
				
				"on" => Ok(FontFeatureSetting(openTypeFeatureTag.as_ref().into(), 1)),
				
				"off" => Ok(FontFeatureSetting(openTypeFeatureTag.as_ref().into(), 0)),
				
				_ => Err(ParseError::Custom(CustomParseError::FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(ident.clone())))
			}
		}
	}
}
