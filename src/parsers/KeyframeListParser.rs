// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Parses a keyframes list, like:
/// 0%, 50% {
///     width: 50%;
/// }
///
/// 40%, 60%, 100% {
///     width: 100%;
/// }
pub(crate) struct KeyframeListParser<'a>
{
	context: &'a ParserContext<'a>,
}

// Default methods reject all @ rules.
impl<'a, 'i> AtRuleParser<'i> for KeyframeListParser<'a>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = Keyframe;
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i> QualifiedRuleParser<'i> for KeyframeListParser<'a>
{
	type Prelude = KeyframeSelectorParserPrelude;
	
	type QualifiedRule = Keyframe;
	
	type Error = CustomParseError<'i>;
	
	fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>) -> Result<Self::Prelude, ParseError<'i, CustomParseError<'i>>>
	{
		let source_location = input.current_source_location();
		match KeyframeSelector::parse(input)
		{
			Ok(selector) => Ok
			(
				KeyframeSelectorParserPrelude
				{
					selector,
					source_location,
				}
			),
			
			Err(error) => Err(ParseError::Custom(CustomParseError::InvalidKeyframeRule)),
		}
	}
	
	fn parse_block<'t>(&mut self, prelude: Self::Prelude, input: &mut Parser<'i, 't>) -> Result<Self::QualifiedRule, ParseError<'i, CustomParseError<'i>>>
	{
		let context = ParserContext::new_with_rule_type(self.context, CssRuleType::Keyframe);
		
		Ok
		(
			Keyframe
			{
				selector: prelude.selector,
				property_declarations: PropertyDeclarations::parse_property_declaration_list(&context, input, true)?,
				source_location: prelude.source_location,
			}
		)
	}
}

impl<'a> KeyframeListParser<'a>
{
	/// Parses a keyframe list from CSS input.
	pub(crate) fn parse_keyframe_list<'i>(context: &ParserContext, input: &mut Parser) -> Result<Vec<Keyframe>, ParseError<'i, CustomParseError<'i>>>
	{
		let mut iter = RuleListParser::new_for_nested_rule(input, KeyframeListParser
		{
			context,
		});
		
		let mut keyframes = Vec::new();
		
		while let Some(keyframe) = iter.next()
		{
			match keyframe
			{
				Ok(keyframe) => keyframes.push(keyframe),
				
				Err(error) => return error,
			}
		}
		
		Ok(keyframes)
	}
}
