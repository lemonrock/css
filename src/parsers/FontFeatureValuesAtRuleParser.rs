// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Parser for `FontFeatureValuesRule`. Parses all blocks
/// <feature-type> {
///   <feature-value-declaration-list>
/// }
/// <feature-type> = @stylistic | @historical-forms | @styleset |
/// @character-variant | @swash | @ornaments | @annotation
pub(crate) struct FontFeatureValuesAtRuleParser<'a>
{
	context: &'a ParserContext,
	rule: &'a mut FontFeatureValuesAtRule,
}

/// Default methods reject all qualified rules.
impl<'a, 'i> QualifiedRuleParser<'i> for FontFeatureValuesAtRuleParser<'a>
{
	type Prelude = ();
	
	type QualifiedRule = ();
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i> AtRuleParser<'i> for FontFeatureValuesAtRuleParser<'a>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = FontFeatureValuesBlockType;
	
	type AtRule = ();
	
	type Error = CustomParseError<'i>;
	
	fn parse_prelude<'t>(&mut self, name: CowRcStr<'i>, _input: &mut Parser<'i, 't>) -> Result<AtRuleType<Self::PreludeNoBlock, Self::PreludeBlock>, ParseError<'i, Self::Error>>
	{
		use self::AtRuleType::WithBlock;
		use self::FontFeatureValuesBlockType::*;
		
		match_ignore_ascii_case!
		{
			&*name,
			
			"swash" => Ok(WithBlock(swash)),
			
			"stylistic" => Ok(WithBlock(stylistic)),
			
			"ornaments" => Ok(WithBlock(ornaments)),
			
			"annotation" => Ok(WithBlock(annotation)),
			
			"character-variant" => Ok(WithBlock(character_variant)),
			
			"styleset" => Ok(WithBlock(styleset)),
			
			_ => Err(BasicParseError::AtRuleBodyInvalid.into()),
		}
	}
	
	fn parse_block<'t>(&mut self, prelude: Self::PreludeBlock, input: &mut Parser<'i, 't>) -> Result<Self::AtRule, ParseError<'i, CustomParseError<'i>>>
	{
		debug_assert_eq!(self.context.rule_type(), CssRuleType::FontFeatureValues);
		
		use self::FontFeatureValuesBlockType::*;
		
		match prelude
		{
			swash => self.parseBlock(input, &mut self.rule.swash),
			stylistic => self.parseBlock(input, &mut self.rule.stylistic),
			ornaments => self.parseBlock(input, &mut self.rule.ornaments),
			annotation => self.parseBlock(input, &mut self.rule.annotation),
			character_variant => self.parseBlock(input, &mut self.rule.character_variant),
			styleset => self.parseBlock(input, &mut self.rule.styleset),
		}
	}
}

impl<'a> FontFeatureValuesAtRuleParser<'a>
{
	#[inline(always)]
	fn parseBlock<'i, 't, T: 'a + ToCss>(&self, input: &mut Parser<'i, 't>, declarations: &'a mut Vec<FontFeatureValuesDeclaration<T>>) -> Result<(), ParseError<'i, CustomParseError<'i>>>
	{
		FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, declarations)
	}
}
