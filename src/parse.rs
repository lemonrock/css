// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub(crate) fn parse<R: ParseErrorReporter>(css: &str, error_reporter: &R) -> (Vec<CssRule>, Vec<PreciseParseError<CustomParseError>>)
{
	const line_number_offset: u32 = 0;
	let mut input = ParserInput::new_with_line_number_offset(css, line_number_offset);
	let mut input = Parser::new(&mut input);
	
	let mut rules = Vec::new();
	let mut failedRules = Vec::new();
	
	let topLevelRuleParser = TopLevelRuleParser
	{
		context: ParserContext
		{
			rule_type: None,
			parsing_mode: ParsingMode::PARSING_MODE_DEFAULT,
			quirks_mode: QuirksMode::NoQuirks,
		},
		error_context: ParserErrorContext { error_reporter },
		state: State::Start,
		had_hierarchy_error: false,
	};
	
	{
		let mut iter = RuleListParser::new_for_stylesheet(&mut input, topLevelRuleParser);
		
		while let Some(result) = iter.next()
		{
			match result
			{
				Ok(rule) => rules.push(rule),
				Err(parseError) => failedRules.push(error),
			}
		}
	}
	
	(rules, failedRules)
}
