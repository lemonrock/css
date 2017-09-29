// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub struct FontFeatureSettings(pub BTreeMap<String, u32>);

impl ToCss for FontFeatureSettings
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		if self.0.is_empty()
		{
			serialize_identifier("normal")
		}
		else
		{
			for &(ref openTypeFeatureTag, ref integer) in self.0.iter()
			{
				serialize_string(openTypeFeatureTag, dest)?;
				let integer = *integer;
				if integer != 1
				{
					dest.write_str(" ")?;
					integer.to_css(dest)?;
				}
			}
			Ok(())
		}
	}
}

impl Parse for FontFeatureSettings
{
	fn parse<'i, 't>(_: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		if input.try(|input| input.expect_ident_matching("normal")).is_ok()
		{
			Ok(FontFeatureSettings(BTreeMap::new()))
		}
		else
		{
			let mut settings = BTreeMap::new();
			for setting in input.parse_comma_separated(|input| FontFeatureSetting::parse(input))?
			{
				settings.insert(setting.openTypeFeatureTag, integer)
			}
			Ok(FontFeatureSettings(settings))
		}
	}
}

impl FontFeatureSettings
{
	#[inline(always)]
	pub fn setting(&self, openTypeFeatureTag: &str) -> Option<u32>
	{
		self.0.get(openTypeFeatureTag)
	}
	
	#[inline(always)]
	pub fn isOn(&self, openTypeFeatureTag: &str) -> Option<bool>
	{
		self.0.get(openTypeFeatureTag).map(|integer| integer == 1)
	}
	
	#[inline(always)]
	pub fn isOff(&self, openTypeFeatureTag: &str) -> Option<bool>
	{
		self.0.get(openTypeFeatureTag).map(|integer| integer == 0)
	}
	
	#[inline(always)]
	pub fn isNormal(&self) -> bool
	{
		self.0.is_none()
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct FontFeatureSetting
{
	pub openTypeFeatureTag: String,
	pub integer: u32, // default is 1; can be omitted
}

impl FeatureSetting
{
	fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		let openTypeFeatureTag = input.expect_string()?.to_owned();
		if openTypeFeatureTag.len() != 4
		{
			return Err(ParseError::Custom(CustomParseError::FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(openTypeFeatureTag)))
		}
		
		for character in openTypeFeatureTag.chars()
		{
			if character <= 0x20 || character > 0x7E
			{
				return Err(ParseError::Custom(CustomParseError::FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(openTypeFeatureTag)))
			}
		}
		
		let integer = input.try(|input| input.expect_integer());
		if let Some(integer) = integer
		{
			if integer < 0
			{
				Err(ParseError::Custom(CustomParseError::FontFeatureSettingIntegerMustBePositive(integer)))
			}
			else
			{
				Ok(FeatureSetting(openTypeFeatureTag, integer))
			}
		}
		else
		{
			let ident = input.expect_ident()?;
			
			match_ignore_ascii_case!
			{
				&ident,
				
				"on" => Ok(FeatureSetting(openTypeFeatureTag, 1)),
				
				"off" => Ok(FeatureSetting(openTypeFeatureTag, 0)),
				
				_ => Err(ParseError::Custom(CustomParseError::FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(ident.to_owned())))
			}
		}
	}
	
	#[inline(always)]
	pub fn isOff(&self) -> bool
	{
		self.integer == 0
	}
	
	#[inline(always)]
	pub fn isOn(&self) -> bool
	{
		self.integer == 1
	}
}
