// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Clone)]
pub(crate) struct NestedRuleParser<'a>
{
	context: &'a ParserContext,
	namespaces: Rc<Namespaces>,
}

impl<'a, 'i> AtRuleParser<'i> for NestedRuleParser<'a>
{
	type PreludeNoBlock = CssRule;
	
	type PreludeBlock = AtRuleBlockPrelude;
	
	type AtRule = CssRule;
	
	type Error = CustomParseError<'i>;
	
	fn parse_prelude<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<AtRuleType<Self::PreludeNoBlock, Self::PreludeBlock>, ParseError<'i, Self::Error>>
	{
		use self::AtRuleType::WithBlock;
		use self::AtRuleBlockPrelude::*;
		use self::VendorPrefix::*;
		
		match_ignore_ascii_case!
		{
			&name,
			
			"counter-style" => Ok(WithBlock(CounterStyle(CounterStyleIdent::parseForCounterStyleAtRule(input)?))),
			
			"document" => Ok(WithBlock(Document(None, DocumentCondition::parse(self.context, input)?))),
			
			"-moz-document" => Ok(WithBlock(Document(Some(moz), DocumentCondition::parse(self.context, input)?))),
			
			"font-face" => Ok(WithBlock(FontFace)),
			
			"font-feature-values" => Ok(WithBlock(FontFeatureValues(FamilyName::parse_family_name_list(self.context, input)?))),
			
			"keyframes" => Ok(WithBlock(Keyframes(None, KeyframesName::parse(input)?))),
			
			"-webkit-keyframes" => Ok(WithBlock(Keyframes(Some(webkit), KeyframesName::parse(input)?))),
			
			"-moz-keyframes" => Ok(WithBlock(Keyframes(Some(moz), KeyframesName::parse(input)?))),
			
			"media" => Ok(WithBlock(Media(MediaList::parse_media_query_list(self.context, input, false)?))),
			
			"page" => Ok(WithBlock(Page(PageSelectorPseudoClass::parse(input)?))),
			
			"supports" => Ok(WithBlock(Supports(SupportsCondition::parse(input)?))),
			
			"viewport" => Ok(WithBlock(Viewport(None))),
			
			"-ms-viewport" => Ok(WithBlock(Viewport(Some(ms)))),
			
			"-o-viewport" => Ok(WithBlock(Viewport(Some(o)))),
			
			_ => Err(ParseError::Custom(CustomParseError::UnsupportedAtRule(name.clone())))
		}
	}
	
	fn parse_block<'t>(&mut self, prelude: AtRuleBlockPrelude, input: &mut Parser<'i, 't>) -> Result<Self::AtRule, ParseError<'i, Self::Error>>
	{
		use self::AtRuleBlockPrelude::*;
		
		let cssRule = match prelude
		{
			CounterStyle(name) => CssRule::CounterStyle(CounterStyleAtRule::parse_body(name, &CssRuleType::CounterStyle.context(self), input)?),
			
			Document(vendor_prefix, condition) => CssRule::Document(DocumentAtRule
			{
				vendor_prefix,
				condition,
				rules: self.parse_nested_rules(input, CssRuleType::Document)?,
			}),
			
			FontFace => CssRule::FontFace(FontFaceAtRule::parse_body(&CssRuleType::FontFace.context(self), input)?),
			
			FontFeatureValues(family_names) => CssRule::FontFeatureValues(FontFeatureValuesAtRule::parse_body(&CssRuleType::FontFeatureValues.context(self), input, family_names)?),
			
			Keyframes(vendor_prefix, name) => CssRule::Keyframes(KeyframesAtRule
			{
				vendor_prefix,
				name,
				keyframes: KeyframeListParser::parse_keyframe_list(&CssRuleType::Keyframes.context(self), input)?,
			}),
			
			Media(media_queries) => CssRule::Media(MediaAtRule
			{
				media_queries,
				rules: self.parse_nested_rules(input, CssRuleType::Media)?,
			}),
			
			Page(page_selector_pseudo_class) => CssRule::Page(PageAtRule
			{
				page_selector_pseudo_class,
				property_declarations: PropertyDeclarations::parse_property_declaration_list(&CssRuleType::Page.context(self), input)?,
			}),
			
			Supports(condition) => CssRule::Supports(SupportsAtRule
			{
				condition,
				rules: self.parse_nested_rules(input, CssRuleType::Supports)?,
			}),
			
			Viewport(vendor_prefix) => CssRule::Viewport(ViewportAtRule::parse_body(vendor_prefix, &CssRuleType::Viewport.context(self), input)?),
		};
		
		Ok(cssRule)
	}
}

impl<'a, 'i> QualifiedRuleParser<'i> for NestedRuleParser<'a>
{
	type Prelude = QualifiedRuleParserPrelude;
	
	type QualifiedRule = CssRule;
	
	type Error = CustomParseError<'i>;
	
	fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>) -> Result<Self::Prelude, ParseError<'i, Self::Error>>
	{
		let ourSelectorParser = OurSelectorParser
		{
			namespaces: self.namespaces.clone(),
		};
		
		let selectors = ourSelectorParser.parse(input)?;
		
		Ok
		(
			QualifiedRuleParserPrelude
			{
				selectors,
			}
		)
	}
	
	fn parse_block<'t>(&mut self, prelude: Self::Prelude, input: &mut Parser<'i, 't>) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>>
	{
		let context = ParserContext::new_with_rule_type(self.context, CssRuleType::Style);
		
		let styleRule = StyleRule
		{
			selectors: prelude.selectors,
			property_declarations: PropertyDeclarations::parse_property_declaration_list(&context, input)?,
		};
		
		Ok(CssRule::Style(styleRule))
	}
}

impl<'a> NestedRuleParser<'a>
{
	#[inline(always)]
	pub(crate) fn context_new_with_rule_type(&self, cssRuleType: CssRuleType) -> ParserContext
	{
		ParserContext::new_with_rule_type(self.context, cssRuleType)
	}
	
	fn parse_nested_rules<'i: 't, 't>(&mut self, input: &mut Parser<'i, 't>, rule_type: CssRuleType) -> Result<CssRules, ParseError<'i, CustomParseError<'i>>>
	{
		let context = rule_type.context(self);
		
		let nested_parser = NestedRuleParser
		{
			context: &context,
			namespaces: self.namespaces.clone(),
		};
		
		let mut iter = RuleListParser::new_for_nested_rule(input, nested_parser);
		let mut rules = Vec::new();
		while let Some(result) = iter.next()
		{
			match result
			{
				Ok(rule) => rules.push(rule),
				Err(preciseParseError) => return Err(preciseParseError.error),
			}
		}
		Ok(CssRules(rules))
	}
}
