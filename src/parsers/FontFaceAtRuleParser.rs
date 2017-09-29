// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


struct FontFaceAtRuleParser<'a, 'b: 'a>
{
	context: &'a ParserContext<'b>,
	rule: &'a mut FontFaceAtRule,
}

/// Default methods reject all at rules.
impl<'a, 'b, 'i> AtRuleParser<'i> for FontFaceAtRuleParser<'a, 'b>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = ();
	
	type Error = SelectorParseError<'i, StyleParseError<'i>>;
}

impl<'a, 'b, 'i> DeclarationParser<'i> for FontFaceAtRuleParser<'a, 'b>
{
	type Declaration = ();
	
	type Error = SelectorParseError<'i, StyleParseError<'i>>;
	
	fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Declaration, ParseError<'i>>
	{
		// DeclarationParser also calls parse_entirely so we’d normally not need to, but in these cases we do because we set the value as a side effect rather than returning it.
		
		match_ignore_ascii_case!
		{
			&*name,
			
			"font-family" => self.rule.family = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"src" => self.rule.sources = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"font-style" => self.rule.style = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"font-weight" => self.rule.weight = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"font-stretch" => self.rule.stretch = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"font-display" => self.rule.display = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"unicode-range" => self.rule.unicode_range = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"font-feature-settings" => self.rule.feature_settings = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			"font-language-override" => self.rule.language_override = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),
			
			_ => return Err(SelectorParseError::UnexpectedIdent(name.clone()).into())
		}
		
		Ok(())
	}
}
