// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


// NOTE: impl<'a, 'b, 'i> DeclarationParser<'i> for CounterStyleRuleParser<'a, 'b> is in the macro definition 'counter_style_descriptors' in 'CounterStyleAtRule.rs'
struct CounterStyleRuleParser<'a, 'b: 'a>
{
	context: &'a ParserContext<'b>,
	rule: &'a mutCounterStyleAtRule,
}

/// Default methods reject all at rules.
impl<'a, 'b, 'i> AtRuleParser<'i> for CounterStyleRuleParser<'a, 'b>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = ();
	
	type Error = SelectorParseError<'i, StyleParseError<'i>>;
}
