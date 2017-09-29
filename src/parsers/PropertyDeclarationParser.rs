// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A struct to parse property declarations.
struct PropertyDeclarationParser<'a, 'b: 'a>
{
	context: &'a ParserContext<'b>,
	isImportantDisallowed: bool,
}

/// In theory, @rules may be present. In practice, none are currently defined (Sep 2017)
impl<'a, 'b, 'i> AtRuleParser<'i> for PropertyDeclarationParser<'a, 'b>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = Importance;
	
	type Error = SelectorParseError<'i, StyleParseError<'i>>;
}

impl<'a, 'b, 'i> DeclarationParser<'i> for PropertyDeclarationParser<'a, 'b>
{
	type Declaration = PropertyDeclaration;
	
	type Error = CustomParseError;
	
	fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Declaration, ParseError<'i, Self::Error>>
	{
		let sourceLocation = input.SourceLocation();
		
		let name = name.to_ascii_lowercase();
		
		let value = input.parse_until_before(Delimiter::Bang, |input|
		{
			if let Some(cssWideKeyword) = input.try(|input| CssWideKeyword::parse(input))
			{
				Ok(UnparsedPropertyValue::CssWideKeyword(cssWideKeyword))
			}
			else
			{
				SpecifiedValue::parse(self.context, input)
			}
		})?;
		
		let importance = Importance::from_bool(input.try(parse_important).is_some());
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

impl<'a, 'b> PropertyDeclarationParser<'a, 'b>
{
	/// Parse a list of property declarations and return a property declaration block.
	pub (crate) fn parse_property_declaration_list(context: &ParserContext, input: &mut Parser, isImportantDisallowed: bool) -> Result<Vec<PropertyDeclaration>, ParseError>
	{
		let mut propertyDeclarations = Vec::new();
		let mut parsedPropertyDeclarations = DeclarationListParser::new(input, PropertyDeclarationParser
		{
			context,
			isImportantDisallowed,
		});
		
		while let Some(propertyDeclaration) = parsedPropertyDeclarations.next()
		{
			propertyDeclarations.push(propertyDeclaration?);
		}
		
		Ok(propertyDeclarations)
	}
}
