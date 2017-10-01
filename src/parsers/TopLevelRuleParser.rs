// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// The parser for the top-level rules in a stylesheet.
pub(crate) struct TopLevelRuleParser
{
	/// This won't contain any namespaces, and only nested parsers created with `ParserContext::new_with_rule_type` will.
	pub(crate) context: ParserContext,
	
	/// The current state of the parser.
	pub(crate) state: State,
	
	pub(crate) namespaces: Namespaces,
}

impl<'i> AtRuleParser<'i> for TopLevelRuleParser
{
	type PreludeNoBlock = CssRule;
	
	type PreludeBlock = AtRuleBlockPrelude;
	
	type AtRule = CssRule;
	
	type Error = CustomParseError<'i>;
	
	fn parse_prelude<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<AtRuleType<Self::PreludeNoBlock, Self::PreludeBlock>, ParseError<'i, Self::Error>>
	{
		let source_location = input.current_source_location();
		
		match_ignore_ascii_case!
		{
			&*name,
			
			"import" =>
			{
				if self.state > State::Imports
				{
					self.had_hierarchy_error = true;
					return Err(ParseError::Custom(CustomParseError::AtRuleImportMustBeBeforeAnyRuleExceptAtRuleCharset));
				}
				
				Ok(CssRule::Import(ImportAtRule
				{
					url: SpecifiedUrl(input.expect_url_or_string()?.as_ref().to_owned()),
					media_list: MediaList::parse_media_query_list(&self.context, input, self.error_context.error_reporter),
					source_location,
				}))
			}
			
			"namespace" =>
			{
				if self.state > State::Namespaces
				{
					// "@namespace must be before any rule but @charset and @import"
					self.had_hierarchy_error = true;
					return Err(ParseError::Custom(CustomParseError::AtRuleNamespaceMustBeBeforeAnyRuleExceptAtRuleCharsetAndAtRuleImport));
				}

				let prefix = input.try(|i| i.expect_ident()).map(|prefix| Atom::from(prefix.to_owned())).ok();
				
				let url = match input.expect_url_or_string()
				{
					Ok(url_or_string) => NamespaceUrl(Atom::from(url_or_string)),
					
					Err(BasicParseError::UnexpectedToken(token)) => return Err(ParseError::Custom(CustomParseError::UnexpectedTokenForAtNamespaceRuleNamespaceValue(token.clone()))),
					
					Err(error) => return Err(error),
				};
				
				self.namespaces.get_mut().expect("@namespace rules are parsed before css selectors").update(prefix.as_ref(), &url);
				
				Ok(CssRule::Namespace(NamespaceAtRule
				{
					prefix,
					url,
					source_location,
				}))
			}
			
			"charset" =>
			{
				// @charset is removed by cssparser if it’s the first rule in the stylesheet; anything left is invalid.
				Err(ParseError::Custom(CustomParseError::UnexpectedCharsetAtRule))
			}
			
			_ =>
			{
				// Don't allow starting with an invalid state
				if self.state > State::Body
				{
					self.state = State::Invalid;
					return Err(ParseError::Custom(CustomParseError::InvalidParseState));
				}
				self.state = State::Body;
				
				(&mut self.nested()).parse_prelude(name, input)
			}
		}
	}
	
	#[inline]
	fn rule_without_block(&mut self, prelude: Self::AtRule) -> Self::AtRule
	{
		use self::State::*;
		use self::CssRule::*;
		
		match prelude
		{
			Import(url, media_list, source_location) =>
			{
				self.state = Imports;
				
				CssRule::Import(ImportAtRule
				{
					url,
					media_list,
					source_location,
				})
			}
			
			Namespace(prefix, url, source_location) =>
			{
				self.state = Namespaces;
				
				CssRule::Namespace(NamespaceAtRule
				{
					prefix,
					url,
					source_location,
				})
			}
		}
	}
	
	#[inline]
	fn parse_block<'t>(&mut self, prelude: Self::PreludeBlock, input: &mut Parser<'i, 't>) -> Result<Self::AtRule, ParseError<'i, Self::Error>>
	{
		let mut nested = self.nested();
		nested.parse_block(prelude, input).map(|rule| { self.state = State::Body; rule })
	}
}

impl<'i> QualifiedRuleParser<'i> for TopLevelRuleParser
{
	type Prelude = QualifiedRuleParserPrelude;
	
	type QualifiedRule = CssRule;
	
	type Error = CustomParseError<'i>;
	
	#[inline]
	fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>) -> Result<Self::Prelude, ParseError<'i, Self::Error>>
	{
		let mut nested = self.nested();
		nested.parse_prelude(input)
	}
	
	#[inline]
	fn parse_block<'t>(&mut self, prelude: Self::Prelude, input: &mut Parser<'i, 't>) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>>
	{
		let mut nested = self.nested();
		nested.parse_block(prelude, input).map(|result| { self.state = State::Body; result })
	}
}

impl TopLevelRuleParser
{
	fn nested<'a>(&'a self) -> NestedRuleParser<'a>
	{
		NestedRuleParser
		{
			context: &self.context,
			namespaces: self.namespaces.clone(),
		}
	}
	
	/// Returns the current state of the parser.
	pub(crate) fn state(&self) -> State
	{
		self.state
	}
}
