// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// The data that the parser needs from outside in order to parse a stylesheet.
pub(crate) struct ParserContext
{
	/// The current rule type, if any.
	pub(crate) rule_type: Option<CssRuleType>,
	
	/// The mode to use when parsing.
	pub(crate) parsing_mode: ParsingMode,
}

impl ParserContext
{
	/// Create a parser context based on a previous context, but with a modified rule type.
	pub(crate) fn new_with_rule_type(context: &ParserContext, rule_type: CssRuleType) -> ParserContext
	{
		Self
		{
			rule_type: Some(rule_type),
			parsing_mode: context.parsing_mode,
		}
	}
	
	/// Get the rule type, which assumes that one is available.
	pub(crate) fn rule_type(&self) -> CssRuleType
	{
		self.rule_type.expect("Rule type expected, but none was found.")
	}
	
	pub(crate) fn parsingModeDoesNotAllowUnitLessLengths(&self) -> bool
	{
		!self.parsing_mode.allows_unitless_lengths()
	}
	
	pub(crate) fn isInPageRule(&self)
	{
		context.rule_type.map_or(false, |rule_type| rule_type == CssRuleType::Page)
	}
}
