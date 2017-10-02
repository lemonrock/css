// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PropertyDeclarations(pub Vec<PropertyDeclaration>);

impl ToCss for PropertyDeclarations
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		for propertyDeclaration in self.0.iter()
		{
			propertyDeclaration.to_css(dest)?;
		}
		
		Ok(())
	}
}

impl PropertyDeclarations
{
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.0.is_empty()
	}
	
	// Parse a list of property declarations and return a property declaration block.
	pub (crate) fn parse_property_declaration_list<'i: 't, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, isImportantDisallowed: bool) -> Result<PropertyDeclarations, ParseError<'i, CustomParseError<'i>>>
	{
		let mut propertyDeclarations = Vec::new();
		let mut parsedPropertyDeclarations = DeclarationListParser::new(input, PropertyDeclarationParser
		{
			context,
			isImportantDisallowed,
		});
		
		while let Some(propertyDeclaration) = parsedPropertyDeclarations.next()
		{
			match propertyDeclaration
			{
				Ok(propertyDeclaration) => propertyDeclarations.push(propertyDeclaration),
				Err(preciseParseError) => return Err(preciseParseError.error),
			}
		}
		
		Ok(PropertyDeclarations(propertyDeclarations))
	}
}
