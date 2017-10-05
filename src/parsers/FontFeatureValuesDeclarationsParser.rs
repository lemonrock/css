// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// @font-feature-values inside block parser. Parses a list of `FontFeatureValuesDeclaration`.
/// (`<ident>: <integer>+`)
struct FontFeatureValuesDeclarationsParser<'a, T: 'a + Parse + ToCss>
{
	context: &'a ParserContext,
	declarations: &'a mut Vec<FontFeatureValuesDeclaration<T>>,
}

/// Default methods reject all at rules.
impl<'a, 'i, T: 'a + Parse + ToCss> AtRuleParser<'i> for FontFeatureValuesDeclarationsParser<'a, T>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = ();
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i, T: 'a + Parse + ToCss> DeclarationParser<'i> for FontFeatureValuesDeclarationsParser<'a, T>
{
	type Declaration = ();
	
	type Error = CustomParseError<'i>;
	
	fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, CustomParseError<'i>>>
	{
		let fontFeatureValuesDeclaration = FontFeatureValuesDeclaration
		{
			name: Atom::from(&*name),
			value: input.parse_entirely(|i| T::parse(self.context, i))?,
		};
		
		self.update_or_push(fontFeatureValuesDeclaration);
		
		Ok(())
	}
}

impl<'a, 'b: 'a, T: 'a + Parse + ToCss> FontFeatureValuesDeclarationsParser<'a, T>
{
	pub(crate) fn parseBlock<'i, 't>(input: &mut Parser<'i, 't>, context: &'a ParserContext, declarations: &'a mut Vec<FontFeatureValuesDeclaration<T>>) -> Result<(), ParseError<'i, CustomParseError<'i>>>
	{
		let parser = Self
		{
			context,
			declarations,
		};
		
		let mut iter = DeclarationListParser::new(input, parser);
		while let Some(possiblePreciseParseError) = iter.next()
		{
			if possiblePreciseParseError.is_err()
			{
				return Err(possiblePreciseParseError.unwrap_err().error);
			}
		}
		
		Ok(())
	}
	
	/// Updates with new value if same `ident` exists, otherwise pushes to the vector.
	fn update_or_push(&mut self, newDeclaration: FontFeatureValuesDeclaration<T>)
	{
		let declarations = &mut self.declarations;
		
		let position = declarations.iter().position(|ref thisDeclaration| thisDeclaration.name == newDeclaration.name);
		
		if let Some(index) = position
		{
			declarations[index].value = newDeclaration.value;
		}
		else
		{
			declarations.push(newDeclaration);
		}
	}
}
