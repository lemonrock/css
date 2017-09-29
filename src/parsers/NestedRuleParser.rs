// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Clone)]
struct NestedRuleParser<'a, 'b: 'a, R: 'b>
{
	context: &'a ParserContext<'b>,
	error_context: &'a ParserErrorContext<'b, R>,
}

impl<'a, 'b, 'i, R: ParseErrorReporter> AtRuleParser<'i> for NestedRuleParser<'a, 'b, R>
{
	type PreludeNoBlock = AtRuleNonBlockPrelude;
	
	type PreludeBlock = AtRuleBlockPrelude;
	
	type AtRule = CssRule;
	
	type Error = SelectorParseError<'i, StyleParseError<'i>>;
	
	fn parse_prelude<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<AtRuleType<AtRuleNonBlockPrelude, AtRuleBlockPrelude>, ParseError<'i, Self::Error>>
	{
		let location = input.current_source_location();
		
		use self::AtRuleType::WithBlock;
		use self::AtRuleBlockPrelude::*;
		
		match_ignore_ascii_case!
		{
			&*name,
			
			"counter-style" => Ok(WithBlock(CounterStyle(CounterStyleIdent::parseForCounterStyleAtRule(input)?))),
			
			"document" | "-moz-document" =>
			{
				let prefix = if ::str::starts_with_ignore_ascii_case(&*name, "-moz-")
                {
                    Some(VendorPrefix::moz)
                }
                else
                {
                    None
				};
				
				Ok(WithBlock(Document(prefix, DocumentCondition::parse(self.context, input)?, location))),
			}
			
			"font-face" => Ok(WithBlock(FontFace(location))),
			
			"font-feature-values" => Ok(WithBlock(FontFeatureValues(parse_family_name_list(self.context, input)?, location))),
			
			"keyframes" | "-webkit-keyframes" | "-moz-keyframes" =>
			{
				let vendor_prefix = if ::str::starts_with_ignore_ascii_case(&*name, "-webkit-")
				{
                    Some(VendorPrefix::webkit)
                }
                else if ::str::starts_with_ignore_ascii_case(&*name, "-moz-")
                {
                    Some(VendorPrefix::moz)
                }
                else
                {
                    None
				};
				
				let name = KeyframesName::parse(self.context, input)?;
				
				Ok(WithBlock(Keyframes(vendor_prefix, name, location)))
			},
			
			"media" => Ok(WithBlock(Media(MediaList::parse_media_query_list(self.context, input, self.error_context.error_reporter), location))),
			
			"page" => Ok(WithBlock(Page(location))),
			
			"supports" => Ok(WithBlock(Supports(SupportsCondition::parse(input)?, location))),
			
			"viewport" => Ok(WithBlock(Viewport)),
			
			_ => Err(StyleParseError::UnsupportedAtRule(name.clone()).into())
		}
	}
	
	fn parse_block<'t>(&mut self, prelude: AtRuleBlockPrelude, input: &mut Parser<'i, 't>) -> Result<Self::AtRule, ParseError<'i, Self::Error>>
	{
		use self::AtRuleBlockPrelude::*;
		
		let cssRule = match prelude
		{
			CounterStyle(name) => CssRule::CounterStyle(CounterStyleAtRule::parse_body(name, &CssRuleType::CounterStyle.context(self), self.error_context, input)?.into()),
			
			Document(vendor_prefix, condition, source_location) => CssRule::Document(DocumentAtRule
			{
				vendor_prefix,
				condition,
				rules: self.parse_nested_rules(input, CssRuleType::Document),
				source_location,
			}),
			
			FontFace(source_location) => CssRule::FontFace(FontFaceAtRule::parse_body(&CssRuleType::FontFace.context(self), input, source_location)?),
			
			FontFeatureValues(family_names, source_location) => CssRule::FontFeatureValues(FontFeatureValuesAtRule::parse_body(&CssRuleType::FontFeatureValues.context(self), self.error_context, input, family_names, source_location)),
			
			Keyframes(vendor_prefix, name, source_location) => CssRule::Keyframes(KeyframesAtRule
			{
				vendor_prefix,
				name,
				keyframes: KeyframeListParser::parse_keyframe_list(&CssRuleType::Keyframes.context(self), self.error_context, input),
				source_location,
			}),
			
			Media(media_queries, source_location) => CssRule::Media(MediaAtRule),
			
			Page(source_location) => CssRule::Page(PageAtRule
			{
				propertyDeclarations: PropertyDeclarationParser::parse_property_declaration_list(&CssRuleType::Page.context(self), input, false),
				source_location,
			}),
			
			Supports(condition, source_location) => CssRule::Supports(SupportsAtRule
			{
				condition,
				rules: self.parse_nested_rules(input, CssRuleType::Supports),
				source_location,
			}),
			
			Viewport => CssRule::Viewport(ViewportAtRule::parse_body(&CssRuleType::Viewport.context(self), self.error_context, input)?),
		};
		
		Ok(cssRule)
	}
}

impl<'a, 'b, 'i, R: ParseErrorReporter> QualifiedRuleParser<'i> for NestedRuleParser<'a, 'b, R>
{
	type Prelude = QualifiedRuleParserPrelude;
	
	type QualifiedRule = CssRule;
	
	type Error = SelectorParseError<'i, StyleParseError<'i>>;
	
	fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>) -> Result<QualifiedRuleParserPrelude, ParseError<'i, Self::Error>>
	{
		let selector_parser = SelectorParser;
		
		let source_location = input.current_source_location();
		let selectors = SelectorList::parse(&selector_parser, input)?;
		
		Ok
		(
			QualifiedRuleParserPrelude
			{
				selectors,
				source_location,
			}
		)
	}
	
	fn parse_block<'t>(&mut self, prelude: QualifiedRuleParserPrelude, input: &mut Parser<'i, 't>) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>>
	{
		let context = ParserContext::new_with_rule_type(self.context, CssRuleType::Style);
		
		let styleRule = StyleRule
		{
			selectors: prelude.selectors,
			propertyDeclarations: PropertyDeclarationParser::parse_property_declaration_list(&context, input, false)?,
			source_location: prelude.source_location,
		};
		
		Ok(CssRule::Style(styleRule))
	}
}

impl<'a, 'b, R: ParseErrorReporter> NestedRuleParser<'a, 'b, R>
{
	#[inline(always)]
	pub(crate) fn context_new_with_rule_type(&self, cssRuleType: CssRuleType) -> ParserContext
	{
		ParserContext::new_with_rule_type(self.context, cssRuleType)
	}
	
	fn parse_nested_rules<'i>(&mut self, input: &mut Parser, rule_type: CssRuleType) -> Result<Vec<CssRule>, PreciseParseError<'i, SelectorParseError<'i, StyleParseError<'i>>>>
	{
		let context = rule_type.context();
		
		let nested_parser = NestedRuleParser
		{
			context: &context,
			error_context: &self.error_context,
		};
		
		let mut iter = RuleListParser::new_for_nested_rule(input, nested_parser);
		let mut rules = Vec::new();
		while let Some(result) = iter.next()
		{
			match result
			{
				Ok(rule) => rules.push(rule),
				Err(error) => return Err(error),
			}
		}
		Ok(rules)
	}
}
