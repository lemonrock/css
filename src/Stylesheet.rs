// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct Stylesheet
{
	pub rules: CssRules,
	pub source_map_url: Option<String>,
	pub source_url: Option<String>,
}

impl Stylesheet
{
	// An solution is to use the HTTP header SourceMap: <url>
	pub fn to_css<W: fmt::Write>(&self, destination: &mut W, include_source_urls: bool) -> fmt::Result
	{
		if include_source_urls
		{
			// An older convention was to use '@' instead of '#'
			
			if let Some(ref source_map_url) = self.source_map_url
			{
				write!(destination, "//# sourceMappingURL=<{}>\n", source_map_url)?;
			}
			
			if let Some(ref source_url) = self.source_url
			{
				write!(destination, "//# sourceURL=<{}>\n", source_url)?;
			}
		}
		
		self.rules.to_css(destination)?;
		
		Ok(())
	}
	
	pub fn parse<'i>(css: &'i str) -> Result<Self, PreciseParseError<CustomParseError<'i>>>
	{
		const LineNumberingIsZeroBased: u32 = 0;
		
		let mut parserInput = ParserInput::new_with_line_number_offset(css, LineNumberingIsZeroBased);
		let mut input = Parser::new(&mut parserInput);
		
		let mut rules = Vec::new();
		
		let topLevelRuleParser = TopLevelRuleParser
		{
			context: ParserContext
			{
				rule_type: None,
				parsing_mode: ParsingMode::Default,
			},
			state: State::Start,
			namespaces: Namespaces::empty(),
		};
		
		{
			let mut iter = RuleListParser::new_for_stylesheet(&mut input, topLevelRuleParser);
			
			while let Some(result) = iter.next()
			{
				match result
				{
					Ok(rule) => rules.push(rule),
					Err(preciseParseError) => return Err(preciseParseError),
				}
			}
		}
		
		Ok
		(
			Self
			{
				rules: CssRules(rules),
				source_map_url: input.current_source_map_url().map(String::from),
				source_url: input.current_source_url().map(String::from),
			}
		)
	}
	
}
