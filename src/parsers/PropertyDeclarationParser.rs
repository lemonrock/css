// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A struct to parse property declarations.
pub(crate) struct PropertyDeclarationParser<'a>
{
	context: &'a ParserContext,
	isImportantDisallowed: bool,
}

/// In theory, @rules may be present. In practice, none are currently defined (Sep 2017)
impl<'a, 'i> AtRuleParser<'i> for PropertyDeclarationParser<'a>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = Importance;
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i> DeclarationParser<'i> for PropertyDeclarationParser<'a>
{
	type Declaration = PropertyDeclaration;
	
	type Error = CustomParseError<'i>;
	
	fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Declaration, ParseError<'i, Self::Error>>
	{
		let sourceLocation = input.current_source_location();
		
		let name = name.to_ascii_lowercase();
		
		let value = input.parse_until_before(Delimiter::Bang, |input|
		{
			if let Ok(cssWideKeyword) = input.try(|input| CssWideKeyword::parse(input))
			{
				Ok(UnparsedPropertyValue::CssWideKeyword(cssWideKeyword))
			}
			else
			{
				Ok(UnparsedPropertyValue::SpecifiedValue(SpecifiedValue::parse(self.context, input)?))
			}
		})?;
		
		let importance = Importance::from_bool(input.try(parse_important).is_ok());
		if importance.important() && self.isImportantDisallowed
		{
			return Err(ParseError::Custom(CustomParseError::ImportantIsNotAllowedInKeyframePropertyDeclarationValues(sourceLocation)));
		}
		
		// In case there is still unparsed text in the declaration, we should roll back.
		input.expect_exhausted()?;
		
		Ok
		(
			PropertyDeclaration
			{
				name,
				value,
				importance,
			}
		)
	}
}
