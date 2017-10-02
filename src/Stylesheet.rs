// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug)]
pub struct Stylesheet(pub Vec<CssRule>);

impl Stylesheet
{
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
				parsing_mode: ParsingMode::PARSING_MODE_DEFAULT,
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
		
		Ok(Stylesheet(rules))
	}
	
}
